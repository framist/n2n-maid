//! 通道打扫工作管理模块（N2N edge 进程管家）
//!
//! ## 设计说明：双轨信息源（stdio + UDP Management API）
//! 恩兔酱会同时“听工作汇报”（stdio：stdout/stderr）和“敲管理口门铃”（UDP Management API，JSON 包）。
//! 这样做的目标是：在 **不同 edge 版本/不同平台** 下也尽量稳定地判断状态，同时保留足够细的错误线索。
//!
//! ### 1) 依赖 stdio 的内容（更细、更贴近现场）
//! - **错误/提示识别**：`extract_user_facing_notice()` 仍主要从 stdout/stderr 文本中提取（例如 TAP busy、MAC/IP 未释放等）。
//! - **网卡信息**：`NetworkInfo`（IP/Mask/MAC）来自 stdout 的 `created local tap device ...` 行解析。
//! - **日志面板**：所有 stdout/stderr 都会原样进入“工作汇报”。
//! - **兼容性兜底**：部分版本会输出 `edge <<<` 等标志；该逻辑保留，但不再作为 UI 判定“已连接”的唯一依据。
//!
//! ### 2) 依赖 UDP Management API 的内容（结构化、更稳）
//! - **连接成功判定（UI 状态优先）**：后台轮询 `timestamps`，用 `last_super/last_p2p` 的“新鲜度”推断是否已连上。
//!   - 对外体现为 `derived_status()`：即使 stdout 没出现特定关键字，也能在心跳正常时进入 `Connected(...)`。
//! - **同伴点名册**：通过 `edges` 获取同伴列表，并缓存后由 `get_peers` 提供给前端展示。
//! - **优雅断开（Gracefully exit）**：`stop()` 优先发送 `w ... stop`，失败再回退到信号/系统命令兜底。
//!
//! ### 3) 提示信息的合成策略（derived_notice）
//! - **优先**：如果 stdio 已提取到明确错误（`last_notice`），就直接提示主人。
//! - **其次**：若 stdio 没线索，则用 `timestamps` 推断“总部不可达/心跳断联”等保守提示。
//! - **最后**：再把 Management API 轮询过程中记录的错误（如 `badauth`）作为调试线索返还。
//!
//! ### 4) 认证（management password）
//! - 如果主人在 `extra_args` 中传入 `--management-password <pw>`，恩兔会自动记下并用于管理口请求。
//! - 若主人未配置密码，部分操作会按文档默认密码 `n2n` 做一次兜底尝试（避免环境差异导致“看得见但用不了”）。
use anyhow::{Context, Result};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::UdpSocket;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;
#[cfg(target_os = "windows")]
use windows_sys::Win32::Globalization::{MultiByteToWideChar, CP_ACP, CP_OEMCP};

#[cfg(unix)]
use nix::sys::signal::{kill, Signal};
#[cfg(unix)]
use nix::unistd::Pid;

use crate::config::N2NConfig;

/// Management API stop 操作超时（毫秒）
const MGMT_STOP_TIMEOUT_MS: u64 = 10000;
/// Management API 查询超时 - socket 读取超时（毫秒）
const MGMT_READ_TIMEOUT_MS: u64 = 200;
/// Management API 查询超时 - 总等待时间（毫秒）
const MGMT_DEADLINE_MS: u64 = 1500;
/// Management API 的门牌号（恩兔只敲本机 127.0.0.1:5644）
const MGMT_ADDR: (&str, u16) = ("127.0.0.1", 5644);
/// 判断心跳是否有效的最大时间间隔（秒）
const HEARTBEAT_MAX_INTERVAL_SECS: u64 = 15;
/// 判断心跳断联的最大时间间隔（秒）- 用于提示"总部不可达"
const HEARTBEAT_DISCONNECT_THRESHOLD_SECS: u64 = 30;
/// edge 启动后等待首次 supernode 连接的超时（秒）
const EDGE_STARTUP_WAIT_SECS: u64 = 30;

/// Windows 下创建子进程时不弹黑框（恩兔把黑框悄悄收起来）
#[cfg(target_os = "windows")]
const CREATE_NO_WINDOW: u32 = 0x08000000;

/// 通道详情信息
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NetworkInfo {
    pub ip: String,
    pub mask: String,
    pub mac: String,
}

/// 同伴节点信息（来自 Management API 的 edges 列表）
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerNodeInfo {
    /// 同伴的昵称（edge 的 -I / desc）
    pub name: Option<String>,
    /// 同伴的 VPN 地址（含 CIDR，例如 10.0.0.2/24）
    pub vpn_addr: Option<String>,
    /// 仅 IP 部分（例如 10.0.0.2）
    pub vpn_ip: Option<String>,
    /// 同伴的公网 Socket 地址（例如 1.2.3.4:56789）
    pub public_addr: Option<String>,
    /// N2N 通道模式（例如 p2p / pSp 等）
    pub mode: Option<String>,
    /// edge 最后一次“看见”该同伴的时间戳（Unix 秒）
    pub last_seen: Option<u64>,
    /// 是否为本机（有些版本会返回 local=1 的记录）
    pub is_local: Option<bool>,
    /// 最近一次 ping 的延迟（毫秒）
    pub latency_ms: Option<f64>,
    /// 最近一次 ping 的时间戳（Unix 秒）
    pub last_ping: Option<u64>,
}

/// 恩兔的工作状态
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionStatus {
    /// 待命中（已断开）
    Disconnected,
    /// 正在铺设通道（连接中）
    Connecting,
    /// 收拾工具中（断开中，优雅退出）
    Disconnecting,
    /// 通道已就绪（已连接，包含详情）
    Connected(Option<NetworkInfo>),
    /// 遇到麻烦了（错误）
    Error(String),
}

/// 恩兔的工作管理器
pub struct N2NProcess {
    /// 工作进程句柄
    child: Arc<Mutex<Option<Child>>>,
    /// 当前工作状态
    status: Arc<Mutex<ConnectionStatus>>,
    /// 最近一次“需要主人注意”的提示（不一定致命，可能只是需要等待/检查配置）
    last_notice: Arc<Mutex<Option<String>>>,
    /// 工作汇报通道
    log_tx: Option<mpsc::UnboundedSender<String>>,
    /// 自动重连配置（断线后自动重新打扫）
    auto_reconnect: Arc<Mutex<Option<N2NConfig>>>,

    /// 是否由主人主动要求停止（用于区分"正常休息"与"意外摔倒"）
    stop_requested: Arc<AtomicBool>,

    /// Management API 密码（如果主人给 edge 设了门禁，恩兔也要带钥匙）
    mgmt_password: Arc<Mutex<Option<String>>>,
    /// Management API 状态缓存（避免 get_status 每次都直接去敲 UDP 门铃）
    mgmt_state: Arc<Mutex<MgmtState>>,
    /// 是否已启动后台“管理口状态刷新”小工人（避免重复开工）
    mgmt_worker_started: Arc<AtomicBool>,
    /// 同伴节点缓存（定期从 Management API 抄写一份“点名册”）
    peer_cache: Arc<Mutex<Vec<PeerNodeInfo>>>,
    /// 同伴延迟缓存（key 是 vpn_ip）
    peer_latency: Arc<Mutex<HashMap<String, (f64, u64)>>>,
    /// 是否已启动后台“点名 + 测延迟”的小工人（避免重复开工）
    peer_worker_started: Arc<AtomicBool>,
}

