/// 构建脚本 - 用于 Tauri 应用构建配置
fn main() {
    // Windows 是“最爱挑剔的客人”，恩兔需要提前把清单（manifest）准备好：
    // - requireAdministrator：配置虚拟网卡/驱动更稳定（每次启动会弹 UAC）
    #[cfg(target_os = "windows")]
    {
        let windows = tauri_build::WindowsAttributes::new()
            .app_manifest(include_str!("windows/app.manifest"));
        let attrs = tauri_build::Attributes::new().windows_attributes(windows);
        tauri_build::try_build(attrs).expect("恩兔酱构建失败：Windows 资源准备不成功");
    }

    #[cfg(not(target_os = "windows"))]
    tauri_build::build()
}
