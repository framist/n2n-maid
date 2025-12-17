import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';
import { N2NConfig, ConnectionStatus, StatusResponse, NetworkInfo, defaultConfig } from './types';
import LogViewer from './components/LogViewer';
import Settings from './components/Settings';

function App() {
  const { t, i18n } = useTranslation();
  const [config, setConfig] = useState<N2NConfig>(defaultConfig);
  const [status, setStatus] = useState<ConnectionStatus>('disconnected');
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [networkInfo, setNetworkInfo] = useState<NetworkInfo | null>(null);
  const [showSettings, setShowSettings] = useState(false);
  const [loading, setLoading] = useState(false);
  const [theme, setTheme] = useState<string>('system');

  // 读取主人的指示
  useEffect(() => {
    loadConfig();
    
    // 定期检查恩兔的工作状态
    const interval = setInterval(checkStatus, 2000);
    return () => clearInterval(interval);
  }, []);

  // 调整外观
  useEffect(() => {
    applyTheme(theme);
  }, [theme]);

  const loadConfig = async () => {
    try {
      const loadedConfig = await invoke<N2NConfig>('get_config');
      setConfig(loadedConfig);
      setTheme(loadedConfig.theme || 'system');
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

  const applyTheme = (selectedTheme: string) => {
    const root = document.documentElement;
    
    if (selectedTheme === 'system') {
      // 检测系统主题
      const isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
      root.classList.toggle('dark', isDark);
    } else {
      root.classList.toggle('dark', selectedTheme === 'dark');
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
      // 收拾工具可能需要一点时间，先提示主人等待
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
      setTheme(newConfig.theme || 'system');
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

  const toggleTheme = () => {
    const themes = ['light', 'dark', 'system'];
    const currentIndex = themes.indexOf(theme);
    const nextTheme = themes[(currentIndex + 1) % themes.length];
    setTheme(nextTheme);
    
    // 保存主题设置
    const newConfig = { ...config, theme: nextTheme };
    invoke('save_config', { config: newConfig }).catch(error => {
      console.error('保存主题失败：', error);
    });
  };

  const getThemeIcon = () => {
    switch (theme) {
      case 'light':
        return '☀️';
      case 'dark':
        return '🌙';
      case 'system':
        return '💻';
      default:
        return '💻';
    }
  };

  const getErrorMessage = () => {
    if (!errorMessage) return null;
    
    // 如果错误消息是翻译键，则翻译它
    if (errorMessage.startsWith('error_')) {
      return t(errorMessage);
    }
    // 否则直接显示原始错误消息
    return errorMessage;
  };

  const getStatusColor = () => {
    switch (status) {
      case 'connected':
        return 'bg-green-500';
      case 'connecting':
      case 'disconnecting':
        return 'bg-yellow-500';
      case 'error':
        return 'bg-red-500';
      default:
        return 'bg-gray-500';
    }
  };

  const getStatusText = () => {
    return t(status);
  };

  return (
    <div className="min-h-screen pb-48 text-gray-900 transition-colors bg-white dark:bg-gray-900 dark:text-white">
      {/* 头部 */}
      <div className="flex items-center justify-between px-6 py-4 bg-gray-100 border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700">
        <h1 className="text-2xl font-bold">{t('app_title')}</h1>
        <div className="flex gap-2">
          <button
            onClick={toggleTheme}
            className="px-3 py-1 text-sm transition-colors bg-gray-200 rounded dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600"
            title={t('theme')}
          >
            {getThemeIcon()}
          </button>
          <button
            onClick={toggleLanguage}
            className="px-3 py-1 text-sm transition-colors bg-gray-200 rounded dark:bg-gray-700 hover:bg-gray-300 dark:hover:bg-gray-600"
          >
            {i18n.language === 'zh' ? 'EN' : '中'}
          </button>
          <button
            onClick={() => setShowSettings(!showSettings)}
            className="px-4 py-2 transition-colors bg-blue-600 rounded hover:bg-blue-700"
          >
            {t('settings')}
          </button>
        </div>
      </div>

      {/* 主界面 */}
      {!showSettings ? (
        <div className="container px-6 py-8 mx-auto">
          {/* 状态指示器 */}
          <div className="flex flex-col items-center mb-8">
            <div className={`w-32 h-32 rounded-full ${getStatusColor()} flex items-center justify-center mb-4 transition-colors duration-300`}>
              <span className="text-2xl font-bold">{getStatusText()}</span>
            </div>
            
            {/* 连接信息 */}
            <div className="mb-6 text-center">
              <p className="mb-2 text-gray-600 dark:text-gray-400">{t('community')}: {config.community || '-'}</p>
              <p className="text-gray-600 dark:text-gray-400">{t('supernode')}: {config.supernode || '-'}</p>
              
              {/* 网卡信息（仅连接后显示） */}
              {status === 'connected' && networkInfo && (
                <div className="p-4 mt-4 border border-green-200 rounded-lg bg-green-50 dark:bg-green-900/20 dark:border-green-700">
                  <p className="mb-2 text-sm font-semibold text-green-800 dark:text-green-300">{t('network_info')}:</p>
                  <div className="flex flex-col gap-1 font-mono text-xs">
                    <p className="text-green-700 dark:text-green-400">{t('ip')}: {networkInfo.ip}</p>
                    <p className="text-green-700 dark:text-green-400">{t('mask')}: {networkInfo.mask}</p>
                    <p className="text-green-700 dark:text-green-400">{t('mac')}: {networkInfo.mac}</p>
                  </div>
                </div>
              )}
            </div>

            {/* 错误消息显示 */}
            {status === 'error' && errorMessage && (
              <div className="p-4 mb-6 border border-red-300 rounded-lg bg-red-50 dark:bg-red-900/50 dark:border-red-500">
                <p className="text-sm text-red-800 dark:text-red-200">
                  <strong>{t('error')}:</strong> {getErrorMessage()}
                </p>
              </div>
            )}

            {/* 连接/断开按钮 */}
            <div className="flex gap-4">
              {status === 'disconnected' || status === 'error' ? (
                <button
                  onClick={handleConnect}
                  disabled={loading || !config.supernode || !config.community}
                  className="px-8 py-4 text-lg font-semibold transition-colors bg-green-600 rounded-lg hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed"
                >
                  {loading ? t('connecting') : t('connect')}
                </button>
              ) : status === 'disconnecting' ? (
                <button
                  onClick={handleForceDisconnect}
                  disabled={loading}
                  className="px-8 py-4 text-lg font-semibold transition-colors bg-red-600 rounded-lg hover:bg-red-700 disabled:bg-gray-600 disabled:cursor-not-allowed"
                >
                  {t('force_disconnect')}
                </button>
              ) : (
                <button
                  onClick={handleDisconnect}
                  disabled={loading}
                  className="px-8 py-4 text-lg font-semibold transition-colors bg-red-600 rounded-lg hover:bg-red-700 disabled:bg-gray-600 disabled:cursor-not-allowed"
                >
                  {t('disconnect')}
                </button>
              )}
            </div>

            {status === 'disconnecting' && (
              <p className="max-w-md mt-3 text-sm text-center text-gray-600 dark:text-gray-400">
                {t('disconnect_waiting')}
              </p>
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
      
      {/* 固定在底部的日志终端 */}
      <LogViewer />
    </div>
  );
}

export default App;