impl N2NProcess {
    /// 创建新的进程管理器实例
    pub fn new() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(ConnectionStatus::Disconnected)),
            last_notice: Arc::new(Mutex::new(None)),
            log_tx: None,
            auto_reconnect: Arc::new(Mutex::new(None)),
            stop_requested: Arc::new(AtomicBool::new(false)),
            mgmt_password: Arc::new(Mutex::new(None)),
            mgmt_state: Arc::new(Mutex::new(MgmtState::default())),
            mgmt_worker_started: Arc::new(AtomicBool::new(false)),
            peer_cache: Arc::new(Mutex::new(Vec::new())),
            peer_latency: Arc::new(Mutex::new(HashMap::new())),
            peer_worker_started: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 设置日志发送通道（并第一时间递上自我介绍汇报单）
    pub fn set_log_sender(&mut self, tx: mpsc::UnboundedSender<String>) {
        self.log_tx = Some(tx);
        // 第一时间递上"上岗汇报单"，让主人一打开日志面板就知道恩兔已就位
        for line in [
            "  >>> 🐰  N2N Maid —— 恩兔酱 已就位，准备打扫！ <<<",
            "  主人您好，恩兔酱（N-Too）是您贴心的网络通道管家～",
            "  只需告诉恩兔「去哪里」「暗号」，其余的交给恩兔！",
            "  ─────────────────────────────────────────────────",
            "  © framist · MIT License",
            "  📦 https://github.com/framist/n2n-maid",
            "  ─────────────────────────────────────────────────",
        ] {
            self.send_log_line(format!("[INFO] {}", line));
        }
    }

    /// 给日志面板塞一条“工作汇报”
    fn send_log_line(&self, line: String) {
        if let Some(ref tx) = self.log_tx {
            let _ = tx.send(line);
        }
    }

    /// 给主人一个进度提示（不会改变连接状态）
    pub fn log_info(&self, msg: impl AsRef<str>) {
        self.send_log_line(format!("[INFO] {}", msg.as_ref()));
    }

    /// 启动 N2N edge 进程
    pub fn start(&self, config: &N2NConfig) -> Result<()> {
        // 检查是否已经在运行
        if self.is_running() {
            return Err(anyhow::anyhow!("N2N 进程已在运行"));
        }

        // 本次启动不是“停止流程”的一部分
        self.stop_requested.store(false, Ordering::SeqCst);

        // 更新状态为连接中
        *self.status.lock().unwrap() = ConnectionStatus::Connecting;
        // 清空上一次的“提醒便签”，避免主人看到过期信息
        *self.last_notice.lock().unwrap() = None;

        // 确定 edge 可执行文件路径
        let edge_path = config
            .edge_path
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_else(|| self.get_default_edge_path());

        // 记录实际使用的 edge 路径（方便调试）
        log::info!("恩兔要打扫通道啦～ edge 可执行文件位置：{}", edge_path);

        // Linux 下 edge 通常需要 root/capabilities（创建 TAP、切换权限等）
        // 这里优先尝试为 edge 二进制授予 capabilities，避免用 pkexec 包裹运行导致 stop() 无法精确控制 edge PID
        #[cfg(target_os = "linux")]
        let edge_path = {
            let mut edge_path = edge_path;
            if !nix::unistd::Uid::effective().is_root() {
                // 解析为绝对路径，避免 setcap/实际启动的二进制不一致
                edge_path = match resolve_edge_path_for_caps(&edge_path) {
                    Ok(p) => p,
                    Err(e) => {
                        *self.status.lock().unwrap() = ConnectionStatus::Error(e.to_string());
                        return Err(e);
                    }
                };

                // 如果用户取消授权或系统缺少依赖，直接中止连接流程，避免后续出现更难理解的 EPERM
                if let Err(e) = ensure_edge_capabilities(&edge_path) {
                    let e = anyhow::anyhow!(
                        "需要管理员授权以配置 edge 权限（KDE 下会弹出授权窗口）。详细错误：{}",
                        e
                    );
                    *self.status.lock().unwrap() = ConnectionStatus::Error(e.to_string());
                    return Err(e);
                }
            }
            edge_path
        };

        // 校验 supernode 格式（必须是 host:port）
        if !config.supernode.contains(':') {
            let e = anyhow::anyhow!("Supernode 地址格式错误，必须包含端口号（如 vpn.example.com:7777）");
            *self.status.lock().unwrap() = ConnectionStatus::Error(e.to_string());
            return Err(e);
        }

        // 构建命令参数
        // -c: 社区名称
        // -l: supernode 地址（host:port）
        //
        // 备注：`-f`（前台运行）在部分 Windows 版本的 edge 中并不存在，会触发
        // `WARNING: unknown option -f`，所以 Windows 下不再传入该参数。
        let mut args = vec![
            "-c".to_string(),
            config.community.clone(),
            "-l".to_string(),
            config.supernode.clone(),
        ];

        #[cfg(not(target_os = "windows"))]
        {
            // -f: 前台运行（不 fork 到后台，便于监控）
            args.insert(0, "-f".to_string());
        }

        // -I: edge 描述/用户名（注意：不是 -n，-n 是路由配置）
        // 需求：配置中可留空，默认使用主机名
        let node_name = if config.username.trim().is_empty() {
            get_default_node_name()
        } else {
            config.username.clone()
        };
        args.push("-I".to_string());
        args.push(node_name);

        // 添加加密密钥
        if !config.encryption_key.is_empty() {
            args.push("-k".to_string());
            args.push(config.encryption_key.clone());
        }

        // IP 地址配置
        if config.ip_mode == "dhcp" {
            args.push("-a".to_string());
            args.push("dhcp:0.0.0.0".to_string());
        } else if let Some(ref static_ip) = config.static_ip {
            args.push("-a".to_string());
            args.push(static_ip.clone());
        }

        // MTU 设置
        if let Some(mtu) = config.mtu {
            args.push("-M".to_string());
            args.push(mtu.to_string());
        }

        // TAP 设备名称
        if let Some(ref tap_device) = config.tap_device {
            args.push("-d".to_string());
            args.push(tap_device.clone());
        }

        // 额外参数
        if let Some(ref extra_args) = config.extra_args {
            let extra: Vec<String> = extra_args
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            args.extend(extra);
        }

        // 如果主人通过 extra_args 给 edge 设置了管理口令，恩兔也悄悄记下来（用于 Management API 查询）
        {
            let pw = extract_management_password(config.extra_args.as_deref());
            *self.mgmt_password.lock().unwrap() = pw;
        }

        log::info!("启动 N2N edge: {} {:?}", edge_path, args);

        // 启动进程
        let mut cmd = Command::new(&edge_path);
        cmd.args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Windows 下别让 edge 额外弹出黑框框（恩兔会把工具箱悄悄拿出来干活）
        #[cfg(target_os = "windows")]
        {
            cmd.creation_flags(CREATE_NO_WINDOW);
        }

        // 在 Linux 上可能需要提权
        #[cfg(target_os = "linux")]
        {
            // 检查是否有 root 权限
            if !nix::unistd::Uid::effective().is_root() {
                log::warn!("N2N 通常需要 root 权限，当前可能无法正常工作");
            }
        }

        let mut child = match cmd.spawn().context("启动 N2N edge 进程失败") {
            Ok(child) => child,
            Err(e) => {
                *self.status.lock().unwrap() = ConnectionStatus::Error(e.to_string());
                return Err(e);
            }
        };

        // 捕获 stdout 和 stderr
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let status_clone = Arc::clone(&self.status);
        let last_notice = Arc::clone(&self.last_notice);
        let log_tx_clone = self.log_tx.clone();
        let stop_requested = Arc::clone(&self.stop_requested);

        // 启动线程读取输出
        if let Some(stdout) = stdout {
            let log_tx = log_tx_clone.clone();
            let status = Arc::clone(&status_clone);
            let stop_requested = Arc::clone(&stop_requested);
            let last_notice = Arc::clone(&last_notice);
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                let mut network_info: Option<NetworkInfo> = None;
                
                for line in reader.lines() {
                    if let Ok(line) = line {
                        log::info!("N2N stdout: {}", line);

                        // 如果主人已经让恩兔“收拾工具”，就别再用 ERROR 把主人吓一跳啦
                        if stop_requested.load(Ordering::SeqCst) {
                            if let Some(ref tx) = log_tx {
                                let _ = tx.send(format!("[OUT] {}", line));
                            }
                            continue;
                        }

                        // 提取网卡信息：created local tap device IP: xxx.xxx.xxx.xxx, Mask: 255.255.255.0, MAC: xx:xx:xx:xx:xx:xx
                        if line.contains("created local tap device") {
                            if let Some(info) = parse_network_info(&line) {
                                network_info = Some(info);
                                log::info!("提取到网卡信息：{:?}", network_info);
                                // 如果已经连接成功了，就把详情也补写进状态里（给主人递上“通道回执单”）
                                if let Some(ref info) = network_info {
                                    let current = status.lock().unwrap().clone();
                                    if matches!(current, ConnectionStatus::Connected(_)) {
                                        *status.lock().unwrap() = ConnectionStatus::Connected(Some(info.clone()));
                                    }
                                }
                            }
                        }

                        // 检测连接成功的关键字
                        // 备注：不同版本 edge 输出不完全一致，这里做兼容匹配
                        if line.contains("edge <<<")
                            || line.contains("[OK] edge <<<")
                        {
                            *status.lock().unwrap() = ConnectionStatus::Connected(network_info.clone());
                            // 连接成功就把“提醒便签”撕掉，避免主人继续担心
                            *last_notice.lock().unwrap() = None;
                        }

                        // 识别常见问题并提示给主人（注意：edge 的 ERROR 可能出现在 stdout）
                        if let Some(notice) = extract_user_facing_notice(&line) {
                            set_last_notice_if_changed(&last_notice, notice);
                        }

                        // 检测启动警告（仅日志提示，不改变状态）
                        if line.contains("WARNING")
                            && (line.contains("failed")
                                || line.contains("invalid")
                                || line.contains("malformed"))
                        {
                            log::warn!("N2N 启动警告：{}", line);
                        }
                        
                        if let Some(ref tx) = log_tx {
                            let _ = tx.send(format!("[OUT] {}", line));
                        }
                    }
                }
            });
        }

        if let Some(stderr) = stderr {
            let log_tx = log_tx_clone;
            let stop_requested = Arc::clone(&stop_requested);
            let last_notice = Arc::clone(&last_notice);
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        log::warn!("N2N stderr: {}", line);

                        if stop_requested.load(Ordering::SeqCst) {
                            if let Some(ref tx) = log_tx {
                                let _ = tx.send(format!("[ERR] {}", line));
                            }
                            continue;
                        }
                        
                        // 识别常见问题并提示给主人（stderr 里也会冒出关键 ERROR）
                        if let Some(notice) = extract_user_facing_notice(&line) {
                            set_last_notice_if_changed(&last_notice, notice);
                        }
                        
                        if let Some(ref tx) = log_tx {
                            let _ = tx.send(format!("[ERR] {}", line));
                        }
                    }
                }
            });
        }

        // 保存子进程句柄
        *self.child.lock().unwrap() = Some(child);
        
        // 保存配置以支持自动重连
        *self.auto_reconnect.lock().unwrap() = Some(config.clone());

        // 后台启动“管理口状态刷新”小工人（缓存连接状态/时间戳等）
        self.start_mgmt_worker_if_needed();

        // 后台启动“同伴点名 + 延迟测量”小工人
        // - 说明：它只在已连接时工作；断开/退出时会自动收工
        // - 注意：必须在 child 句柄写入后再启动，否则小工人会误判“没有在工作”而提前收工
        self.start_peer_worker_if_needed();
        
        // 启动进程监控线程
        let _ = self.start_monitor();

        Ok(())
    }
    
    /// 启动进程监控线程（用于自动重连）
    fn start_monitor(&self) -> Result<()> {
        let child_clone = Arc::clone(&self.child);
        let status_clone = Arc::clone(&self.status);
        let last_notice = Arc::clone(&self.last_notice);
        let log_tx_clone = self.log_tx.clone();
        let stop_requested = Arc::clone(&self.stop_requested);
        
        thread::spawn(move || {
            loop {
            // 断开流程可能较长，这里加快轮询以便 UI 更快感知退出
            thread::sleep(std::time::Duration::from_secs(1));
                
                let mut child_guard = child_clone.lock().unwrap();
                
                if let Some(child) = child_guard.as_mut() {
                    // 检查进程是否还在运行
                    match child.try_wait() {
                        Ok(Some(exit_status)) => {
                            if stop_requested.load(Ordering::SeqCst) {
                                log::info!("N2N 进程已退出（优雅断开完成），状态：{:?}", exit_status);
                            } else {
                                log::warn!("N2N 进程意外退出，状态：{:?}", exit_status);
                            }
                            
                            if let Some(ref tx) = log_tx_clone {
                                if stop_requested.load(Ordering::SeqCst) {
                                    let _ = tx.send("[INFO] N2N 进程已断开".to_string());
                                } else {
                                    let _ = tx.send(format!("[WARN] N2N 进程意外退出：{:?}", exit_status));
                                }
                            }
                            
                            // 清除子进程句柄
                            *child_guard = None;
                            drop(child_guard);

                            // 主人主动断开：回到待命；否则：进程都摔倒了，必须给主人一个“出错了”的交代
                            if stop_requested.load(Ordering::SeqCst) {
                                *status_clone.lock().unwrap() = ConnectionStatus::Disconnected;
                                *last_notice.lock().unwrap() = None;
                            } else {
                                let msg = last_notice
                                    .lock()
                                    .unwrap()
                                    .clone()
                                    .unwrap_or_else(|| "error_edge_exited".to_string());
                                *status_clone.lock().unwrap() = ConnectionStatus::Error(msg);
                            }
                            break;
                        }
                        Ok(None) => {
                            // 进程仍在运行
                        }
                        Err(e) => {
                            log::error!("检查进程状态失败：{}", e);
                        }
                    }
                } else {
                    // 没有运行中的进程，退出监控线程
                    break;
                }
            }
        });
        Ok(())
    }

    /// 停止 N2N edge 进程
    pub fn stop(&self) -> Result<()> {
        // 清除自动重连配置
        *self.auto_reconnect.lock().unwrap() = None;

        // 标记为主动停止，避免读线程把退出误判为错误
        self.stop_requested.store(true, Ordering::SeqCst);
        // 主人都叫停了，就别再拿旧的“提醒便签”继续叨叨啦
        *self.last_notice.lock().unwrap() = None;

        // 立刻切换状态，UI 侧可提示用户等待
        *self.status.lock().unwrap() = ConnectionStatus::Disconnecting;

        // 优先用 Management API 的 stop 来“礼貌请离”，避免 Linux 下还得借 pkexec 才能发信号
        // - 备注：写操作通常需要认证；如果主人没设置，默认密码是 n2n
        if self.try_management_stop().is_ok() {
            // stop 已递出：把缓存收一收，UI 就不会展示旧信息啦
            self.reset_peer_state();
            self.reset_mgmt_state();
            return Ok(());
        }

        // Management API stop 失败：再走传统 SIGINT 路线兜底
        self.reset_peer_state();
        self.reset_mgmt_state();

        let child_guard = self.child.lock().unwrap();
        if let Some(child) = child_guard.as_ref() {
            let pid = child.id() as i32;
            log::info!("开始优雅停止 N2N edge 进程（SIGINT），PID: {}", pid);
            
            // edge 启动后会 setuid 降权到 nobody，普通用户无法直接发送信号
            // 需要通过 pkexec/sudo 来发送信号
            #[cfg(target_os = "linux")]
            {
                // 先尝试直接发送（如果是 root 或者 edge 没有降权）
                let direct_result = kill(Pid::from_raw(pid), Signal::SIGINT);
                if direct_result.is_err() {
                    log::info!("直接发送 SIGINT 失败（可能 edge 已降权），尝试通过 pkexec 发送");
                    // 使用 pkexec kill 发送信号
                    let status = Command::new("pkexec")
                        .arg("kill")
                        .arg("-SIGINT")
                        .arg(pid.to_string())
                        .status();
                    match status {
                        Ok(s) if s.success() => {
                            log::info!("通过 pkexec 发送 SIGINT 成功");
                        }
                        Ok(s) => {
                            log::warn!("pkexec kill 返回非零退出码：{:?}", s.code());
                        }
                        Err(e) => {
                            log::error!("执行 pkexec kill 失败：{}", e);
                        }
                    }
                } else {
                    log::info!("直接发送 SIGINT 成功");
                }
            }
            
            #[cfg(all(unix, not(target_os = "linux")))]
            {
                let _ = kill(Pid::from_raw(pid), Signal::SIGINT);
            }

            #[cfg(windows)]
            {
                // Windows 没有 SIGINT，恩兔就改用系统自带的 taskkill 来“轻轻拍一下肩膀”
                // 备注：不加 /F 代表尽量温柔；如果 edge 不听话，主人还可以用“强制断开”
                let status = Command::new("taskkill")
                    .arg("/PID")
                    .arg(pid.to_string())
                    .status();
                match status {
                    Ok(s) if s.success() => {
                        log::info!("Windows taskkill 已发送停止请求（PID: {}）", pid);
                    }
                    Ok(s) => {
                        log::warn!("Windows taskkill 返回非零退出码：{:?}", s.code());
                    }
                    Err(e) => {
                        log::error!("Windows taskkill 执行失败：{}", e);
                    }
                }
            }
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("N2N 进程未运行"))
        }
    }

    /// 强制停止 N2N edge 进程（SIGKILL），用于优雅退出卡住时兜底
    pub fn stop_force(&self) -> Result<()> {
        // 清除自动重连配置
        *self.auto_reconnect.lock().unwrap() = None;

        self.stop_requested.store(true, Ordering::SeqCst);
        *self.last_notice.lock().unwrap() = None;

        // 强制停工也要把“点名册/延迟表”收拾干净
        self.reset_peer_state();
        self.reset_mgmt_state();

        let mut child_guard = self.child.lock().unwrap();
        if let Some(child) = child_guard.as_mut() {
            let pid = child.id() as i32;
            log::warn!("强制停止 N2N edge 进程（SIGKILL），PID: {}", pid);
            
            // edge 启动后会 setuid 降权到 nobody，普通用户无法直接发送信号
            #[cfg(target_os = "linux")]
            {
                // 先尝试直接发送
                let direct_result = kill(Pid::from_raw(pid), Signal::SIGKILL);
                if direct_result.is_err() {
                    log::info!("直接发送 SIGKILL 失败，尝试通过 pkexec 发送");
                    let _ = Command::new("pkexec")
                        .arg("kill")
                        .arg("-9")
                        .arg(pid.to_string())
                        .status();
                }
            }
            
            #[cfg(all(unix, not(target_os = "linux")))]
            {
                let _ = kill(Pid::from_raw(pid), Signal::SIGKILL);
            }

            #[cfg(windows)]
            {
                // Windows 上就用“掸子重击”模式：/T 递归清理子进程，/F 强制结束
                let status = Command::new("taskkill")
                    .arg("/T")
                    .arg("/F")
                    .arg("/PID")
                    .arg(pid.to_string())
                    .status();
                match status {
                    Ok(s) if s.success() => {
                        log::warn!("Windows taskkill 已强制清理（PID: {}）", pid);
                    }
                    Ok(s) => {
                        log::warn!("Windows taskkill（强制）返回非零退出码：{:?}", s.code());
                    }
                    Err(e) => {
                        log::error!("Windows taskkill（强制）执行失败：{}", e);
                    }
                }
            }

            // 尝试快速回收子进程，避免残留/僵尸
            let deadline = Instant::now() + Duration::from_secs(3);
            loop {
                match child.try_wait() {
                    Ok(Some(_)) => {
                        *child_guard = None;
                        break;
                    }
                    Ok(None) => {
                        if Instant::now() >= deadline {
                            break;
                        }
                        thread::sleep(Duration::from_millis(100));
                    }
                    Err(e) => {
                        log::error!("强制停止后检查进程状态失败：{}", e);
                        break;
                    }
                }
            }

            *self.status.lock().unwrap() = ConnectionStatus::Disconnected;
            Ok(())
        } else {
            Err(anyhow::anyhow!("N2N 进程未运行"))
        }
    }

    /// 检查进程是否在运行
    pub fn is_running(&self) -> bool {
        let child_guard = self.child.lock().unwrap();
        child_guard.is_some()
    }

    /// 获取当前状态
    pub fn status(&self) -> ConnectionStatus {
        self.status.lock().unwrap().clone()
    }

    /// 取出最近一次“需要主人注意”的提示
    /// - 说明：这不等价于“致命错误”；有些情况 edge 会继续重试（例如 MAC/IP 未释放）
    pub fn last_notice(&self) -> Option<String> {
        self.last_notice.lock().unwrap().clone()
    }

    /// 尝试给出更“客观”的提示信息：
    /// - 优先使用从 stdout/stderr 抓到的明确错误
    /// - 否则用 Management API 的时间戳做保守推断（例如：长时间收不到 supernode 心跳）
    pub fn derived_notice(&self) -> Option<String> {
        if let Some(n) = self.last_notice() {
            return Some(n);
        }

        let raw = self.status.lock().unwrap().clone();
        if matches!(
            raw,
            ConnectionStatus::Connecting | ConnectionStatus::Connected(_)
        ) {
            let st = self.mgmt_state.lock().unwrap().clone();
            if let Some(ts) = st.timestamps {
                let now = unix_now_seconds();
                if ts.last_super == 0 {
                    // edge 已经开工一会儿但 still 没摸到 supernode，就先给主人一个“总部不可达”的温柔提示
                    if now.saturating_sub(ts.start_time) > EDGE_STARTUP_WAIT_SECS {
                        return Some("error_supernode_unreachable".to_string());
                    }
                } else {
                    // 已经连上过：如果心跳太久没更新，多半是总部断联了
                    if now.saturating_sub(ts.last_super) > HEARTBEAT_DISCONNECT_THRESHOLD_SECS {
                        return Some("error_supernode_unreachable".to_string());
                    }
                }
            }

            // 最后兜底：把管理口错误原样返还（主要用于调试）
            if let Some(e) = st.last_error {
                return Some(e);
            }
        }

        None
    }

    /// 基于 Management API 缓存推断“是否已连上 supernode”（用于 UI 状态显示）
    pub fn mgmt_is_connected(&self) -> bool {
        self.mgmt_state.lock().unwrap().is_connected()
    }

    /// 基于 Management API 缓存，给出“更像事实”的连接状态（尽量不依赖 stdout 文本匹配）
    pub fn derived_status(&self) -> ConnectionStatus {
        let raw = self.status.lock().unwrap().clone();
        match raw {
            ConnectionStatus::Disconnecting | ConnectionStatus::Disconnected | ConnectionStatus::Error(_) => raw,
            ConnectionStatus::Connecting | ConnectionStatus::Connected(_) => {
                if self.mgmt_is_connected() {
                    // 保留 stdout 里提取到的网卡信息（如果有），但不把“是否已连接”这件事绑死在 stdout 上
                    let network_info = match raw {
                        ConnectionStatus::Connected(info) => info,
                        _ => None,
                    };
                    ConnectionStatus::Connected(network_info)
                } else {
                    ConnectionStatus::Connecting
                }
            }
        }
    }

    /// 把“同伴点名册”递给主人（前端展示用）
    pub fn peers_snapshot(&self) -> Vec<PeerNodeInfo> {
        let peers = self.peer_cache.lock().unwrap().clone();
        let latency = self.peer_latency.lock().unwrap().clone();

        peers
            .into_iter()
            .map(|mut p| {
                if let Some(ref ip) = p.vpn_ip {
                    if let Some((ms, ts)) = latency.get(ip) {
                        p.latency_ms = Some(*ms);
                        p.last_ping = Some(*ts);
                    }
                }
                p
            })
            .collect()
    }

    /// 获取默认的 edge 可执行文件路径
    fn get_default_edge_path(&self) -> String {
        #[cfg(target_os = "windows")]
        {
            // Windows 下在程序目录的 bin 子目录查找
            "bin/edge.exe".to_string()
        }

        #[cfg(not(target_os = "windows"))]
        {
            // Linux/macOS 下假设在 PATH 中或程序目录
            if which::which("edge").is_ok() {
                "edge".to_string()
            } else {
                "./bin/edge".to_string()
            }
        }
    }

    /// 清空同伴相关状态（断开/停止时调用）
    fn reset_peer_state(&self) {
        self.peer_cache.lock().unwrap().clear();
        self.peer_latency.lock().unwrap().clear();
        // 允许下次连接重新启动后台小工人
        self.peer_worker_started.store(false, Ordering::SeqCst);
    }

    /// 清空管理口状态缓存（断开/停止时调用）
    fn reset_mgmt_state(&self) {
        *self.mgmt_state.lock().unwrap() = MgmtState::default();
        self.mgmt_worker_started.store(false, Ordering::SeqCst);
    }

    /// 尝试通过 Management API 让 edge 优雅退出（stop）
    fn try_management_stop(&self) -> Result<()> {
        // stop 属于写操作：如果主人没配置密码，就先试默认 n2n
        let pw = self
            .mgmt_password
            .lock()
            .unwrap()
            .clone()
            .or_else(|| Some("n2n".to_string()));

        let tag = next_mgmt_tag();
        let options = match pw.as_deref() {
            Some(p) if !p.is_empty() => format!("{tag}:1:{p}"),
            _ => tag.clone(),
        };
        let req = format!("w {options} stop\n");
        let socket = send_mgmt_request(
            &req,
            "准备 stop 纸条失败",
            "把 stop 纸条递给 edge（Management API）失败",
        )?;
        let deadline = Instant::now() + Duration::from_millis(MGMT_STOP_TIMEOUT_MS);
        wait_mgmt_end(&socket, &tag, deadline, "读取 stop 回信失败", "Management API stop 失败")
    }

    /// 启动后台“点名 + 测延迟”小工人（仅一次）
    fn start_peer_worker_if_needed(&self) {
        if self
            .peer_worker_started
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return;
        }

        let child = Arc::clone(&self.child);
        let stop_requested = Arc::clone(&self.stop_requested);
        let mgmt_password = Arc::clone(&self.mgmt_password);
        let mgmt_state = Arc::clone(&self.mgmt_state);
        let peer_cache = Arc::clone(&self.peer_cache);
        let peer_latency = Arc::clone(&self.peer_latency);
        let peer_worker_started = Arc::clone(&self.peer_worker_started);

        thread::spawn(move || {
            let mut fail_streak = 0u32;
            loop {
                if stop_requested.load(Ordering::SeqCst) {
                    break;
                }

                if child.lock().unwrap().is_none() {
                    break;
                }

                if !mgmt_state.lock().unwrap().is_connected() {
                    thread::sleep(Duration::from_secs(1));
                    continue;
                }

                let pw = mgmt_password.lock().unwrap().clone();
                match query_edges_from_management_api(pw.as_deref()) {
                    Ok(mut peers) => {
                        fail_streak = 0;

                        // 过滤掉本机条目（如果有的话），只给主人看“其他伙伴”
                        peers.retain(|p| p.is_local != Some(true));

                        // 逐个 ping 一下，给主人一个“距离感”
                        let now = unix_now_seconds();
                        for p in peers.iter_mut() {
                            let Some(ref ip) = p.vpn_ip else { continue };
                            if let Ok(Some(ms)) = ping_once(ip, 1000) {
                                peer_latency.lock().unwrap().insert(ip.clone(), (ms, now));
                            }
                        }

                        *peer_cache.lock().unwrap() = peers;
                    }
                    Err(e) => {
                        fail_streak = fail_streak.saturating_add(1);
                        // 别太吵：连续失败时降低频率，避免把日志刷爆
                        log::debug!("Management API 查询失败：{}", e);
                    }
                }

                let sleep_secs = if fail_streak >= 3 { 10 } else { 5 };
                thread::sleep(Duration::from_secs(sleep_secs));
            }

            // 收工：清空缓存，避免主人看到“过期点名册”
            peer_cache.lock().unwrap().clear();
            peer_latency.lock().unwrap().clear();
            peer_worker_started.store(false, Ordering::SeqCst);
        });
    }

    /// 启动后台“管理口状态刷新”小工人（仅一次）
    fn start_mgmt_worker_if_needed(&self) {
        if self
            .mgmt_worker_started
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            return;
        }

        let child = Arc::clone(&self.child);
        let stop_requested = Arc::clone(&self.stop_requested);
        let mgmt_password = Arc::clone(&self.mgmt_password);
        let mgmt_state = Arc::clone(&self.mgmt_state);
        let mgmt_worker_started = Arc::clone(&self.mgmt_worker_started);

        thread::spawn(move || {
            let mut fail_streak = 0u32;
            loop {
                if stop_requested.load(Ordering::SeqCst) {
                    break;
                }
                if child.lock().unwrap().is_none() {
                    break;
                }

                // 读操作理论上不需要密码，但有些动作（如 subscribe/stop）会要求认证；
                // 这里顺便把已知密码带上，避免环境差异导致读不到状态。
                let pw = mgmt_password.lock().unwrap().clone();

                match query_mgmt_state_snapshot(pw.as_deref()) {
                    Ok(snapshot) => {
                        fail_streak = 0;
                        *mgmt_state.lock().unwrap() = snapshot;
                    }
                    Err(e) => {
                        fail_streak = fail_streak.saturating_add(1);
                        mgmt_state.lock().unwrap().last_error = Some(e.to_string());
                    }
                }

                let sleep_ms = if fail_streak >= 3 { 3000 } else { 1200 };
                thread::sleep(Duration::from_millis(sleep_ms));
            }

            *mgmt_state.lock().unwrap() = MgmtState::default();
            mgmt_worker_started.store(false, Ordering::SeqCst);
        });
    }
}

