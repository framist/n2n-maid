/// TypeScript 类型定义

/// 网卡信息
export interface NetworkInfo {
  ip: string;
  mask: string;
  mac: string;
}

export interface N2NConfig {
  supernode: string;
  community: string;
  username: string;
  encryption_key: string;
  ip_mode: string;
  static_ip?: string | null;
  extra_args?: string | null;
  edge_path?: string | null;
  tap_device?: string | null;
  mtu?: number | null;
}

export type ConnectionStatus = 'disconnected' | 'connecting' | 'disconnecting' | 'connected' | 'error';

export interface StatusResponse {
  status: ConnectionStatus;
  error: string | null;
  networkInfo?: NetworkInfo | null;
}

export const defaultConfig: N2NConfig = {
  supernode: '',
  community: '',
  username: '',
  encryption_key: '',
  ip_mode: 'dhcp',
  static_ip: null,
  extra_args: null,
  edge_path: null,
  tap_device: null,
  mtu: 1290,
};
