# N2N Edge 二进制文件

请在此目录放置 N2N edge 可执行文件：

- Windows: edge.exe
- Linux: edge
- macOS: edge

从 https://github.com/ntop/n2n/releases 下载最新版本。

## Windows 额外准备：TAP-Windows 安装包（可选但强烈建议）

为了让恩兔酱在 Windows 上“开机就能跑”，建议主人把 TAP-Windows 安装程序也放进来：

- Windows: `tap-windows.exe` 或 `tap-windows.msi`

打包时 `src-tauri/tauri.conf.json` 会把 `bin/*` 一起带走；这样恩兔就能在启动时检测到缺失并直接端上安装程序。