/// Management API 的 tag 自增器（让每次点名都有自己的编号）
static MGMT_TAG_COUNTER: AtomicU32 = AtomicU32::new(1);

#[derive(Debug, Clone, Default)]
struct MgmtState {
    timestamps: Option<MgmtTimestampsRow>,
    last_error: Option<String>,
}

impl MgmtState {
    fn is_connected(&self) -> bool {
        let Some(ts) = self.timestamps.as_ref() else { return false };
        let now = unix_now_seconds();
        let super_ok = ts.last_super != 0 && now.saturating_sub(ts.last_super) <= HEARTBEAT_MAX_INTERVAL_SECS;
        let p2p_ok = ts.last_p2p != 0 && now.saturating_sub(ts.last_p2p) <= HEARTBEAT_MAX_INTERVAL_SECS;
        // edge 的 last_super 通常几秒内就会更新；这里给一个宽松阈值，避免短暂抖动把 UI 误判成“断开”
        super_ok || p2p_ok
    }
}

#[derive(Debug, serde::Deserialize)]
struct MgmtPacket {
    #[serde(rename = "_tag")]
    tag: Option<String>,
    #[serde(rename = "_type")]
    kind: String,
    error: Option<String>,
    #[serde(flatten)]
    extra: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct MgmtTimestampsRow {
    start_time: u64,
    last_super: u64,
    last_p2p: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct MgmtEdgeRow {
    mode: Option<String>,
    ip4addr: Option<String>,
    sockaddr: Option<String>,
    desc: Option<String>,
    #[serde(alias = "lastseen")]
    last_seen: Option<u64>,
    local: Option<u64>,
}

fn unix_now_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_secs()
}

/// 从 extra_args 中悄悄摸出 management password（如果主人给了的话）
fn extract_management_password(extra_args: Option<&str>) -> Option<String> {
    let s = extra_args?;
    let mut it = s.split_whitespace().peekable();
    while let Some(tok) = it.next() {
        if tok == "--management-password" {
            return it.next().map(|v| v.to_string());
        }
    }
    None
}

fn next_mgmt_tag() -> String {
    let n = MGMT_TAG_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}", n % 1000)
}

