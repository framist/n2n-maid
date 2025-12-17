# N2N GUI 应用程序设计方案

N2N 是一款强大的 P2P VPN 工具，但官方只提供了命令行界面（CLI），这对普通用户门槛太高。使用 Rust 开发一个现代化、跨平台的 GUI 包装器（Wrapper）非常有价值。

核心：

基于 rust 实现的 n2n ui 应用程序，与 https://github.com/ntop/n2n 松耦合
轻量的，极简的，现代的，易于在 windows ready to run 运行的，非技术人员友好的（同时也为技术人员提供足够的 debug 功能），理想情况就是除第一次配置外，可以双击直接运行，无需复杂配置；i18n 支持 linux 支持
为未来考虑 supernode 订阅机制


### 1. 技术栈选型

为了兼顾“现代 UI”、“轻量级”和“Rust 后端能力”，推荐使用 **Tauri** 框架。

*   **GUI 框架：Tauri v2**
    *   **理由**：Tauri 使用系统原生的 WebView（Windows 上是 WebView2，Linux 上是 WebKit），相比 Electron 极度轻量（安装包通常 < 10MB）。
    *   **前端**：React + TailwindCSS 或 SolidJS + TailwindCSS。这样可以轻松实现“极简”、“现代化”的 UI 设计。
    *   **后端**：Rust。负责系统调用、进程管理、托盘图标、配置持久化。
*   **N2N 集成方式：松耦合 (Process Spawning)**
    *   不直接调用 C 库 (FFI)，而是将编译好的 `edge` (N2N 客户端) 二进制文件打包在应用中，或者允许用户指定路径。
    *   Rust 通过 `std::process::Command` 启动子进程，并通过 `stdio` 捕获日志，或通过 UDP 管理端口与 N2N 通信。
*   **数据存储**：
    *   纯文本 `TOML` 文件，存储在用户配置目录中。

### 2. 核心架构设计

#### 2.1 极简 UI 设计 (非技术人员视角)
*   **主界面**：
    *   类似 VPN 客户端的极简设计。
    *   不要使用过多的圆角和阴影，保持扁平化风格。
    *   下拉菜单选择“服务器/环境”（基于订阅或预设）。
    *   一个 连接/断开 按钮
    *   当前显示的虚拟 IP 地址。主要状态（灰色：断开，黄：连接中，绿：已连接，红：错误）。
    *   连接信息/编辑配置/全局设置 按钮，进入二级界面。
*   **系统托盘 (Tray)**：
    *   双击运行后，主要在后台运行。
    *   右键菜单：连接、断开、退出。
    *   实现“开机自启”和“静默启动”。

#### 2.2 调试与高级功能 (技术人员视角)
*   **日志面板**：一个可折叠的终端风格窗口，实时显示 `edge` 进程的 stdout/stderr 输出。
*   **高级设置**：
    *   自定义参数输入框 (Extra Args)。
    *   手动指定 TAP 网卡名称。
    *   MTU 设置、路由表转发设置。
    *   切换 `edge` 二进制文件路径（方便测试不同版本 N2N）。

#### 2.3 交互逻辑
Rust 主进程充当“守护进程”：
1.  读取配置 -> 2. 拼接参数 -> 3. 启动 `edge.exe` -> 4. 监控进程存活 -> 5. 解析输出更新 UI 状态。

---

### 3. Windows 特性适配 (Ready to Run)

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

---

### 5. 实施路线图 (Roadmap)

#### 阶段一：MVP (最小可行性产品)
*   **目标**：Windows 下能跑通，能连接，能断开。
*   **功能**：
    *   搭建 Tauri + React + Tailwind 框架。
    *   UI 包含：Supernode 地址、小组名、用户名、密码、IP 模式。
    *   Rust 后端：打包 `edge.exe`，实现启动/停止逻辑。
    *   日志窗口：实时显示 N2N 输出。

#### 阶段二：用户体验优化 & 持久化
*   **目标**：非技术人员可以轻松使用，“双击即用”。
*   **功能**：
    *   配置保存到本地磁盘。
    *   系统托盘图标支持（最小化到托盘）。
    *   i18n 国际化架构搭建（通过 `rust-i18n` 或前端 i18n 库），支持中/英。
    *   自动重连机制（如果 `edge` 意外退出）。
    *   N2N 管理端口集成（通过 UDP 端口 5645 获取详细的 Peers 信息和流量统计，并在 UI 绘制图表）。

#### 阶段三：Windows 深度适配
*   **目标**：解决驱动和权限问题。
*   **功能**：
    *   检测 TAP 适配器。
    *   添加 Windows 安装包构建脚本 (NSIS 或 WiX)，制作 `.msi` 或 `.exe` 安装程序。
    *   数字签名（如果预算允许，否则 Windows Defender 可能会报毒）。

#### 阶段四：高级功能 & Linux
*   **目标**：跨平台与订阅。
*   **功能**：
    *   实现 JSON 订阅源解析。
    *   Linux 适配（主要处理 `sudo` 提权问题，Linux 下通常使用 `pkexec` 或要求用户以 root 运行 GUI）。

---

### 6. 项目结构示例 (Rust/Tauri)

```text
n2n-ui/
├── src-tauri/
│   ├── src/
│   │   ├── main.rs         # 核心逻辑
│   │   ├── n2n_process.rs  # 封装 Command，处理子进程
│   │   ├── config.rs       # 配置读写
│   │   └── tray.rs         # 托盘逻辑
│   ├── bin/
│   │   └── edge.exe        # 嵌入的 N2N 二进制 (Windows)
│   ├── icons/
│   └── tauri.conf.json
├── src/                    # 前端代码 (React/Vue)
│   ├── components/
│   │   ├── BigSwitch.tsx   # 大按钮
│   │   ├── LogViewer.tsx   # 日志查看器
│   │   └── Settings.tsx
│   ├── locales/            # i18n JSON
│   └── App.tsx
└── README.md
```

### 7. 给开发者的建议

1.  **关于 N2N 版本**：N2N 的协议（v2, v2.5, v3）不兼容。建议默认内置最新的 Stable 版本（目前是 v3 系列），并在设置里允许用户替换二进制文件以兼容旧版。
2.  **网络接口冲突**：Windows 下如果存在多个 TAP 适配器，N2N 有时会选错。最好在设置里提供一个“网卡接口白名单”或自动检测 N2N 创建的接口 IP。
3.  **安全性**：不要在日志中明文打印密码（虽然 N2N 本身可能不会打印，但要注意 UI 层的保护）。

这个方案能够在保持极简外观的同时，利用 Rust 强大的系统编程能力解决底层的进程管理和跨平台问题。