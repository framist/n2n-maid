/// 系统托盘管理模块
/// 负责创建和管理系统托盘图标及菜单
use tauri::{
    AppHandle, Emitter, Manager,
    tray::{TrayIconBuilder, TrayIconEvent},
    menu::{Menu, MenuItem},
};
use crate::n2n_process::ConnectionStatus;

/// 创建系统托盘
pub fn create_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
    let show_i = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
    let connect_i = MenuItem::with_id(app, "connect", "连接", true, None::<&str>)?;
    let disconnect_i = MenuItem::with_id(app, "disconnect", "断开", false, None::<&str>)?;
    
    let menu = Menu::with_items(app, &[&show_i, &connect_i, &disconnect_i, &quit_i])?;

    let _ = TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
            "connect" => {
                // 触发连接命令
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("tray-connect", ());
                }
            }
            "disconnect" => {
                // 触发断开命令
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.emit("tray-disconnect", ());
                }
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                ..
            } = event
            {
                // 左键点击显示窗口
                let app_handle = tray.app_handle();
                if let Some(window) = app_handle.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}

/// 更新托盘菜单状态
pub fn update_tray_menu(app: &AppHandle, _status: &ConnectionStatus) -> Result<(), Box<dyn std::error::Error>> {
    let tray = app.tray_by_id("main").unwrap();
    
    // Tauri 2 中托盘菜单更新的 API 可能需要重新构建菜单
    // 暂时简化实现，只更新提示文本
    
    // 更新托盘图标提示文本
    let tooltip = match _status {
        ConnectionStatus::Disconnected => "N2N UI - 已断开".to_string(),
        ConnectionStatus::Connecting => "N2N UI - 连接中...".to_string(),
        ConnectionStatus::Disconnecting => "N2N UI - 断开中...".to_string(),
        ConnectionStatus::Connected(_) => "N2N UI - 已连接".to_string(),
        ConnectionStatus::Error(msg) => format!("N2N UI - 错误: {}", msg),
    };

    tray.set_tooltip(Some(&tooltip))?;
    
    Ok(())
}
