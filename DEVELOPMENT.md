# N2N UI 开发指南

## 快速开始

### 1. 安装依赖

```bash
# 安装 Node.js 依赖
npm install

# Rust 依赖会在首次构建时自动下载
```

### 2. 准备 N2N Edge 二进制文件

从 [N2N Releases](https://github.com/ntop/n2n/releases) 下载适合你系统的 edge 可执行文件，放置在 `bin/` 目录：

```bash
# Linux 示例
wget https://github.com/ntop/n2n/releases/download/3.0/n2n-3.0-Linux.tar.gz
tar -xzf n2n-3.0-Linux.tar.gz
cp n2n-3.0-Linux/edge bin/
chmod +x bin/edge
```

或者从源码编译：

```bash
git clone https://github.com/ntop/n2n.git
cd n2n
./autogen.sh
./configure
make
cp edge ../n2n-ui/bin/
```

### 3. 运行开发模式

```bash
npm run tauri dev
```

这会启动：
1. Vite 开发服务器（前端热重载）
2. Tauri 应用窗口

### 4. 构建生产版本

```bash
npm run tauri build
```

生成的文件：
- **Linux**: `src-tauri/target/release/bundle/deb/n2n-ui_0.1.0_amd64.deb`
- **Windows**: `src-tauri/target/release/bundle/msi/n2n-ui_0.1.0_x64.msi`

## 项目架构

### 前端（React + TypeScript）

```
src/
├── App.tsx              # 主应用组件，管理状态和路由
├── main.tsx             # React 入口文件
├── i18n.ts              # i18next 国际化配置
├── types.ts             # TypeScript 类型定义
├── styles.css           # TailwindCSS 全局样式
└── components/
    ├── LogViewer.tsx    # 日志查看器组件
    └── Settings.tsx     # 设置面板组件
```

**关键技术**：
- React Hooks（useState, useEffect）
- Tauri IPC 通信（@tauri-apps/api）
- TailwindCSS 样式
- react-i18next 国际化

### 后端（Rust + Tauri）

```
src/
├── main.rs              # Tauri 主程序，定义 commands
├── config.rs            # 配置文件读写（TOML）
├── n2n_process.rs       # N2N 进程管理（启动/停止/监控）
└── tray.rs              # 系统托盘逻辑
```

**关键技术**：
- Tauri Commands（IPC）
- tokio 异步运行时
- serde 序列化/反序列化
- std::process 进程管理

## 核心功能实现

### 1. 配置管理

**文件**: `src/config.rs`

配置自动保存到系统配置目录：
- Linux: `~/.config/n2n-ui/config.toml`
- Windows: `%APPDATA%/n2n-ui/config.toml`

```rust
let manager = ConfigManager::new()?;
let config = manager.load()?;
manager.save(&config)?;
```

### 2. N2N 进程管理

**文件**: `src/n2n_process.rs`

核心流程：
1. 根据配置构建命令行参数
2. 使用 `std::process::Command` 启动 edge
3. 捕获 stdout/stderr 到日志通道
4. 监控进程状态，支持自动重连

```rust
let process = N2NProcess::new();
process.start(&config)?;  // 启动
process.stop()?;          // 停止
process.status();         // 获取状态
```

### 3. 前后端通信

**前端调用后端**:

```typescript
import { invoke } from '@tauri-apps/api/core';

// 获取配置
const config = await invoke<N2NConfig>('get_config');

// 连接
await invoke('connect', { config });

// 断开
await invoke('disconnect');
```

**后端定义 Command**:

```rust
#[tauri::command]
async fn get_config(state: State<'_, AppState>) -> Result<N2NConfig, String> {
    // 实现
}
```

### 4. 系统托盘

**文件**: `src/tray.rs`

- 创建托盘图标和菜单
- 处理点击事件
- 根据连接状态更新菜单项

### 5. 日志系统

日志通过 `tokio::sync::mpsc` 通道从 Rust 传递到前端：

1. N2N 进程输出 → Rust 读取线程
2. Rust 读取线程 → mpsc 通道
3. 前端轮询 `get_logs` command
4. 显示在 LogViewer 组件

## 调试技巧

### 查看 Rust 日志

开发模式下，Rust 日志会输出到终端：

```bash
RUST_LOG=debug npm run tauri dev
```

### 查看前端控制台

在 Tauri 窗口中按 F12 打开开发者工具。

### 手动测试 N2N

在 `bin/` 目录直接运行 edge：

```bash
./bin/edge -c test -l supernode.example.com:7777 -a dhcp:0.0.0.0
```

### 检查配置文件

```bash
# Linux
cat ~/.config/n2n-ui/config.toml

# Windows
type %APPDATA%\n2n-ui\config.toml
```

## 常见问题

### Q: 前端修改后没有生效？

A: Vite 会热重载，但如果不生效，尝试：
1. 刷新浏览器（Ctrl+R）
2. 重启开发服务器

### Q: Rust 修改后没有生效？

A: Tauri 需要完全重新编译，Ctrl+C 停止后重新运行 `npm run tauri dev`。

### Q: 找不到 edge 可执行文件？

A: 检查：
1. `bin/edge` 是否存在
2. 是否有执行权限（Linux: `chmod +x bin/edge`）
3. 在设置中指定完整路径

### Q: Linux 上提示权限错误？

A: N2N 需要 root 权限创建 TAP 设备：

```bash
sudo npm run tauri dev
# 或
sudo setcap cap_net_admin+ep bin/edge
```

## 代码风格

### Rust

- 使用 `rustfmt` 格式化代码
- 遵循 Rust 命名约定（snake_case）
- 添加中文文档注释

### TypeScript/React

- 使用 ESLint 和 Prettier
- 函数组件 + Hooks
- Props 使用 interface 定义
- 中文注释

## 发布流程

1. 更新版本号（package.json 和 Cargo.toml）
2. 运行测试：`cargo test`
3. 构建发布版本：`npm run tauri build`
4. 测试生成的安装包
5. 创建 Git tag 并推送
6. 上传到 GitHub Releases

## 贡献指南

1. Fork 项目
2. 创建功能分支：`git checkout -b feature/my-feature`
3. 提交更改：`git commit -am 'Add some feature'`
4. 推送分支：`git push origin feature/my-feature`
5. 创建 Pull Request

## 参考资源

- [Tauri 官方文档](https://tauri.app/)
- [N2N 文档](https://github.com/ntop/n2n/tree/dev/doc)
- [React 文档](https://react.dev/)
- [TailwindCSS 文档](https://tailwindcss.com/)