fn build_mgmt_request(method: &str, password: Option<&str>) -> (String, String) {
    let tag = next_mgmt_tag();
    let options = match password {
        Some(pw) if !pw.is_empty() => format!("{tag}:1:{pw}"),
        _ => tag.clone(),
    };
    // 备注：协议要求单行、<=80 bytes；这里 method 都很短
    (tag, format!("r {options} {method}\n"))
}

fn send_mgmt_request(req: &str, prepare_hint: &'static str, send_hint: &'static str) -> Result<UdpSocket> {
    let socket = UdpSocket::bind(("127.0.0.1", 0)).context(prepare_hint)?;
    socket
        .set_read_timeout(Some(Duration::from_millis(MGMT_READ_TIMEOUT_MS)))
        .ok();
    socket.send_to(req.as_bytes(), MGMT_ADDR).context(send_hint)?;
    Ok(socket)
}

fn parse_mgmt_packet(buf: &[u8]) -> Option<MgmtPacket> {
    let mut s = String::from_utf8_lossy(buf).to_string();
    // edge 回包尾部可能带 \0，让恩兔把它扫掉
    s = s.trim_matches('\u{0}').trim().to_string();
    serde_json::from_str(&s).ok()
}

fn collect_mgmt_rows_json(socket: &UdpSocket, tag: &str, deadline: Instant) -> Result<Vec<serde_json::Value>> {
    let mut buf = vec![0u8; 65535];
    let mut rows: Vec<serde_json::Value> = Vec::new();

    while Instant::now() < deadline {
        let (n, _) = match socket.recv_from(&mut buf) {
            Ok(v) => v,
            Err(e) if should_stop_mgmt_read(&e) => break,
            Err(e) => return Err(anyhow::anyhow!("读取 Management API 回信失败：{}", e)),
        };

        let pkt = match parse_mgmt_packet(&buf[..n]) {
            Some(v) => v,
            None => continue,
        };

        if pkt.tag.as_deref() != Some(tag) {
            continue;
        }

        match pkt.kind.as_str() {
            "error" => {
                let err = pkt.error.unwrap_or_else(|| "unknown".to_string());
                return Err(anyhow::anyhow!("Management API 返回错误：{}", err));
            }
            "row" => rows.push(serde_json::Value::Object(pkt.extra)),
            "end" => break,
            _ => {}
        }
    }

    Ok(rows)
}

