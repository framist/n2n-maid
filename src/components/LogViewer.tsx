import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';

/**
 * 工作日志查看器组件 - 稿纸风格
 * 恩兔的工作汇报区域，固定在窗口底部
 * 支持彩色输出，方便主人查看
 */
const LogViewer: React.FC = () => {
  const { t } = useTranslation();
  const [logs, setLogs] = useState<string[]>([]);
  const [isCollapsed, setIsCollapsed] = useState(false);
  const logEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // 定期接收恩兔的工作汇报
    const interval = setInterval(async () => {
      try {
        const newLogs = await invoke<string[]>('get_logs');
        if (newLogs.length > 0) {
          setLogs(prev => [...prev, ...newLogs]);
        }
      } catch (error) {
        console.error('接收工作汇报失败：', error);
      }
    }, 500);

    return () => clearInterval(interval);
  }, []);

  // 自动滚动到最新的汇报
  useEffect(() => {
    logEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [logs]);

  const handleClearLogs = () => {
    setLogs([]);
  };

  /**
   * 渲染带颜色的汇报行
   * [OUT] = 正常，[ERR] = 出错了，[WARN] = 需要注意，[INFO] = 进展信息
   */
  const renderLogLine = (log: string) => {
    if (log.startsWith('[ERR]')) {
      return <span className="text-red-500">{log}</span>;
    } else if (log.startsWith('[WARN]')) {
      return <span className="text-yellow-500">{log}</span>;
    } else if (log.startsWith('[INFO]')) {
      return <span className="text-blue-400">{log}</span>;
    } else if (log.startsWith('[OUT]')) {
      return <span className="text-gray-100">{log}</span>;
    } else {
      return <span className="text-gray-400">{log}</span>;
    }
  };

  return (
    <div className="fixed bottom-0 left-0 right-0 transition-all duration-300 bg-gray-900 border-t border-gray-300 dark:border-gray-700">
      {/* 终端标题栏（可点击折叠/展开） */}
      <div 
        className="flex items-center justify-between px-4 py-2 transition-colors bg-gray-800 border-b border-gray-700 cursor-pointer hover:bg-gray-750"
        onClick={() => setIsCollapsed(!isCollapsed)}
      >
        <div className="flex items-center gap-2">
          <span className="text-sm font-semibold text-gray-300">
            {isCollapsed ? '▶' : '▼'} 📝 {t('logs')}
          </span>
          <span className="text-xs text-gray-500">({logs.length} {t('log_lines')})</span>
        </div>
        <div className="flex items-center gap-2" onClick={(e) => e.stopPropagation()}>
          <button
            onClick={handleClearLogs}
            className="px-3 py-1 text-xs text-gray-300 transition-colors bg-gray-700 rounded hover:bg-gray-600"
            title={t('clear_logs')}
          >
            🗑️ {t('clear')}
          </button>
        </div>
      </div>
      
      {/* 终端内容区域（可折叠） */}
      {!isCollapsed && (
        <div className="h-48 p-3 overflow-y-auto font-mono text-xs bg-black">
          {logs.length === 0 ? (
            <p className="text-gray-600">{t('no_logs')}</p>
          ) : (
            logs.map((log, index) => (
              <div key={index} className="leading-5">
                {renderLogLine(log)}
              </div>
            ))
          )}
          <div ref={logEndRef} />
        </div>
      )}
    </div>
  );
};

export default LogViewer;
