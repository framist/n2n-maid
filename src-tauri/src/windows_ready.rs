//! Windows Ready-to-Run 小管家 🧹
//!
//! 这位小管家专门负责 Windows 平台最容易“绊主人一跤”的两件事：
//! 1) 检查 TAP-Windows 适配器是否已安装
//! 2) 如果缺失，弹窗提示并尽量引导安装/下载

use anyhow::{Context, Result};
use std::path::PathBuf;
use std::process::Command;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;
use windows_sys::Win32::UI::Shell::ShellExecuteW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    MessageBoxW, IDYES, MB_DEFBUTTON1, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK, SW_SHOWNORMAL,
    MB_SETFOREGROUND, MB_SYSTEMMODAL, MB_TOPMOST, MB_YESNO,
};

/// Windows 启动前的“地毯式检查”（Ready to Run）
pub fn ready_to_run(app: &AppHandle) -> Result<()> {
    // 1) TAP 驱动检测
    if is_tap_windows_installed()? {
        log::info!("Windows 检测通过：已找到 TAP-Windows 适配器，恩兔可以开始打扫通道啦");
        return Ok(());
    }

    log::warn!("Windows 检测到缺少 TAP-Windows 适配器，通道可能无法创建");

    let title = "N2N Maid · 恩兔酱";
    let msg = concat!(
        "😢 呜呜，恩兔发现主人电脑里还没有安装 TAP-Windows 适配器。\r\n",
        "\r\n",
        "没有它的话，N2N 很可能没法创建虚拟网卡（也就打扫不出通道啦）。\r\n",
        "\r\n",
        "要不要让恩兔现在带主人去安装呢？"
    );

    let install_now = message_box_yes_no(title, msg, MB_ICONWARNING);
    if !install_now {
        message_box_ok(
            title,
            "好的主人～那恩兔先继续待命。\r\n需要连接时记得先装好 TAP-Windows 哦～",
            MB_ICONINFORMATION,
        );
        return Ok(());
    }

    // 2) 尝试运行打包好的安装程序（如果主人把它放进 bin/，它会一起被打包进资源目录）
    if let Some(installer) = find_tap_installer(app) {
        log::info!("准备启动 TAP 安装程序：{}", installer.display());
        launch_installer(&installer)?;

        message_box_ok(
            title,
            "恩兔已经把安装程序端上来了！\r\n安装完成后，请重新启动恩兔酱再来打扫通道～",
            MB_ICONINFORMATION,
        );

        // 交给安装程序接管现场，恩兔先下班，避免安装过程被占用/干扰
        std::process::exit(0);
    }

    // 3) 找不到安装包：引导主人去下载
    let download_url = "https://openvpn.net/community-downloads/";
    let msg = format!(
        "😢 呜呜，恩兔没在随身行李里找到 TAP 安装包。\r\n\r\n\
请主人先下载并安装 TAP-Windows（OpenVPN TAP Driver），再回来召唤恩兔继续打扫。\r\n\r\n\
要现在打开下载页面吗？\r\n{}\r\n",
        download_url
    );

    let open_now = message_box_yes_no(title, &msg, MB_ICONWARNING);
    if open_now {
        open_url(download_url).ok();
    }

    Ok(())
}

fn find_tap_installer(app: &AppHandle) -> Option<PathBuf> {
    // 生产包：resources 根目录
    for name in ["tap-windows.exe", "tap-windows.msi"] {
        if let Ok(p) = app.path().resolve(name, BaseDirectory::Resource) {
            if p.exists() {
                return Some(p);
            }
        }
    }
    // 生产包：如果资源被保留在 bin/ 子目录里，也别漏掉
    for name in ["tap-windows.exe", "tap-windows.msi"] {
        let candidate = format!("bin/{name}");
        if let Ok(p) = app.path().resolve(candidate, BaseDirectory::Resource) {
            if p.exists() {
                return Some(p);
            }
        }
    }

    // 开发模式：工作目录下的 bin/
    for name in ["tap-windows.exe", "tap-windows.msi"] {
        let dev_guess = PathBuf::from("bin").join(name);
        if dev_guess.exists() {
            return Some(dev_guess);
        }
    }

    None
}

fn launch_installer(installer: &PathBuf) -> Result<()> {
    let is_msi = installer
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| e.eq_ignore_ascii_case("msi"));

    if is_msi {
        Command::new("msiexec")
            .arg("/i")
            .arg(installer)
            .spawn()
            .with_context(|| format!("启动 MSI 安装程序失败：{}", installer.display()))?;
        return Ok(());
    }

    Command::new(installer)
        .spawn()
        .with_context(|| format!("启动安装程序失败：{}", installer.display()))?;
    Ok(())
}

/// 检查系统里是否已安装 TAP-Windows（常见 ComponentId 为 tap0901）
fn is_tap_windows_installed() -> Result<bool> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    // 方式 1：检查经典服务名（OpenVPN TAP 9）
    for svc in ["tap0901", "tap0901t", "tap0901e"] {
        let key = format!(r"SYSTEM\CurrentControlSet\Services\{svc}");
        if hklm.open_subkey(key).is_ok() {
            return Ok(true);
        }
    }

    // 方式 2：枚举网卡 Class，寻找 ComponentId=tap0901
    let class_path = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e972-e325-11ce-bfc1-08002be10318}";
    let class = match hklm.open_subkey(class_path) {
        Ok(k) => k,
        Err(_) => return Ok(false),
    };

    // 子项通常是 0000, 0001...（理论上不会太多，恩兔扫一遍就好）
    for i in 0..=256u32 {
        let sub = format!("{i:04}");
        let sk = match class.open_subkey(&sub) {
            Ok(k) => k,
            Err(_) => continue,
        };

        let component_id: Result<String, _> = sk.get_value("ComponentId");
        if let Ok(component_id) = component_id {
            if component_id.eq_ignore_ascii_case("tap0901") {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn message_box_yes_no(title: &str, text: &str, icon: u32) -> bool {
    let title = to_wide(title);
    let text = to_wide(text);
    let flags = MB_YESNO
        | MB_DEFBUTTON1
        | MB_SETFOREGROUND
        | MB_TOPMOST
        | MB_SYSTEMMODAL
        | icon;
    unsafe { MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), flags) == IDYES }
}

fn message_box_ok(title: &str, text: &str, icon: u32) {
    let title = to_wide(title);
    let text = to_wide(text);
    let flags = MB_OK | MB_SETFOREGROUND | MB_TOPMOST | MB_SYSTEMMODAL | icon;
    unsafe {
        MessageBoxW(std::ptr::null_mut(), text.as_ptr(), title.as_ptr(), flags);
    }
}

fn open_url(url: &str) -> Result<()> {
    let op = to_wide("open");
    let url = to_wide(url);
    let ret = unsafe {
        ShellExecuteW(
            std::ptr::null_mut(),
            op.as_ptr(),
            url.as_ptr(),
            std::ptr::null(),
            std::ptr::null(),
            SW_SHOWNORMAL,
        )
    };
    // 按 Win32 约定：返回值 > 32 表示成功
    if (ret as isize) <= 32 {
        anyhow::bail!("打开下载页面失败（ShellExecute 返回值：{}）", ret as isize);
    }
    Ok(())
}

fn to_wide(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(s).encode_wide().chain(std::iter::once(0)).collect()
}
