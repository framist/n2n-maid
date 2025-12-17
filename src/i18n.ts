import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

const resources = {
  zh: {
    translation: {
      "app_title": "N2N UI",
      "connect": "连接",
      "disconnect": "断开",
      "connecting": "连接中...",
      "connected": "已连接",
      "disconnected": "已断开",
      "error": "错误",
      "settings": "设置",
      "logs": "日志",
      "save": "保存",
      "cancel": "取消",
      
      // 配置相关
      "supernode": "Supernode 地址",
      "community": "社区名称",
      "username": "节点标识名称",
      "encryption_key": "加密密钥",
      "ip_mode": "IP 模式",
      "static_ip": "静态 IP",
      "dhcp": "DHCP",
      "static": "静态",
      "advanced_settings": "高级设置",
      "extra_args": "额外参数",
      "edge_path": "Edge 路径",
      "tap_device": "TAP 设备",
      "mtu": "MTU",
      
      // 状态信息
      "virtual_ip": "虚拟 IP",
      "status": "状态",
      
      // 提示信息
      "save_success": "保存成功",
      "save_failed": "保存失败",
      "connect_success": "连接成功",
      "connect_failed": "连接失败",
      "disconnect_success": "断开成功",
      "disconnect_failed": "断开失败",
      
      // 日志相关
      "show_logs": "显示日志",
      "hide_logs": "隐藏日志",
      "clear_logs": "清空日志",
      "clear": "清空",
      "no_logs": "暂无日志",
      "log_lines": "行",
      
      // 网卡信息
      "network_info": "网卡信息",
      "ip": "IP 地址",
      "mask": "子网掩码",
      "mac": "MAC 地址",
      
      // 参数说明
      "supernode_desc": "Supernode 服务器地址，格式：host:port（如 vpn.example.com:7777）",
      "community_desc": "VPN 社区名称，同一社区的节点可以互相通信",
      "username_desc": "可选。用于在管理界面中区分不同设备；留空将自动使用本机主机名",
      "encryption_key_desc": "AES 加密密钥（可选），留空则不加密。建议使用强密码保护数据传输",
      "ip_mode_desc": "IP 地址分配方式：DHCP 从 supernode 自动获取，静态模式需手动指定",
      "static_ip_desc": "静态 IP 地址，格式：10.0.0.2 或 10.0.0.2/24（CIDR 表示法）",
      "extra_args_desc": "额外的 edge 命令行参数，如：-v（详细日志）、-r（启用包转发）、-E（接受组播）",
      "edge_path_desc": "edge 可执行文件路径，留空则使用默认路径（bin/edge）",
      "tap_device_desc": "TAP 虚拟网卡名称，留空则自动分配（如 edge0）",
      "mtu_desc": "最大传输单元，默认 1290 字节。较小的 MTU 可减少分片但会降低效率",
      
      // 错误提示
      "error_mac_in_use": "MAC 地址已被占用，可能其他设备使用了相同的配置或 supernode 尚未释放",
      "error_ip_in_use": "IP 地址已被占用，请检查网络中是否有其他设备使用相同 IP",
      "error_tap_create_failed": "创建 TAP 设备失败，请确保有足够的系统权限",
      "error_supernode_unreachable": "无法连接到 supernode，请检查地址和网络连接",
      "error_auth_failed": "认证失败，请检查社区名称和加密密钥是否正确",
      "error_permission_denied": "权限不足，Linux 下需要授予 edge 必要的 capabilities",
      
      // 主题相关
      "theme": "主题",
      "theme_light": "亮色",
      "theme_dark": "暗色",
      "theme_system": "跟随系统",

      // 断开相关
      "disconnecting": "断开中...",
      "force_disconnect": "强制退出",
      "disconnect_waiting": "正在优雅退出，可能需要较长时间。若长时间无响应可强制退出。",
    }
  },
  en: {
    translation: {
      "app_title": "N2N UI",
      "connect": "Connect",
      "disconnect": "Disconnect",
      "connecting": "Connecting...",
      "connected": "Connected",
      "disconnected": "Disconnected",
      "error": "Error",
      "settings": "Settings",
      "logs": "Logs",
      "save": "Save",
      "cancel": "Cancel",
      
      // Configuration
      "supernode": "Supernode Address",
      "community": "Community Name",
      "username": "Node Name",
      "encryption_key": "Encryption Key",
      "ip_mode": "IP Mode",
      "static_ip": "Static IP",
      "dhcp": "DHCP",
      "static": "Static",
      "advanced_settings": "Advanced Settings",
      "extra_args": "Extra Arguments",
      "edge_path": "Edge Path",
      "tap_device": "TAP Device",
      "mtu": "MTU",
      
      // Status
      "virtual_ip": "Virtual IP",
      "status": "Status",
      
      // Messages
      "save_success": "Saved successfully",
      "save_failed": "Failed to save",
      "connect_success": "Connected successfully",
      "connect_failed": "Failed to connect",
      "disconnect_success": "Disconnected successfully",
      "disconnect_failed": "Failed to disconnect",
      
      // Logs
      "show_logs": "Show Logs",
      "hide_logs": "Hide Logs",
      "clear_logs": "Clear Logs",
      "clear": "Clear",
      "no_logs": "No logs yet",
      "log_lines": "lines",
      
      // Network info
      "network_info": "Network Info",
      "ip": "IP",
      "mask": "Mask",
      "mac": "MAC",
      
      // Parameter descriptions
      "supernode_desc": "Supernode server address, format: host:port (e.g. vpn.example.com:7777)",
      "community_desc": "VPN community name, nodes in the same community can communicate",
      "username_desc": "Optional. Used to distinguish different devices; leave empty to use the host name",
      "encryption_key_desc": "AES encryption key (optional), leave empty for no encryption. Strong password recommended",
      "ip_mode_desc": "IP address assignment: DHCP auto-assigns from supernode, static requires manual configuration",
      "static_ip_desc": "Static IP address, format: 10.0.0.2 or 10.0.0.2/24 (CIDR notation)",
      "extra_args_desc": "Additional edge command line arguments, e.g.: -v (verbose), -r (packet forwarding), -E (multicast)",
      "edge_path_desc": "Path to edge executable, leave empty for default path (bin/edge)",
      "tap_device_desc": "TAP virtual network interface name, leave empty for auto-assignment (e.g. edge0)",
      "mtu_desc": "Maximum Transmission Unit, default 1290 bytes. Smaller MTU reduces fragmentation but lowers efficiency",
      
      // Error messages
      "error_mac_in_use": "MAC address already in use, another device may be using same config or supernode hasn't released it",
      "error_ip_in_use": "IP address already in use, check if other devices are using the same IP",
      "error_tap_create_failed": "Failed to create TAP device, please ensure sufficient system permissions",
      "error_supernode_unreachable": "Cannot reach supernode, please check address and network connection",
      "error_auth_failed": "Authentication failed, please check community name and encryption key",
      "error_permission_denied": "Permission denied, Linux requires necessary capabilities for edge binary",
      
      // Theme
      "theme": "Theme",
      "theme_light": "Light",
      "theme_dark": "Dark",
      "theme_system": "System",

      // Disconnect
      "disconnecting": "Disconnecting...",
      "force_disconnect": "Force Quit",
      "disconnect_waiting": "Graceful shutdown in progress, it may take a while. Force quit if it hangs.",
    }
  }
};

i18n
  .use(initReactI18next)
  .init({
    resources,
    lng: 'zh', // 默认语言
    fallbackLng: 'en',
    interpolation: {
      escapeValue: false
    }
  });

export default i18n;