fn wait_mgmt_end(
    socket: &UdpSocket,
    tag: &str,
    deadline: Instant,
    read_error_hint: &str,
    error_prefix: &str,
) -> Result<()> {
    let mut buf = vec![0u8; 65535];
    while Instant::now() < deadline {
        let (n, _) = match socket.recv_from(&mut buf) {
            Ok(v) => v,
            Err(e) if should_stop_mgmt_read(&e) => break,
            Err(e) => return Err(anyhow::anyhow!("{}：{}", read_error_hint, e)),
        };

        let pkt = match parse_mgmt_packet(&buf[..n]) {
            Some(v) => v,
            None => continue,
        };

        if pkt.tag.as_deref() != Some(tag) {
            continue;
        }

        if pkt.kind == "error" {
            let err = pkt.error.unwrap_or_else(|| "unknown".to_string());
            return Err(anyhow::anyhow!("{}：{}", error_prefix, err));
        }
        if pkt.kind == "end" {
            break;
        }
    }

    Ok(())
}

fn query_mgmt_state_snapshot(password: Option<&str>) -> Result<MgmtState> {
    let timestamps = query_mgmt_single_row::<MgmtTimestampsRow>("timestamps", password)?;

    Ok(MgmtState {
        timestamps,
        last_error: None,
    })
}

