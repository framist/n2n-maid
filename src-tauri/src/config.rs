/// N2N 配置管理模块
/// 负责读取、写入和管理 N2N 连接配置
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// N2N 连接配置结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N2NConfig {
    /// Supernode 地址（格式：host:port）
    pub supernode: String,
    /// 社区名称
    pub community: String,
    /// 用户名（edge 名称）
    pub username: String,
    /// 加密密钥
    pub encryption_key: String,
    /// IP 地址模式（"dhcp" 或静态 IP）
    pub ip_mode: String,
    /// 静态 IP 地址（仅当 ip_mode 为静态时使用）
    pub static_ip: Option<String>,
    /// 自定义参数
    pub extra_args: Option<String>,
    /// edge 二进制文件路径
    pub edge_path: Option<String>,
    /// TAP 网卡名称
    pub tap_device: Option<String>,
    /// MTU 设置
    pub mtu: Option<u16>,
    /// 主题设置（light/dark/system）
    pub theme: Option<String>,
}

impl Default for N2NConfig {
    fn default() -> Self {
        Self {
            supernode: String::new(),
            community: String::new(),
            username: String::new(),
            encryption_key: String::new(),
            ip_mode: "dhcp".to_string(),
            static_ip: None,
            extra_args: None,
            edge_path: None,
            tap_device: None,
            mtu: Some(1290),
            theme: Some("system".to_string()),
        }
    }
}

/// 配置管理器
pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    /// 创建新的配置管理器实例
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .context("无法获取配置目录")?
            .join("n2n-ui");
        
        // 确保配置目录存在
        fs::create_dir_all(&config_dir)?;
        
        let config_path = config_dir.join("config.toml");
        
        Ok(Self { config_path })
    }

    /// 加载配置文件
    pub fn load(&self) -> Result<N2NConfig> {
        if !self.config_path.exists() {
            return Ok(N2NConfig::default());
        }

        let content = fs::read_to_string(&self.config_path)
            .context("读取配置文件失败")?;
        
        let config: N2NConfig = toml::from_str(&content)
            .context("解析配置文件失败")?;
        
        Ok(config)
    }

    /// 保存配置文件
    pub fn save(&self, config: &N2NConfig) -> Result<()> {
        let content = toml::to_string_pretty(config)
            .context("序列化配置失败")?;
        
        fs::write(&self.config_path, content)
            .context("写入配置文件失败")?;
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = N2NConfig::default();
        assert_eq!(config.ip_mode, "dhcp");
        assert_eq!(config.mtu, Some(1290));
    }

    #[test]
    fn test_config_serialization() {
        let config = N2NConfig::default();
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: N2NConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config.ip_mode, deserialized.ip_mode);
    }
}
