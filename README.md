
<div align="center">
  <img src="src-tauri/icons/icon-source.png" alt="N2N Maid Icon" width="128" height="128"/>
  <h1>N2N 女仆 · 恩兔酱 | N2N Maid · N-Too</h1>
  <p>一个开源跨平台并且可爱的 N2N 图形界面客户端</p>
  <p><i>🧹主人，您的专属网络通道已打扫完毕！✨</i></p>
</div>

<p align="center">
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/Rust-1.92-orange" alt="Rust"></a>
  <a href="https://tauri.app/"><img src="https://img.shields.io/badge/Tauri-2.0-blue" alt="Tauri"></a>
  <a href="https://react.dev/"><img src="https://img.shields.io/badge/React-18-blue" alt="React"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-green" alt="License"></a>
</p>

恩兔酱是一个可爱又实用的 [N2N](https://github.com/ntop/n2n) VPN 图形界面客户端，基于 Rust 和 Tauri 2 构建。
让复杂的网络配置变得简单有趣，就像有位贴心的女仆帮您打理一切～ 详细介绍[在这里](./intro.md)呢！


## ✨ 恩兔酱的特长

- 🎀 **轻盈灵活** - 使用 Tauri 框架，可执行文件不到 10MB，不占主人的空间
- 🌍 **跨平台待命** - Windows 和 Linux 都能为主人服务
- 💾 **记忆力超好** - 主人的设置会自动保存，下次不用重新配置
- 🗣️ **Let's speak English!** - Support English, adapt to the needs of different masters

![demo](http://framist-bucket-openread.oss-cn-shanghai.aliyuncs.com/img/2025/12/22/20251222220916.png)

## 技术栈

- **后端**: Rust + Tauri 2.x
- **前端**: React 18 + TypeScript + TailwindCSS
- **构建工具**: Vite
- **N2N 集成**: 进程调用方式
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

### CI/CD 发布（不内置 edge）

GitHub Actions 会在推送 tag 时触发构建（仅 Windows + Linux），发布产物文件名为 `"[name]_[version]_[platform]_[arch]_[bundle]_lite[ext]"`，lite 用来标识**不包含** `bin/edge`。
运行时请在设置里填写 `edge_path`，或提前把 `edge` 放进系统 PATH。

```bash
git tag v0.1.0
git push origin v0.1.0
```

## 项目结构

```
n2n-maid/
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
│   └── icons/              # 应用图标（由 `tauri icon` 生成）
├── bin/                    # N2N edge 二进制文件
│   └── README.md
├── src-tauri/tauri.conf.json # Tauri 配置
├── package.json            # 前端依赖配置
├── Cargo.toml              # Rust 依赖配置
└── README.md               # 本文件
```

## 配置说明

### N2N Edge 二进制文件

将 N2N edge 可执行文件放置在 `bin/` 目录：

- Windows: `bin/edge.exe`
- Linux/macOS: `bin/edge`

从 [N2N binaries](https://github.com/lucktu/n2n) 下载最新版本。或从 [N2N 官方仓库](https://github.com/ntop/n2n/releases) 自行编译。

### Windows 准备事项（TAP + 管理员权限）

- **TAP-Windows 适配器**：Windows 上创建虚拟网卡通常需要 TAP 驱动。恩兔酱启动时会检查是否存在 TAP-Windows（常见为 `tap0901`），缺失则弹窗提示安装/下载。
- **打包内置安装包**：把 `tap-windows.exe` 或 `tap-windows.msi` 放进 `bin/`，构建安装包时会随应用一起打包，恩兔就能直接帮主人拉起安装程序。
- **UAC（管理员权限）**：生产构建已设置 `requireAdministrator`，每次启动都会弹 UAC，这是 Windows 上最稳定的“开机就能创建网卡”的方案。

### 应用图标

应用图标使用 `tauri icon` 生成的产物，默认输出在 `src-tauri/icons/`。
如果要重新生成，建议把源图标放在 `src-tauri/icons/icon-source.png`，然后运行：

```bash
npm install -g @tauri-apps/cli
tauri icon src-tauri/icons/icon-source.png
```

### 配置文件位置

配置文件自动保存在：

- Windows: `%APPDATA%/n2n-maid/config.toml`
- Linux: `~/.config/n2n-maid/config.toml`

## 使用说明

### 基本使用

1. 启动应用，恩兔酱会立刻出现在主人面前
2. 点击「📋 服务准备」按钮
3. 填写工作清单（必填项）：
   - 🏢 总部地址（格式：`host:port`，如 `vpn.example.com:7777`）
   - 🔑 工作暗号
   - 👤 我的工号
   - 🔐 保密密语（可选，但恩兔建议设置哦）
4. 选择地址分配方式（自动分配或手动指定）
5. 点击「确认」，恩兔会记住主人的指示
6. 返回主界面，开始！

### 系统托盘

恩兔会在系统托盘静静守候，右键菜单提供：

- 召唤主窗口
- 开始打扫/休息一下
- 让恩兔下班

## Linux 注意事项

N2N 通常需要 root 权限来创建 TAP 设备。在 Linux 上有两种方式运行：

### 方法 1: 使用 sudo（推荐）

```bash
sudo ./n2n-maid
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
- [x] 阶段三：Windows 深度适配
  - [x] TAP 驱动检测（启动时提示并引导安装）
  - [x] UAC 权限（requireAdministrator）
  - [x] 安装包构建（MSI/EXE）
- [ ] 阶段四：高级功能
  - [ ] 多实例支持
  - [ ] Supernode 订阅机制
  - [ ] 流量统计图表
  - [x] Peers 信息显示
  - [ ] 托盘等系统有关功能优化

## 贡献

欢迎提交 Issue 和 Pull Request！