fn query_mgmt_single_row<T>(method: &str, password: Option<&str>) -> Result<Option<T>>
where
    T: serde::de::DeserializeOwned,
{
    let mut rows = query_mgmt_rows::<T>(method, password)?;
    Ok(rows.pop())
}

fn query_mgmt_rows<T>(method: &str, password: Option<&str>) -> Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let rows_json = query_mgmt_rows_json(method, password)?;
    let mut out = Vec::new();
    for row in rows_json {
        if let Ok(v) = serde_json::from_value::<T>(row) {
            out.push(v);
        }
    }
    Ok(out)
}

fn query_mgmt_rows_json(method: &str, password: Option<&str>) -> Result<Vec<serde_json::Value>> {
    match query_mgmt_rows_json_once(method, password) {
        Ok(v) => Ok(v),
        Err(e) => {
            // 默认密码是 n2n：如果主人没配置密码且遇到 badauth，就用默认钥匙再试一次
            if password.is_none() && e.to_string().contains("badauth") {
                return query_mgmt_rows_json_once(method, Some("n2n"));
            }
            Err(e)
        }
    }
}

/// 通过 Management API 查询多行结果（JSON 格式）
fn query_mgmt_rows_json_once(method: &str, password: Option<&str>) -> Result<Vec<serde_json::Value>> {
    let (tag, req) = build_mgmt_request(method, password);
    let deadline = Instant::now() + Duration::from_millis(MGMT_DEADLINE_MS);
    let socket = send_mgmt_request(
        &req,
        "准备 Management API 询问纸条失败",
        "把询问纸条递给 edge（Management API）失败",
    )?;
    collect_mgmt_rows_json(&socket, &tag, deadline)
}

