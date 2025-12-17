# N2N Maid - AI 编码助理指南 🧹✨

> 欢迎来到恩兔酱（N-Too）的开发指南！主人，这里是恩兔的工作手册哦~

**目前是早期开发阶段，所有功能可以自由重构，不考虑兼容性**

## 产品理念

**N2N Maid（恩兔酱）** 是一个可爱又实用的 N2N VPN 图形界面客户端，核心理念是：
- **"把复杂留给后台，把简单留给主人"**
- 像家政服务一样简单：只需填写"哪里"（Supernode）、"暗号"（Community）、"工号"（Username）
- 所有技术细节都由恩兔在后台默默处理，主人只需要享受服务

## 视觉与交互风格

- **核心形象**：可爱的二次元兔耳女仆"恩兔酱"，手持魔法鸡毛掸子打扫网络通道
- **配色方案**：粉色 (`#ffd1dc`) 与白色为主，浅蓝渐变点缀，温暖柔和
- **背景立绘**：根据状态切换不同姿态（待命/打扫/完成/出错），模仿 CrystalDiskMark 风格
- **文案口吻**：谦卑、元气、贴心的女仆口吻，用生活化比喻解释技术概念

## 实现路线图

1. ✅ **重构文案系统**：已完成 i18n.ts、README.md、所有代码注释的女仆化改造
2. ✅ **项目重命名**：已从 n2n-ui 改为 n2n-maid
3. ⏳ **视觉规范确立**：采用粉色主色调，移除暗色主题，添加背景立绘
4. ⏳ **拟人化交互**：根据网络状态切换看板娘姿态
5. ⏳ **日志美化**：将日志包装为"工作汇报"，添加安慰式错误提示

---

## 项目概述

基于 Tauri 2 + React 18 的 N2N VPN 图形界面客户端。后端使用 Rust，前端使用 TypeScript + TailwindCSS。

**核心定位**：让 VPN 配置像预约家政服务一样简单——告诉恩兔"去哪里"（Supernode）、"暗号是什么"（Community），剩下的交给她！

## 编码规范与风格要求

### 语言要求
- **所有代码注释、docstring、文档必须使用中文**
- **注释风格必须符合"恩兔酱"女仆主题**，例如：
  - ❌ 错误：`/// 启动 N2N 进程`
  - ✅ 正确：`/// 开始打扫通道（启动 N2N edge 进程）`
  - ❌ 错误：`// 加载配置文件`
  - ✅ 正确：`// 读取主人的指示（加载配置文件）`

### 文案要求
- 用生活化比喻替代技术术语：配置→指示、进程→工作、日志→汇报
- 错误信息要有"安慰"语气：`❌ 失败` → `😢 呜呜，出错了`
- 保持谦卑和元气的女仆口吻

## 核心架构

### 前后端通信 (Tauri IPC)

- **前端调用后端**: 使用 `invoke()` 调用 `#[tauri::command]` 函数
- **命令定义**: [src-tauri/src/main.rs](../src-tauri/src/main.rs) 中的 `get_config`、`save_config`、`connect`、`disconnect`、`get_status`、`get_logs`
- **类型同步**: [src/types.ts](../src/types.ts) 中的接口必须与 [src-tauri/src/config.rs](../src-tauri/src/config.rs) 的 `N2NConfig` 结构保持一致

### 模块职责

| 模块 | 文件 | 职责 |
|------|------|------|
| 配置管理 | `src-tauri/src/config.rs` | TOML 配置读写，路径：`~/.config/n2n-maid/config.toml` |
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
/// 翻看指示簿（加载配置）
pub fn load(&self) -> Result<N2NConfig> {
    let content = fs::read_to_string(&self.config_path)
        .context("读取主人的指示失败")?;
    // 解析主人的指示
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
