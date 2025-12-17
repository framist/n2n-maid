# N2N UI

> 🚀 一个现代化、轻量级的 N2N VPN 图形界面客户端

基于 Rust 和 Tauri 2 构建，提供极简优雅的 N2N 连接体验。

[![Rust](https://img.shields.io/badge/Rust-1.92-orange)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/Tauri-2.0-blue)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-18-blue)](https://react.dev/)
[![License](https://img.shields.io/badge/License-MIT-green)](LICENSE)

## ✨ 功能特性

- ✅ **极简 UI 设计** - 类似 VPN 客户端的直观界面，非技术人员友好
- ✅ **轻量级** - 使用 Tauri 框架，安装包小于 10MB
- ✅ **跨平台** - 支持 Windows 和 Linux
- ✅ **系统托盘** - 后台运行，支持托盘图标和快捷菜单
- ✅ **实时日志** - 终端风格的可折叠日志查看器，支持彩色输出
- ✅ **网卡信息显示** - 连接成功后自动显示 IP、掩码和 MAC 地址
- ✅ **配置持久化** - 自动保存配置到本地 TOML 文件
- ✅ **国际化** - 支持中文和英文界面
- ✅ **主题切换** - 支持亮色、暗色和跟随系统三种模式
- ✅ **高级设置** - 自定义 edge 路径、TAP 设备、MTU 等参数

## 技术栈

- **后端**: Rust + Tauri 2.x
- **前端**: React 18 + TypeScript + TailwindCSS
- **构建工具**: Vite
- **N2N 集成**: 进程调用方式（松耦合）

## 开发环境设置

### 前置要求

- Node.js 18+ 和 npm
- Rust 1.70+
- N2N edge 可执行文件

### 安装依赖

```bash
# 安装前端依赖
npm install

# Rust 依赖会在构建时自动安装
```

### 开发模式运行

```bash
# 启动开发服务器
npm run tauri dev
```

### 构建生产版本

```bash
# 构建应用
npm run tauri build
```

生成的安装包位于 `src-tauri/target/release/bundle/` 目录。

## 项目结构

```
n2n-ui/
├── src/                    # 前端源代码
│   ├── components/         # React 组件
│   │   ├── LogViewer.tsx   # 日志查看器
│   │   └── Settings.tsx    # 设置面板
│   ├── App.tsx             # 主应用组件
│   ├── main.tsx            # 前端入口
│   ├── i18n.ts             # 国际化配置
│   ├── types.ts            # TypeScript 类型定义
│   └── styles.css          # 全局样式
├── src-tauri/              # Tauri/Rust 后端（需要创建）
│   └── src/
│       ├── main.rs         # 主程序入口
│       ├── config.rs       # 配置管理
│       ├── n2n_process.rs  # N2N 进程管理
│       └── tray.rs         # 系统托盘
├── bin/                    # N2N edge 二进制文件
│   └── README.md
├── icons/                  # 应用图标
│   └── README.md
├── tauri.conf.json         # Tauri 配置
├── package.json            # 前端依赖配置
├── Cargo.toml              # Rust 依赖配置
└── README.md               # 本文件
```

## 配置说明

### N2N Edge 二进制文件

将 N2N edge 可执行文件放置在 `bin/` 目录：

- Windows: `bin/edge.exe`
- Linux/macOS: `bin/edge`

从 [N2N 官方仓库](https://github.com/ntop/n2n/releases) 下载最新版本。

### 应用图标

图标文件需要放置在 `icons/` 目录。可以使用 Tauri 图标生成工具：

```bash
npm install -g @tauri-apps/cli
tauri icon path/to/your-icon.png
```

### 配置文件位置

配置文件自动保存在：

- Windows: `%APPDATA%/n2n-ui/config.toml`
- Linux: `~/.config/n2n-ui/config.toml`
- macOS: `~/Library/Application Support/n2n-ui/config.toml`

## 使用说明

### 基本使用

1. 启动应用
2. 点击「设置」按钮
3. 填写必填项：
   - Supernode 地址（格式：`host:port`，如 `vpn.example.com:7777`）
   - 社区名称
   - 用户名
   - （可选）加密密钥
4. 选择 IP 模式（DHCP 或静态 IP）
5. 点击「保存」
6. 返回主界面，点击「连接」按钮

### 高级功能

在设置界面展开「高级设置」可以配置：

- **Edge 路径**: 自定义 N2N edge 可执行文件路径
- **TAP 设备**: 指定虚拟网卡名称
- **MTU**: 设置最大传输单元（默认 1290）
- **额外参数**: 添加其他 N2N 命令行参数

### 日志查看

底部固定显示日志终端，实时显示 N2N edge 进程的输出。可以点击标题栏折叠/展开。日志支持彩色输出，方便调试。

### 主题切换

点击顶部主题图标（☀️/🌙/💻）可以在亮色、暗色和跟随系统三种模式间切换。

### 系统托盘

应用会在系统托盘显示图标，右键菜单提供：

- 显示主窗口
- 连接/断开
- 退出应用

## Linux 注意事项

N2N 通常需要 root 权限来创建 TAP 设备。在 Linux 上有两种方式运行：

### 方法 1: 使用 sudo（推荐）

```bash
sudo ./n2n-ui
```

### 方法 2: 设置 CAP_NET_ADMIN 权限

```bash
sudo setcap cap_net_admin+ep ./bin/edge
```

然后普通用户运行应用。

## 故障排查

### 连接失败

1. 检查 Supernode 地址是否正确
2. 检查防火墙是否阻止了 N2N 端口
3. 查看日志输出，寻找错误信息
4. 确认 edge 二进制文件有执行权限

### 找不到 edge 可执行文件

- 确认 `bin/` 目录下有正确的 edge 文件
- 或在「高级设置」中指定完整路径

### Linux 权限错误

```
ERROR: unable to open tap device
```

需要以 root 权限运行，或设置 CAP_NET_ADMIN。

## 开发路线图

- [x] 阶段一：MVP（最小可行性产品）
  - [x] Tauri + React + Tailwind 框架
  - [x] 基本 UI 和连接功能
  - [x] 日志查看器
- [x] 阶段二：用户体验优化
  - [x] 配置持久化
  - [x] 系统托盘支持
  - [x] 国际化（中英文）
  - [x] 自动重连机制
- [ ] 阶段三：Windows 深度适配
  - [ ] TAP 驱动检测
  - [ ] 安装包构建（MSI/EXE）
- [ ] 阶段四：高级功能
  - [ ] Supernode 订阅机制
  - [ ] 流量统计图表
  - [ ] Peers 信息显示

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License

## 相关链接

- [N2N 官方仓库](https://github.com/ntop/n2n)
- [Tauri 官方文档](https://tauri.app/)