fn query_edges_from_management_api(password: Option<&str>) -> Result<Vec<PeerNodeInfo>> {
    match query_edges_from_management_api_once(password) {
        Ok(v) => Ok(v),
        Err(e) => {
            // 默认密码是 n2n：如果主人没配置密码且遇到 badauth，就用默认钥匙再试一次
            if password.is_none() && e.to_string().contains("badauth") {
                return query_edges_from_management_api_once(Some("n2n"));
            }
            Err(e)
        }
    }
}

/// 通过 Management API 查询同伴列表（edges）
fn query_edges_from_management_api_once(password: Option<&str>) -> Result<Vec<PeerNodeInfo>> {
    let (tag, req) = build_mgmt_request("edges", password);
    let deadline = Instant::now() + Duration::from_millis(MGMT_DEADLINE_MS);
    let socket = send_mgmt_request(
        &req,
        "准备 Management API 询问纸条失败",
        "把询问纸条递给 edge（Management API）失败",
    )?;
    let rows = collect_mgmt_rows_json(&socket, &tag, deadline)?;

    let mut peers = Vec::new();
    for row in rows {
        let parsed: MgmtEdgeRow = match serde_json::from_value(row) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let vpn_ip = parsed.ip4addr.as_deref().and_then(|s| s.split('/').next()).map(|s| s.to_string());
        peers.push(PeerNodeInfo {
            name: parsed.desc,
            vpn_addr: parsed.ip4addr,
            vpn_ip,
            public_addr: parsed.sockaddr,
            mode: parsed.mode,
            last_seen: parsed.last_seen,
            is_local: parsed.local.map(|v| v != 0),
            latency_ms: None,
            last_ping: None,
        });
    }

    Ok(peers)
}

fn ping_once(ip: &str, timeout_ms: u64) -> Result<Option<f64>> {
    let mut cmd = Command::new("ping");
    #[cfg(target_os = "windows")]
    {
        // Windows GUI 程序里频繁调用 ping 会让黑框闪现，恩兔把它悄悄藏起来～
        cmd.creation_flags(CREATE_NO_WINDOW);
        cmd.args(["-n", "1", "-w", &timeout_ms.to_string(), ip]);
    }
    #[cfg(not(target_os = "windows"))]
    {
        let sec = ((timeout_ms + 999) / 1000).max(1);
        cmd.args(["-n", "-c", "1", "-W", &sec.to_string(), ip]);
    }

    let output = match cmd.output() {
        Ok(o) => o,
        Err(_) => return Ok(None),
    };

    #[cfg(target_os = "windows")]
    let combined = {
        // Windows 的 ping 输出通常是本地代码页（例如 GBK/CP936），直接按 UTF-8 读会变成乱码，
        // 进而解析不到“时间=xxms”，导致延迟一直是“-”。
        let stdout = decode_windows_cmd_output(&output.stdout);
        let stderr = decode_windows_cmd_output(&output.stderr);
        format!("{stdout}\n{stderr}")
    };
    #[cfg(not(target_os = "windows"))]
    let combined = {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        format!("{stdout}\n{stderr}")
    };

    Ok(parse_ping_latency_ms(&combined))
}

#[cfg(target_os = "windows")]
fn decode_windows_cmd_output(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    // 少数环境可能会直接输出 UTF-8，先试一下（省得做转换）
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }

    // Windows 命令行工具更常见的是 OEM/ANSI 代码页（例如中文 Windows 的 CP936）。
    decode_with_code_page(bytes, CP_OEMCP)
        .or_else(|| decode_with_code_page(bytes, CP_ACP))
        .unwrap_or_else(|| String::from_utf8_lossy(bytes).to_string())
}

#[cfg(target_os = "windows")]
fn decode_with_code_page(bytes: &[u8], code_page: u32) -> Option<String> {
    let len = unsafe {
        MultiByteToWideChar(
            code_page,
            0,
            bytes.as_ptr(),
            bytes.len() as i32,
            std::ptr::null_mut(),
            0,
        )
    };
    if len <= 0 {
        return None;
    }

    let mut wide = vec![0u16; len as usize];
    let written = unsafe {
        MultiByteToWideChar(
            code_page,
            0,
            bytes.as_ptr(),
            bytes.len() as i32,
            wide.as_mut_ptr(),
            len,
        )
    };
    if written <= 0 {
        return None;
    }

    String::from_utf16(&wide[..written as usize]).ok()
}

fn should_stop_mgmt_read(e: &std::io::Error) -> bool {
    // WouldBlock/TimedOut：本轮没等到回信，直接结束就好
    if e.kind() == std::io::ErrorKind::WouldBlock || e.kind() == std::io::ErrorKind::TimedOut {
        return true;
    }

    // Windows 下：UDP 目标端口尚未监听时，可能会抛出 10054（ConnectionReset）。
    // 这属于“管理口还没准备好”的正常抖动，别拿它吓主人一跳～
    #[cfg(target_os = "windows")]
    {
        if e.kind() == std::io::ErrorKind::ConnectionReset {
            return true;
        }
    }

    false
}

