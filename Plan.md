# N2N GUI 应用程序设计方案

N2N 是一款强大的 P2P VPN 工具，但官方只提供了命令行界面（CLI），这对普通用户门槛太高。使用 Rust 开发一个现代化、跨平台的 GUI 包装器（Wrapper）非常有价值。

核心：

基于 rust 实现的 n2n ui 应用程序，与 https://github.com/ntop/n2n 松耦合
轻量的，极简的，现代的，易于在 windows ready to run 运行的，非技术人员友好的（同时也为技术人员提供足够的 debug 功能），理想情况就是除第一次配置外，可以双击直接运行，无需复杂配置；i18n 支持 linux 支持
为未来考虑 supernode 订阅机制

### Windows 特性适配 (Ready to Run)

Windows 是痛点最多的平台，需要重点处理：

1.  **TAP 驱动检测**：
    *   程序启动时，Rust 检查系统中是否安装了 TAP-Windows 适配器。
    *   如果没有，弹窗提示并尝试调用打包好的 TAP 安装程序，或者引导用户下载。
2.  **UAC 权限 (Admin Rights)**：
    *   N2N 配置虚拟网卡通常需要管理员权限。
    *   **方案**：在 `Cargo.toml` 和 manifest 中设置 `requireAdministrator`。虽然每次启动会弹 UAC 框，但这是最稳定的方式。
3.  **防止黑框**：
    *   通过 Tauri 的 `Command` 启动 `edge.exe` 时设置 `CREATE_NO_WINDOW` 标志，确保后台静默运行。

---

### 4. Supernode 订阅机制 (未来扩展)

为了解决“小白用户不懂填 IP 和 Key”的问题，设计订阅机制：

*   **订阅格式**：定义一个标准的 JSON 格式，托管在 GitHub Gist 或私有服务器上。
    ```json
    {
      "version": 1,
      "nodes": [
        {
          "name": "公司内网",
          "supernode": "vpn.example.com:7777",
          "community": "my_company",
          "key_required": true, // 提示 UI 弹窗让用户输入密码，不硬编码在订阅里
          "auto_ip": true
        }
      ]
    }
    ```
*   **逻辑**：程序自动拉取列表，用户点击列表项即可连接。

### 给开发者的建议

1.  **关于 N2N 版本**：N2N 的协议（v2, v2.5, v3）不兼容。建议默认内置最新的 Stable 版本（目前是 v3 系列），并在设置里允许用户替换二进制文件以兼容旧版。
2.  **网络接口冲突**：Windows 下如果存在多个 TAP 适配器，N2N 有时会选错。最好在设置里提供一个“网卡接口白名单”或自动检测 N2N 创建的接口 IP。
3.  **安全性**：不要在日志中明文打印密码（虽然 N2N 本身可能不会打印，但要注意 UI 层的保护）。

这个方案能够在保持极简外观的同时，利用 Rust 强大的系统编程能力解决底层的进程管理和跨平台问题。