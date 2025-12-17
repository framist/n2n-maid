// 防止 Windows 发布版额外蹦出黑框框（恩兔想把工作台保持干净整洁）
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod n2n_process;
mod tray;

// Windows 专属开机体检（TAP/UAC 等“高频痛点”）
#[cfg(target_os = "windows")]
mod windows_ready;

use config::{ConfigManager, N2NConfig};
use n2n_process::{ConnectionStatus, N2NProcess};
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri::State;
use tokio::sync::mpsc;
#[cfg(target_os = "windows")]
use tauri::path::BaseDirectory;

/// 恩兔酱的工作台状态
struct AppState {
    /// N2N 进程管理器（恩兔的工作记录）
    process: Arc<Mutex<N2NProcess>>,
    /// 配置管理器（主人的指示簿）
    config_manager: Arc<Mutex<ConfigManager>>,
    /// 日志接收器（工作汇报通道）
    log_rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<String>>>>,
}

/// 获取主人的指示（读取配置）
#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<N2NConfig, String> {
    let manager = state.config_manager.lock().unwrap();
    manager.load().map_err(|e| e.to_string())
}

/// 记下主人的指示（保存配置）
#[tauri::command]
async fn save_config(config: N2NConfig, state: State<'_, AppState>) -> Result<(), String> {
    let manager = state.config_manager.lock().unwrap();
    manager.save(&config).map_err(|e| e.to_string())
}

/// 开始打扫通道（启动 N2N 连接）
#[tauri::command]
async fn connect(config: N2NConfig, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let process = state.process.lock().unwrap();
    
    // 先保存配置
    let mut config = config;

    // Windows 打包模式下：优先使用资源目录里的 edge.exe（避免工作目录变化导致找不到 bin/edge.exe）
    #[cfg(target_os = "windows")]
    {
        if config.edge_path.is_none() {
            if let Ok(p) = app.path().resolve("edge.exe", BaseDirectory::Resource) {
                if p.exists() {
                    config.edge_path = Some(p.to_string_lossy().to_string());
                }
            }
            if config.edge_path.is_none() {
                if let Ok(p) = app.path().resolve("bin/edge.exe", BaseDirectory::Resource) {
                    if p.exists() {
                        config.edge_path = Some(p.to_string_lossy().to_string());
                    }
                }
            }
        }
    }

    let manager = state.config_manager.lock().unwrap();
    manager.save(&config).map_err(|e| e.to_string())?;
    drop(manager);
    
    // 启动连接
    process.start(&config).map_err(|e| e.to_string())?;
    
    // 更新托盘状态
    let status = process.status();
    let _ = tray::update_tray_menu(&app, &status);
    
    Ok(())
}

/// 收拾工具休息（断开 N2N 连接）
#[tauri::command]
async fn disconnect(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let process = state.process.lock().unwrap();
    process.stop().map_err(|e| e.to_string())?;
    
    // 更新托盘状态
    let status = process.status();
    let _ = tray::update_tray_menu(&app, &status);
    
    Ok(())
}

/// 立即停止工作（强制断开，用于温柔关闭卡住时）
#[tauri::command]
async fn disconnect_force(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let process = state.process.lock().unwrap();
    process.stop_force().map_err(|e| e.to_string())?;

    // 更新托盘状态
    let status = process.status();
    let _ = tray::update_tray_menu(&app, &status);

    Ok(())
}

/// 查看工作状态（获取连接状态）
#[tauri::command]
async fn get_status(state: State<'_, AppState>) -> Result<serde_json::Value, String> {
    let process = state.process.lock().unwrap();
    let status = process.status();
    
    let result = match status {
        ConnectionStatus::Disconnected => serde_json::json!({
            "status": "disconnected",
            "error": null,
            "networkInfo": null
        }),
        ConnectionStatus::Connecting => serde_json::json!({
            "status": "connecting",
            "error": null,
            "networkInfo": null
        }),
        ConnectionStatus::Disconnecting => serde_json::json!({
            "status": "disconnecting",
            "error": null,
            "networkInfo": null
        }),
        ConnectionStatus::Connected(network_info) => serde_json::json!({
            "status": "connected",
            "error": null,
            "networkInfo": network_info
        }),
        ConnectionStatus::Error(msg) => serde_json::json!({
            "status": "error",
            "error": msg,
            "networkInfo": null
        }),
    };
    
    Ok(result)
}

/// 获取工作汇报（读取日志）
#[tauri::command]
async fn get_logs(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let mut rx_guard = state.log_rx.lock().unwrap();
    let mut logs = Vec::new();
    
    if let Some(rx) = rx_guard.as_mut() {
        while let Ok(log) = rx.try_recv() {
            logs.push(log);
        }
    }
    
    Ok(logs)
}

fn main() {
    // 初始化日志系统
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 创建工作汇报通道
    let (log_tx, log_rx) = mpsc::unbounded_channel();
    
    // 唤醒恩兔酱（创建 N2N 进程管理器）
    let mut process = N2NProcess::new();
    process.set_log_sender(log_tx);
    
    // 准备指示簿（创建配置管理器）
    let config_manager = ConfigManager::new().expect("无法创建配置管理器");

    tauri::Builder::default()
        .setup(|app| {
            // Windows 开机体检：缺 TAP 就先提示主人安装，避免后面连接时才摔跤
            #[cfg(target_os = "windows")]
            {
                if let Err(e) = windows_ready::ready_to_run(&app.handle()) {
                    log::error!("Windows Ready-to-Run 检查失败：{}", e);
                }
            }

            // 创建系统托盘
            tray::create_tray(&app.handle())?;
            
            log::info!("恩兔酱准备就绪，随时为主人服务！");
            Ok(())
        })
        .manage(AppState {
            process: Arc::new(Mutex::new(process)),
            config_manager: Arc::new(Mutex::new(config_manager)),
            log_rx: Arc::new(Mutex::new(Some(log_rx))),
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            connect,
            disconnect,
            disconnect_force,
            get_status,
            get_logs,
        ])
        .run(tauri::generate_context!())
        .expect("启动 Tauri 应用失败");
}