fn parse_ping_latency_ms(s: &str) -> Option<f64> {
    // Linux/macOS: time=12.3 ms / time<1 ms
    // Windows（中英本地化都可能出现）: time=12ms / 时间=12ms
    for key in ["time=", "time<", "时间=", "时间<"] {
        if let Some(pos) = s.find(key) {
            let rest = &s[pos + key.len()..];
            let mut num = String::new();
            for ch in rest.chars() {
                if ch.is_ascii_digit() || ch == '.' {
                    num.push(ch);
                    continue;
                }
                break;
            }
            if key.ends_with('<') {
                // time<1ms 这种就当 1ms（保守一点给主人看）
                return Some(1.0);
            }
            if !num.is_empty() {
                if let Ok(v) = num.parse::<f64>() {
                    return Some(v);
                }
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
fn resolve_edge_path_for_caps(edge_path: &str) -> Result<String> {
    // 如果是显式路径（含 /），优先使用它
    if edge_path.contains('/') {
        return Ok(edge_path.to_string());
    }
    // 否则尝试从 PATH 解析为绝对路径（pkexec/setcap 环境下更可靠）
    let resolved = which::which(edge_path).context("无法在 PATH 中找到 edge 可执行文件")?;
    Ok(resolved.to_string_lossy().to_string())
}

/// 确保 edge 二进制具备所需 capabilities
/// - 目标：在非 root 下也能创建 TAP，并能执行 drop privileges 相关系统调用
/// - 实现：使用 pkexec 运行 setcap（KDE 下由 polkit 弹窗授权）
#[cfg(target_os = "linux")]
fn ensure_edge_capabilities(edge_path: &str) -> Result<()> {
    // 需要的 capabilities：
    // - cap_net_admin/cap_net_raw：创建/配置 TAP、收发原始包
    // - cap_setuid/cap_setgid：允许 edge 在启动后 drop privileges
    // 备注：不同发行版/edge 版本可能要求略有差异，但这组在实践中更稳
    let required = ["cap_net_admin", "cap_net_raw", "cap_setuid", "cap_setgid"];
    let cap_spec = "cap_net_admin,cap_net_raw,cap_setuid,cap_setgid+eip";

    let pkexec = which::which("pkexec").context("未找到 pkexec，请安装 polkit（KDE 可用 polkit-kde-agent）")?;
    let setcap = which::which("setcap").context("未找到 setcap，请安装 libcap 工具包")?;

    // 如果 getcap 存在且已满足，就直接返回
    if let Ok(getcap) = which::which("getcap") {
        let out = Command::new(getcap)
            .arg(edge_path)
            .output()
            .context("执行 getcap 失败")?;

        let stdout = String::from_utf8_lossy(&out.stdout);
        if out.status.success() {
            let ok = required.iter().all(|c| stdout.contains(c));
            if ok {
                return Ok(());
            }
        }
    }

    log::info!("检测到非 root 运行环境，尝试为 edge 自动申请权限（pkexec + setcap）");
    let status = Command::new(pkexec)
        .arg(setcap)
        .arg(cap_spec)
        .arg(edge_path)
        .status()
        .context("执行 pkexec setcap 失败")?;

    if !status.success() {
        return Err(anyhow::anyhow!("pkexec setcap 执行失败，退出码：{:?}", status.code()));
    }

    Ok(())
}

impl Drop for N2NProcess {
    fn drop(&mut self) {
        // 进程退出时尽量避免残留子进程
        // 说明：正常点击“断开”会优雅退出；应用退出时这里也尽量先温柔收拾（SIGINT），再兜底强制清理
        if !self.is_running() {
            return;
        }

        // 先尝试温柔收拾工具（优雅退出）
        let _ = self.stop();

        // 稍等一会儿，让 edge 自己把活收尾；如果不听话，再请“掸子重击”出场
        let mut need_force = true;
        {
            let mut child_guard = self.child.lock().unwrap();
            if let Some(child) = child_guard.as_mut() {
                if let Ok(true) = wait_child_exit(child, Duration::from_millis(MGMT_STOP_TIMEOUT_MS)) {
                    *child_guard = None;
                    need_force = false;
                }
            }
        }

        if need_force {
            let _ = self.stop_force();
        }
    }
}

/// 等待子进程退出（轮询 try_wait），超时返回 false
#[allow(dead_code)]
fn wait_child_exit(child: &mut Child, timeout: Duration) -> Result<bool> {
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait().context("检查进程状态失败")? {
            Some(_status) => return Ok(true),
            None => {
                if Instant::now() >= deadline {
                    return Ok(false);
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

/// 解析网卡信息
/// 示例：created local tap device IP: 192.168.125.67, Mask: 255.255.255.0, MAC: C6:D2:CB:35:42:85
fn parse_network_info(line: &str) -> Option<NetworkInfo> {
    let ip = extract_field(line, "IP:")?;
    let mask = extract_field(line, "Mask:")?;
    let mac = extract_field(line, "MAC:")?;
    
    Some(NetworkInfo {
        ip: ip.to_string(),
        mask: mask.to_string(),
        mac: mac.to_string(),
    })
}

/// 从日志行中提取字段值
fn extract_field<'a>(line: &'a str, field: &str) -> Option<&'a str> {
    let start_idx = line.find(field)? + field.len();
    let remaining = &line[start_idx..].trim();
    
    // 提取到逗号或行尾
    let end_idx = remaining.find(',').unwrap_or(remaining.len());
    Some(remaining[..end_idx].trim())
}

/// 获取默认节点标识名称（主机名）
fn get_default_node_name() -> String {
    #[cfg(target_os = "windows")]
    {
        "n2n-maid".to_string()
    }

    #[cfg(not(target_os = "windows"))]
    {
        match nix::unistd::gethostname() {
            Ok(name) => {
                let s = name.to_string_lossy().trim().to_string();
                if s.is_empty() { "n2n-maid".to_string() } else { s }
            }
            Err(_) => "n2n-maid".to_string(),
        }
    }
}

/// 从 edge 的输出里提取一个“对主人友好”的提示文案（i18n key 或原始片段）
fn extract_user_facing_notice(line: &str) -> Option<String> {
    let l = line.to_ascii_lowercase();

    // TAP 创建被占用：典型表现是 tuntap ioctl + TUNSETIFF + Device or resource busy
    if l.contains("tunsetiff") && (l.contains("device or resource busy") || l.contains("resource busy")) {
        return Some("error_tap_busy".to_string());
    }

    // supernode 端认为 MAC/IP 还没释放：edge 会持续重试，不一定会退出
    if l.contains("authentication error")
        && l.contains("mac or ip")
        && l.contains("already in use")
    {
        return Some("error_mac_or_ip_in_use".to_string());
    }

    // 分开匹配：MAC/IP 已被占用（可能是另一个设备还在用）
    if l.contains("already in use") {
        if l.contains("mac") {
            return Some("error_mac_in_use".to_string());
        }
        if l.contains(" ip ") || l.contains("ip address") {
            return Some("error_ip_in_use".to_string());
        }
    }

    // 权限问题（Linux 常见：Operation not permitted / EPERM）
    if l.contains("operation not permitted") || l.contains("permission denied") || l.contains("eperm") {
        return Some("error_permission_denied".to_string());
    }

    // “联系不上总部”（域名解析/超时/无路由等）
    if l.contains("no route to host")
        || l.contains("network is unreachable")
        || l.contains("connection timed out")
        || l.contains("timed out")
        || l.contains("unreachable")
        || l.contains("unable to resolve")
        || l.contains("failed to resolve")
    {
        return Some("error_supernode_unreachable".to_string());
    }

    // 其他认证失败（密钥/暗号不对等）
    if l.contains("authentication error") || (l.contains("auth") && l.contains("error")) {
        return Some("error_auth_failed".to_string());
    }

    // TODO Windows 下相关错误 WSAGetLastError()
    if l.contains("wsagetlasterror") {
        return Some("error_wsagetlasterror".to_string());
    }

    // 兜底：把明显的 ERROR/failed/Cannot 行直接递给主人（原样显示）
    if l.contains("error") || l.contains("failed") || l.contains("cannot") {
        return Some(line.trim().to_string());
    }

    None
}

fn set_last_notice_if_changed(last_notice: &Arc<Mutex<Option<String>>>, notice: String) {
    let mut guard = last_notice.lock().unwrap();
    let changed = guard.as_deref() != Some(notice.as_str());
    if changed {
        *guard = Some(notice);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_creation() {
        let process = N2NProcess::new();
        assert_eq!(process.status(), ConnectionStatus::Disconnected);
        assert!(!process.is_running());
    }

    #[test]
    fn test_extract_notice_tap_busy() {
        let line = "ERROR: tuntap ioctl(TUNSETIFF, IFF_TAP) error: Device or resource busy[-1]";
        assert_eq!(
            extract_user_facing_notice(line).as_deref(),
            Some("error_tap_busy")
        );
    }

    #[test]
    fn test_extract_notice_mac_or_ip_in_use() {
        let line = "[edge_utils.c:2558] ERROR: authentication error, MAC or IP address already in use or not released yet by supernode";
        assert_eq!(
            extract_user_facing_notice(line).as_deref(),
            Some("error_mac_or_ip_in_use")
        );
    }

    #[test]
    fn test_extract_management_password_from_extra_args() {
        let args = Some("--management-password mypw -v -E");
        assert_eq!(extract_management_password(args), Some("mypw".to_string()));
        assert_eq!(extract_management_password(Some("-v -E")), None);
    }

    #[test]
    fn test_parse_ping_latency_ms_variants() {
        assert_eq!(parse_ping_latency_ms("64 bytes from 1.1.1.1: time=12.34 ms"), Some(12.34));
        assert_eq!(parse_ping_latency_ms("64 bytes from 1.1.1.1: time<1 ms"), Some(1.0));
        assert_eq!(parse_ping_latency_ms("来自 1.1.1.1 的回复：时间=23ms TTL=64"), Some(23.0));
    }
}
