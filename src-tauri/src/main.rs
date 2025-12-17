// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;
mod n2n_process;
mod tray;

use config::{ConfigManager, N2NConfig};
use n2n_process::{ConnectionStatus, N2NProcess};
use std::sync::{Arc, Mutex};
use tauri::State;
use tokio::sync::mpsc;

/// 应用程序状态
struct AppState {
    /// N2N 进程管理器
    process: Arc<Mutex<N2NProcess>>,
    /// 配置管理器
    config_manager: Arc<Mutex<ConfigManager>>,
    /// 日志接收器
    log_rx: Arc<Mutex<Option<mpsc::UnboundedReceiver<String>>>>,
}

/// 获取当前配置
#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<N2NConfig, String> {
    let manager = state.config_manager.lock().unwrap();
    manager.load().map_err(|e| e.to_string())
}

/// 保存配置
#[tauri::command]
async fn save_config(config: N2NConfig, state: State<'_, AppState>) -> Result<(), String> {
    let manager = state.config_manager.lock().unwrap();
    manager.save(&config).map_err(|e| e.to_string())
}

/// 启动 N2N 连接
#[tauri::command]
async fn connect(config: N2NConfig, state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let process = state.process.lock().unwrap();
    
    // 先保存配置
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

/// 断开 N2N 连接
#[tauri::command]
async fn disconnect(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let process = state.process.lock().unwrap();
    process.stop().map_err(|e| e.to_string())?;
    
    // 更新托盘状态
    let status = process.status();
    let _ = tray::update_tray_menu(&app, &status);
    
    Ok(())
}

/// 强制断开 N2N 连接（用于优雅退出卡住时）
#[tauri::command]
async fn disconnect_force(state: State<'_, AppState>, app: tauri::AppHandle) -> Result<(), String> {
    let process = state.process.lock().unwrap();
    process.stop_force().map_err(|e| e.to_string())?;

    // 更新托盘状态
    let status = process.status();
    let _ = tray::update_tray_menu(&app, &status);

    Ok(())
}

/// 获取连接状态
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

/// 获取日志
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
    // 初始化日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // 创建日志通道
    let (log_tx, log_rx) = mpsc::unbounded_channel();
    
    // 创建 N2N 进程管理器
    let mut process = N2NProcess::new();
    process.set_log_sender(log_tx);
    
    // 创建配置管理器
    let config_manager = ConfigManager::new().expect("无法创建配置管理器");

    tauri::Builder::default()
        .setup(|app| {
            // 创建系统托盘
            tray::create_tray(&app.handle())?;
            
            log::info!("N2N UI 启动成功");
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

