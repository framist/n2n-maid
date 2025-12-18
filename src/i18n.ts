import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

const resources = {
  zh: {
    translation: {
      "app_title": "N2N Maid · 恩兔 妹抖",
      "connect": "建立通道",
      "disconnect": "休息一下",
      "connecting": "正在铺设通道...",
      "connected": "扫清障碍，通道已建立完毕！",
      "disconnected": "恩兔在待命中",
      "error": "呜呜，出错了",
      "settings": "服务准备",
      "logs": "工作日志",
      "save": "可以",
      "cancel": "哒咩",
      
      // 配置相关 - 家政预约风格
      "supernode": "总部地址",
      "community": "小组暗号",
      "username": "我是谁",
      "encryption_key": "保密密语",
      "ip_mode": "地址分配",
      "static_ip": "指定地址",
      "dhcp": "自动的 (dhcp)",
      "static": "不动的 (static)",
      "advanced_settings": "高级的设置",
      "extra_args": "奇妙指令",
      "edge_path": "工具箱路径",
      "tap_device": "设备名称",
      "mtu": "通道宽度",
      
      // 状态信息
      "virtual_ip": "我的地址",
      "status": "工作状态",
      
      // 提示信息 - 女仆口吻
      "save_success": "主人的指示已记下！",
      "save_failed": "哎呀，记录失败了...",
      "connect_success": "走廊已经扫干净啦，主人可以随时通行！",
      "connect_failed": "抱歉主人，通道出故障了",
      "disconnect_success": "恩兔去休息咯~",
      "disconnect_failed": "唔...收拾工具时出错了",
      
      // 日志相关
      "show_logs": "查看日志",
      "hide_logs": "收起日志",
      "clear_logs": "清空记录",
      "clear": "清空",
      "no_logs": "还没有工作记录呢",
      "log_lines": "条记录",
      
      // 网卡信息
      "network_info": "通道详情",
      "network_info_waiting": "恩兔正在整理通道回执单（网卡信息还没拿到）…",
      "ip": "地址",
      "mask": "子网掩码",
      "mac": "硬件编号",

      // 同伴节点
      "peer_list": "同伴列表",
      "peer_list_empty": "暂时还没看到其他伙伴呢",
      "peer_name": "昵称",
      "peer_vpn_ip": "VPN 地址",
      "peer_mode": "模式",
      "peer_public_addr": "公网地址",
      "peer_latency": "延迟",
      "peer_last_seen": "再见",
      "latency_unknown": "—",
      
      // 参数说明 - 简化且亲切
      "supernode_desc": "总部服务器的地址，告诉恩兔去哪里领任务呀（格式：vpn.example.com:7777）",
      "community_desc": "工作团队的暗号，只有知道暗号的伙伴才能进入同一个通道哦",
      "username_desc": "可选。给设备起个昵称，方便主人辨认；留空就用电脑的名字啦",
      "encryption_key_desc": "保密用的密语（可选），设置后通道会更安全。留空就不加密，但不太推荐呢",
      "ip_mode_desc": "选择恩兔怎么获取地址：自动分配就交给总部，手动指定就由主人决定",
      "static_ip_desc": "手动指定的地址，格式像这样：192.168.125.xxx",
      "extra_args_desc": "给恩兔的魔法掸子加点特技，比如 -v（多话模式）、-r（帮忙转发）、-E（接收广播）",
      "edge_path_desc": "恩兔的工具箱放在哪里，留空就用默认位置（bin/edge）",
      "tap_device_desc": "虚拟网卡的名字，留空恩兔会自动取名（比如 edge0）",
      "mtu_desc": "通道的宽度，默认 1290。太小会影响速度，太大可能卡住",
      
      // 错误提示 - 安慰式
      "error_mac_in_use": "这个硬件编号已经有人用了，可能是其他设备还在工作，或者总部还没注销旧记录",
      "error_ip_in_use": "这个地址已经有人占了，要不换一个试试？",
      "error_mac_or_ip_in_use": "总部说这台设备的硬件编号/地址还没释放呢（可能上次断开太匆忙）。主人可以先等一小会儿让总部清理，或者点“断开”让恩兔发出 SIGINT 再重新连接。",
      "error_tap_create_failed": "恩兔没权限创建设备，需要主人帮忙授权呢",
      "error_tap_busy": "虚拟网卡被占用了（Device or resource busy）。可能已经有别的 edge 在工作，或者 TAP 名称冲突；请先断开其它连接/重启相关服务，或在设置里换个 TAP 设备名再试。",
      "error_supernode_unreachable": "联系不上总部了，是不是地址写错了，或者网络断了？",
      "error_auth_failed": "门被锁住了，检查一下暗号和密语对不对吧",
      "error_permission_denied": "权限不够呀，Linux 系统需要给工具箱特殊权限才行",
      "error_edge_exited": "呜呜…恩兔的工具箱突然停工了（edge 意外退出）。请打开“工作汇报”看看最后一条报错线索。",
      
      // 快捷配置
      "quick_config": "当前配置",

      // 断开相关
      "disconnecting": "正在收拾工具...",
      "force_disconnect": "立即停止",
      "disconnect_waiting": "恩兔在温柔地关闭通道，可能需要一小会儿。如果一直等不来，可以选择强制停止哦。",
    }
  },
  en: {
    translation: {
      "app_title": "N2N Maid · N-Too",
      "connect": "Build the Path",
      "disconnect": "Take a Break",
      "connecting": "Preparing your path...",
      "connected": "Master, your path is ready!",
      "disconnected": "N-Too is on standby",
      "error": "Oopsy! Something went wrong",
      "settings": "Mission Prep",
      "logs": "Work Report",
      "save": "Confirm",
      "cancel": "Cancel",
      
      // Configuration - Home service style
      "supernode": "Head Office",
      "community": "Secret Code",
      "username": "My ID",
      "encryption_key": "Secret Password",
      "ip_mode": "Address Mode",
      "static_ip": "Fixed Address",
      "dhcp": "Auto Assign",
      "static": "Manual",
      "advanced_settings": "Advanced",
      "extra_args": "Special Orders",
      "edge_path": "Toolbox Path",
      "tap_device": "Device Name",
      "mtu": "Tunnel Width",
      
      // Status
      "virtual_ip": "My Address",
      "status": "Work Status",
      
      // Messages - Maid tone
      "save_success": "Master's orders noted!",
      "save_failed": "Oops, failed to record...",
      "connect_success": "The hallway is sparkling clean, Master can pass anytime!",
      "connect_failed": "Sorry Master, the tunnel is broken",
      "disconnect_success": "N-Too is off to rest~",
      "disconnect_failed": "Um... error packing up tools",
      
      // Logs
      "show_logs": "View Logs",
      "hide_logs": "Hide Logs",
      "clear_logs": "Clear Records",
      "clear": "Clear",
      "no_logs": "No work records yet",
      "log_lines": "records",
      
      // Network info
      "network_info": "Tunnel Details",
      "network_info_waiting": "N-Too is still preparing the receipt (network details not ready yet)…",
      "ip": "Address",
      "mask": "Subnet Mask",
      "mac": "Hardware ID",

      // Peers
      "peer_list": "Peers",
      "peer_list_empty": "No other peers spotted yet...",
      "peer_name": "Name",
      "peer_vpn_ip": "VPN IP",
      "peer_mode": "Mode",
      "peer_public_addr": "Public",
      "peer_latency": "Latency",
      "peer_last_seen": "Last seen",
      "latency_unknown": "—",
      
      // Parameter descriptions - Simplified and friendly
      "supernode_desc": "The head office address where N-Too gets her tasks (format: vpn.example.com:7777)",
      "community_desc": "Team secret code - only friends who know it can enter the same tunnel",
      "username_desc": "Optional. Give your device a nickname for easy recognition; leave empty to use computer name",
      "encryption_key_desc": "Secret password (optional) to make the tunnel safer. Leave empty means no encryption, but not recommended",
      "ip_mode_desc": "How N-Too gets the address: Auto means head office assigns, Manual means Master decides",
      "static_ip_desc": "Manually specified address, like this: 10.0.0.2 or 10.0.0.2/24",
      "extra_args_desc": "Special skills for N-Too's magic duster, like -v (chatty mode), -r (help forward), -E (receive broadcast)",
      "edge_path_desc": "Where N-Too's toolbox is stored, leave empty for default location (bin/edge)",
      "tap_device_desc": "Virtual network card name, leave empty and N-Too will pick one (like edge0)",
      "mtu_desc": "Tunnel width, default 1290. Too small affects speed, too large might get stuck",
      
      // Error messages - Comforting style
      "error_mac_in_use": "This hardware ID is already taken, maybe another device is using it or head office hasn't cleared the old record",
      "error_ip_in_use": "This address is occupied, shall we try another one?",
      "error_mac_or_ip_in_use": "Head office says your MAC/IP is still marked as in-use (maybe the last session wasn't released yet). You can wait a bit, or click “Take a Break” to send SIGINT and try again.",
      "error_tap_create_failed": "N-Too doesn't have permission to create device, Master needs to grant authorization",
      "error_tap_busy": "The virtual adapter is busy (Device or resource busy). Another edge may still be running, or the TAP name conflicts. Disconnect other sessions or change the TAP device name and retry.",
      "error_supernode_unreachable": "Can't reach head office, is the address wrong or network disconnected?",
      "error_auth_failed": "The door is locked, please check if the code and password are correct",
      "error_permission_denied": "Not enough permission, Linux systems need special capabilities for the toolbox",
      "error_edge_exited": "Oops… the toolbox stopped unexpectedly (edge exited). Check the “Work Report” for the last error line.",
      
      // Quick config
      "quick_config": "Current Config",

      // Disconnect
      "disconnecting": "Packing up tools...",
      "force_disconnect": "Force Stop",
      "disconnect_waiting": "N-Too is gently closing the tunnel, might take a moment. If it takes too long, you can force stop.",
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
