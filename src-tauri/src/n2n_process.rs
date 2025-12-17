/// 通道打扫工作管理模块
/// 负责启动、停止和监控恩兔的通道打扫工作（N2N edge 进程）
use anyhow::{Context, Result};
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

#[cfg(unix)]
use nix::sys::signal::{kill, Signal};
#[cfg(unix)]
use nix::unistd::Pid;

use crate::config::N2NConfig;

/// 通道详情信息
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct NetworkInfo {
    pub ip: String,
    pub mask: String,
    pub mac: String,
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
    /// 工作汇报通道
    log_tx: Option<mpsc::UnboundedSender<String>>,
    /// 自动重连配置（断线后自动重新打扫）
    auto_reconnect: Arc<Mutex<Option<N2NConfig>>>,

    /// 是否由主人主动要求停止（用于区分"正常休息"与"意外摔倒"）
    stop_requested: Arc<AtomicBool>,
}

impl N2NProcess {
    /// 创建新的进程管理器实例
    pub fn new() -> Self {
        Self {
            child: Arc::new(Mutex::new(None)),
            status: Arc::new(Mutex::new(ConnectionStatus::Disconnected)),
            log_tx: None,
            auto_reconnect: Arc::new(Mutex::new(None)),
            stop_requested: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 设置日志发送通道
    pub fn set_log_sender(&mut self, tx: mpsc::UnboundedSender<String>) {
        self.log_tx = Some(tx);
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

        // 确定 edge 可执行文件路径
        let edge_path = config
            .edge_path
            .as_ref()
            .map(|p| p.to_string())
            .unwrap_or_else(|| self.get_default_edge_path());

        // Linux 下 edge 通常需要 root/capabilities（创建 TAP、切换权限等）
        // 这里优先尝试为 edge 二进制授予 capabilities，避免用 pkexec 包裹运行导致 stop() 无法精确控制 edge PID
        #[cfg(target_os = "linux")]
        let edge_path = {
            let mut edge_path = edge_path;
            if !nix::unistd::Uid::effective().is_root() {
                // 解析为绝对路径，避免 setcap/实际启动的二进制不一致
                edge_path = resolve_edge_path_for_caps(&edge_path)?;

                // 如果用户取消授权或系统缺少依赖，直接中止连接流程，避免后续出现更难理解的 EPERM
                ensure_edge_capabilities(&edge_path).map_err(|e| {
                    anyhow::anyhow!(
                        "需要管理员授权以配置 edge 权限（KDE 下会弹出授权窗口）。详细错误：{}",
                        e
                    )
                })?;
            }
            edge_path
        };

        // 校验 supernode 格式（必须是 host:port）
        if !config.supernode.contains(':') {
            return Err(anyhow::anyhow!("Supernode 地址格式错误，必须包含端口号（如 vpn.example.com:7777）"));
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

        log::info!("启动 N2N edge: {} {:?}", edge_path, args);

        // 启动进程
        let mut cmd = Command::new(&edge_path);
        cmd.args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Windows 下别让 edge 额外弹出黑框框（恩兔会把工具箱悄悄拿出来干活）
        #[cfg(target_os = "windows")]
        {
            const CREATE_NO_WINDOW: u32 = 0x08000000;
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

        let mut child = cmd.spawn()
            .context("启动 N2N edge 进程失败")?;

        // 捕获 stdout 和 stderr
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let status_clone = Arc::clone(&self.status);
        let log_tx_clone = self.log_tx.clone();
        let stop_requested = Arc::clone(&self.stop_requested);

        // 启动线程读取输出
        if let Some(stdout) = stdout {
            let log_tx = log_tx_clone.clone();
            let status = Arc::clone(&status_clone);
            let stop_requested = Arc::clone(&stop_requested);
            thread::spawn(move || {
                let reader = BufReader::new(stdout);
                let mut connected = false;
                let mut saw_error = None::<String>;
                let mut network_info: Option<NetworkInfo> = None;
                
                for line in reader.lines() {
                    if let Ok(line) = line {
                        log::info!("N2N stdout: {}", line);

                        // 提取网卡信息：created local tap device IP: 192.168.125.67, Mask: 255.255.255.0, MAC: C6:D2:CB:35:42:85
                        if line.contains("created local tap device") {
                            if let Some(info) = parse_network_info(&line) {
                                network_info = Some(info);
                                log::info!("提取到网卡信息：{:?}", network_info);
                            }
                        }

                        // 检测连接成功的关键字
                        // 备注：不同版本 edge 输出不完全一致，这里做兼容匹配
                        if line.contains("Registered with")
                            || line.contains("edge <<<")
                            || line.contains("edge started")
                            || line.contains("[OK] edge <<<")
                        {
                            *status.lock().unwrap() = ConnectionStatus::Connected(network_info.clone());
                            connected = true;
                        }

                        // 检测错误（注意：edge 的 ERROR 可能出现在 stdout）
                        if line.contains("ERROR")
                            || line.contains("authentication error")
                            || line.contains("already in use")
                            || line.contains("failed")
                            || line.contains("Cannot")
                        {
                            // 识别常见错误并提供友好提示
                            let error_msg = if line.contains("MAC") && line.contains("already in use") {
                                "error_mac_in_use".to_string()
                            } else if line.contains("IP") && line.contains("already in use") {
                                "error_ip_in_use".to_string()
                            } else if line.contains("TAP") || line.contains("tuntap") {
                                "error_tap_create_failed".to_string()
                            } else if line.contains("authentication") || line.contains("auth") {
                                "error_auth_failed".to_string()
                            } else if line.contains("unreachable") || line.contains("connect") {
                                "error_supernode_unreachable".to_string()
                            } else if line.contains("permission") || line.contains("EPERM") {
                                "error_permission_denied".to_string()
                            } else {
                                line.clone()
                            };
                            saw_error = Some(error_msg.clone());
                            *status.lock().unwrap() = ConnectionStatus::Error(error_msg);
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
                // stdout 关闭意味着进程退出
                log::info!("N2N stdout 读取结束，连接状态：{}", connected);

                // 主动断开：保持/切换为 Disconnected，不要标记为 Error
                if stop_requested.load(Ordering::SeqCst) {
                    *status.lock().unwrap() = ConnectionStatus::Disconnected;
                    return;
                }

                // 非主动断开：如果从未成功连接且也没捕获到具体错误，则给出兜底错误
                if !connected {
                    let current = status.lock().unwrap().clone();
                    if matches!(current, ConnectionStatus::Connecting) {
                        let msg = saw_error.unwrap_or_else(|| "进程启动失败".to_string());
                        *status.lock().unwrap() = ConnectionStatus::Error(msg);
                    }
                }
            });
        }

        if let Some(stderr) = stderr {
            let log_tx = log_tx_clone;
            let status = Arc::clone(&status_clone);
            let stop_requested = Arc::clone(&stop_requested);
            thread::spawn(move || {
                let reader = BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        log::warn!("N2N stderr: {}", line);
                        
                        // 检测错误
                        if line.contains("ERROR") || line.contains("failed") || line.contains("Cannot") {
                            // 识别常见错误并提供友好提示
                            let error_msg = if line.contains("MAC") && line.contains("already in use") {
                                "error_mac_in_use".to_string()
                            } else if line.contains("IP") && line.contains("already in use") {
                                "error_ip_in_use".to_string()
                            } else if line.contains("TAP") || line.contains("tuntap") {
                                "error_tap_create_failed".to_string()
                            } else if line.contains("authentication") || line.contains("auth") {
                                "error_auth_failed".to_string()
                            } else if line.contains("unreachable") || line.contains("connect") {
                                "error_supernode_unreachable".to_string()
                            } else if line.contains("permission") || line.contains("EPERM") {
                                "error_permission_denied".to_string()
                            } else {
                                line.clone()
                            };
                            *status.lock().unwrap() = ConnectionStatus::Error(error_msg);
                        }
                        
                        if let Some(ref tx) = log_tx {
                            let _ = tx.send(format!("[ERR] {}", line));
                        }
                    }
                }
                // stderr 关闭时，检查当前状态
                if stop_requested.load(Ordering::SeqCst) {
                    *status.lock().unwrap() = ConnectionStatus::Disconnected;
                    return;
                }

                let current = status.lock().unwrap().clone();
                if matches!(current, ConnectionStatus::Connecting) {
                    *status.lock().unwrap() = ConnectionStatus::Disconnected;
                }
            });
        }

        // 保存子进程句柄
        *self.child.lock().unwrap() = Some(child);
        
        // 保存配置以支持自动重连
        *self.auto_reconnect.lock().unwrap() = Some(config.clone());
        
        // 启动进程监控线程
        let _ = self.start_monitor();

        Ok(())
    }
    
    /// 启动进程监控线程（用于自动重连）
    fn start_monitor(&self) -> Result<()> {
        let child_clone = Arc::clone(&self.child);
        let status_clone = Arc::clone(&self.status);
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
                                    let _ = tx.send("[WARN] N2N 进程意外退出...".to_string());
                                }
                            }
                            
                            // 清除子进程句柄
                            *child_guard = None;
                            drop(child_guard);

                            *status_clone.lock().unwrap() = ConnectionStatus::Disconnected;
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

        // 立刻切换状态，UI 侧可提示用户等待
        *self.status.lock().unwrap() = ConnectionStatus::Disconnecting;

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
        // 说明：正常点击“断开”会优雅退出；应用退出时这里兜底强制清理
        let _ = self.stop_force();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_creation() {
        let process = N2NProcess::new();
        assert_eq!(process.status(), ConnectionStatus::Disconnected);
        assert!(!process.is_running());
    }
}
