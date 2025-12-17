/**
 * N2N Maid 主界面 - 恩兔酱的工作台 💖
 * 横向布局：左侧背景立绘，右侧卡片化操作面板
 */
import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import { N2NConfig, ConnectionStatus, StatusResponse, NetworkInfo, defaultConfig } from './types';
import LogViewer from './components/LogViewer';
import Settings from './components/Settings';

// 根据连接状态获取对应的立绘图片
const getBackgroundImage = (status: ConnectionStatus): string => {
  switch (status) {
    case 'connected':
      return '/assets/bg-connected.png';
    case 'connecting':
    case 'disconnecting':
      return '/assets/bg-connecting.png';
    case 'error':
      return '/assets/bg-error.png';
    default:
      return '/assets/bg-disconnected.png';
  }
};

function App() {
  const { t, i18n } = useTranslation();
  const [config, setConfig] = useState<N2NConfig>(defaultConfig);
  const [status, setStatus] = useState<ConnectionStatus>('disconnected');
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [networkInfo, setNetworkInfo] = useState<NetworkInfo | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [loading, setLoading] = useState(false);

  // 读取主人的指示
  useEffect(() => {
    loadConfig();
    // 定期检查恩兔的工作状态
    const interval = setInterval(checkStatus, 2000);
    return () => clearInterval(interval);
  }, []);

  const loadConfig = async () => {
    try {
      const loadedConfig = await invoke<N2NConfig>('get_config');
      setConfig(loadedConfig);
    } catch (error) {
      console.error('读取主人指示失败：', error);
    }
  };

  const checkStatus = async () => {
    try {
      const response = await invoke<StatusResponse>('get_status');
      setStatus(response.status);
      setErrorMessage(response.error);
      setNetworkInfo(response.networkInfo || null);
    } catch (error) {
      console.error('查看恩兔工作状态失败：', error);
    }
  };

  const handleConnect = async () => {
    setLoading(true);
    try {
      await invoke('connect', { config });
      console.log('恩兔开始工作啦');
    } catch (error) {
      console.error('启动工作失败：', error);
      alert(`${t('connect_failed')}: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleDisconnect = async () => {
    setLoading(true);
    try {
      setStatus('disconnecting');
      setErrorMessage(null);
      await invoke('disconnect');
    } catch (error) {
      console.error('让恩兔休息失败：', error);
      alert(`${t('disconnect_failed')}: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleForceDisconnect = async () => {
    setLoading(true);
    try {
      await invoke('disconnect_force');
    } catch (error) {
      console.error('强制让恩兔停止失败：', error);
      alert(`${t('disconnect_failed')}: ${error}`);
    } finally {
      setLoading(false);
    }
  };

  const handleSaveConfig = async (newConfig: N2NConfig) => {
    try {
      await invoke('save_config', { config: newConfig });
      setConfig(newConfig);
      setShowSettings(false);
      alert(t('save_success'));
    } catch (error) {
      console.error('保存配置失败：', error);
      alert(`${t('save_failed')}: ${error}`);
    }
  };

  const toggleLanguage = () => {
    const newLang = i18n.language === 'zh' ? 'en' : 'zh';
    i18n.changeLanguage(newLang);
  };

  const getErrorMessage = () => {
    if (!errorMessage) return null;
    if (errorMessage.startsWith('error_')) {
      return t(errorMessage);
    }
    return errorMessage;
  };

  // 获取状态徽章样式
  const getStatusBadgeClass = () => {
    switch (status) {
      case 'connected':
        return 'status-connected';
      case 'connecting':
      case 'disconnecting':
        return 'status-connecting';
      case 'error':
        return 'status-error';
      default:
        return 'status-disconnected';
    }
  };

  // 获取状态图标 - 使用颜文字
  const getStatusIcon = () => {
    switch (status) {
      case 'connected':
        return '(๑˃ᴗ˂)ﻭ';
      case 'connecting':
        return '(｡•̀ᴗ-)✧';
      case 'disconnecting':
        return '(｡•́︿•̀｡)';
      case 'error':
        return '(´；ω；`)';
      default:
        return '(｡-ω-)zzz';
    }
  };

  return (
    <div id="root">
      {/* 全屏背景层 - 立绘/状态背景覆盖整个窗口 */}
      <div className="maid-bg-layer">
        <img
          src={getBackgroundImage(status)}
          alt="恩兔酱背景"
          className="transition-transform duration-[8000ms]"
        />
      </div>

      {/* 顶栏：全宽透明，标题更清晰 */}
      <header className="maid-topbar">
        <h1 className="text-xl font-bold maid-topbar-title">{t('app_title')}</h1>
        <div className="flex gap-2">
          <button
            onClick={toggleLanguage}
            className="text-sm maid-button-secondary"
          >
            {i18n.language === 'zh' ? 'EN' : '中'}
          </button>
          <button
            onClick={() => setShowSettings(!showSettings)}
            className="text-sm maid-button-secondary"
          >
            {t('settings')}
          </button>
        </div>
      </header>

      {/* 右侧浮动操作区，占据窗口约 2/3 宽度 */}
      <div className="maid-shell">
        <div className="maid-panel">
          {/* 主面板区域 */}
          <main className="flex-1 min-h-0 overflow-y-auto pb-14">
            {!showSettings ? (
              <div className="flex flex-col min-h-full gap-5">
                {/* 状态卡片 */}
                <div className="p-5 maid-card">
                  <div className="flex items-center justify-between mb-4">
                    <h2 className="text-lg font-semibold text-gray-700">
                      {t('status')}
                    </h2>
                    <span className={`status-badge ${getStatusBadgeClass()}`}>
                      {getStatusIcon()} {t(status)}
                    </span>
                  </div>

                  {/* 当前配置信息 */}
                  <div className="space-y-2 text-sm">
                    <div className="flex justify-between">
                      <span className="text-gray-500">{t('supernode')}</span>
                      <span className="font-mono text-gray-700">{config.supernode || '-'}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500">{t('community')}</span>
                      <span className="font-mono text-gray-700">{config.community || '-'}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-500">{t('ip_mode')}</span>
                      <span className="text-gray-700">{t(config.ip_mode)}</span>
                    </div>
                  </div>



                  {/* 错误信息 */}
                  {status === 'error' && errorMessage && (
                    <div className="p-3 mt-4 border border-red-200 rounded-lg bg-red-50">
                      <p className="text-sm text-red-700">
                        {getErrorMessage()}
                      </p>
                    </div>
                  )}

                  {/* 断开等待提示 */}
                  {status === 'disconnecting' && (
                    <p className="mt-3 text-sm text-center text-gray-500">
                      {t('disconnect_waiting')}
                    </p>
                  )}
                </div>

                {/* 网络信息（连接后显示） */}
                  {status === 'connected' && networkInfo && (
                    <div className="mt-4 network-info-card">
                      <p className="mb-2 text-sm font-medium text-gray-700">
                        {t('network_info')}
                      </p>
                      <div className="grid grid-cols-3 gap-2 font-mono text-xs">
                        <div>
                          <span className="text-gray-500">{t('ip')}:</span>
                          <br />
                          <span className="ml-1 text-gray-700">{networkInfo.ip}</span>
                        </div>
                        <div>
                          <span className="text-gray-500">{t('mask')}:</span>
                          <br />
                          <span className="ml-1 text-gray-700">{networkInfo.mask}</span>
                        </div>
                        <div>
                          <span className="text-gray-500">{t('mac')}:</span>
                          <br />
                          <span className="ml-1 text-gray-700">{networkInfo.mac}</span>
                        </div>
                      </div>
                    </div>
                  )}

                {/* 操作按钮区：沉底 */}
                <div className="flex gap-3 mt-auto">
                  {status === 'disconnected' || status === 'error' ? (
                    <button
                      onClick={handleConnect}
                      disabled={loading || !config.supernode || !config.community}
                      className="flex-1 py-4 text-lg maid-button-primary"
                    >
                      {loading ? t('connecting') : t('connect')}
                    </button>
                  ) : status === 'disconnecting' ? (
                    <button
                      onClick={handleForceDisconnect}
                      disabled={loading}
                      className="flex-1 py-4 text-lg maid-button-danger"
                    >
                      {t('force_disconnect')}
                    </button>
                  ) : (
                    <button
                      onClick={handleDisconnect}
                      disabled={loading}
                      className="flex-1 py-4 text-lg maid-button-disconnect"
                    >
                      {t('disconnect')}
                    </button>
                  )}
                </div>
              </div>
            ) : (
              <Settings
                config={config}
                onSave={handleSaveConfig}
                onCancel={() => setShowSettings(false)}
              />
            )}
          </main>
        </div>
      </div>

      {/* 底部日志面板 - 独立于面板，占据整个窗口底部 */}
      <LogViewer />
    </div>
  );
}

export default App;
