# N2N UI - AI 编码助理指南

**目前是早期开发阶段，所有功能可以自由重构，不考虑兼容性**


## 项目概述

基于 Tauri 2 + React 18 的 N2N VPN 图形界面客户端。后端使用 Rust，前端使用 TypeScript + TailwindCSS。

## 语言规范

- **所有代码注释、docstring、文档必须使用中文**、

## 核心架构

### 前后端通信 (Tauri IPC)

- **前端调用后端**: 使用 `invoke()` 调用 `#[tauri::command]` 函数
- **命令定义**: [src-tauri/src/main.rs](../src-tauri/src/main.rs) 中的 `get_config`、`save_config`、`connect`、`disconnect`、`get_status`、`get_logs`
- **类型同步**: [src/types.ts](../src/types.ts) 中的接口必须与 [src-tauri/src/config.rs](../src-tauri/src/config.rs) 的 `N2NConfig` 结构保持一致

### 模块职责

| 模块 | 文件 | 职责 |
|------|------|------|
| 配置管理 | `src-tauri/src/config.rs` | TOML 配置读写，路径：`~/.config/n2n-ui/config.toml` |
| 进程管理 | `src-tauri/src/n2n_process.rs` | 启动/停止/监控 N2N edge 子进程 |
| 系统托盘 | `src-tauri/src/tray.rs` | 托盘图标、菜单、事件处理 |
| 国际化 | `src/i18n.ts` | 中英文翻译资源（内联定义） |

### N2N 集成方式

采用**进程调用松耦合**模式，通过 `std::process::Command` 启动 `edge` 二进制：
- 默认路径：`bin/edge`（Linux）或 `bin/edge.exe`（Windows）
- 可在设置中自定义 `edge_path`
- 通过 stdout/stderr 捕获日志，检测连接状态（如 `"Registered with"` 表示连接成功）

## 开发命令

```bash
# 安装依赖
npm install

# 开发模式（前端热重载 + Tauri 窗口）
npm run tauri dev

# 生产构建
npm run tauri build
# 输出: src-tauri/target/release/bundle/

```

## 代码规范

### Rust 代码

- 使用 `anyhow::Result` 处理错误，配合 `context()` 提供中文错误信息
- 异步运行时：tokio
- 日志：`log::info!()` / `log::warn!()` / `log::error!()`
- 状态管理：`Arc<Mutex<T>>` 模式

```rust
/// 示例：配置加载（带中文 docstring）
pub fn load(&self) -> Result<N2NConfig> {
    let content = fs::read_to_string(&self.config_path)
        .context("读取配置文件失败")?;
    // ...
}
```

### TypeScript/React 代码

- 组件位于 `src/components/`
- 使用 `useTranslation()` 实现 i18n
- 使用 `invoke<T>('command_name', { args })` 调用 Rust 命令
- 状态轮询：`setInterval(checkStatus, 2000)` 检查连接状态

## 添加新功能检查清单

1. **新配置项**: 同时更新 `config.rs` 的 `N2NConfig` 和 `types.ts` 的接口
2. **新 Tauri 命令**: 在 `main.rs` 中用 `#[tauri::command]` 标记，并注册到 `invoke_handler`
3. **新 UI 文案**: 在 `i18n.ts` 的 `zh` 和 `en` 资源中同时添加
4. **进程参数变更**: 修改 `n2n_process.rs` 的 `start()` 方法中的参数构建逻辑

## 测试建议

- 使用 `bin/edge-mock.sh` 模拟 edge 进程进行 UI 测试
- Linux 上 N2N 需要 root 权限才能创建 TAP 设备

## 对助理的要求

涉及网络下载时，`export https_proxy=http://127.0.0.1:7890;export http_proxy=http://127.0.0.1:7890;export all_proxy=socks5://127.0.0.1:7890;export TELOXIDE_PROXY=http://127.0.0.1:7890`
