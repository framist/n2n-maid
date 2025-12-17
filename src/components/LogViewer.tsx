import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from 'react-i18next';

/**
 * 工作日志查看器组件 - 稿纸风格 📝
 * 恩兔的工作汇报区域，固定在窗口底部
 * 采用温暖的浅黄色稿纸风格，半透明设计
 */
const LogViewer: React.FC = () => {
  const { t } = useTranslation();
  const [logs, setLogs] = useState<string[]>([]);
  const [isCollapsed, setIsCollapsed] = useState(true);
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
   * 渲染带颜色的汇报行 - 温暖色调
   * [OUT] = 正常工作，[ERR] = 出错了，[WARN] = 需要注意，[INFO] = 进展信息
   */
  const renderLogLine = (log: string) => {
    if (log.startsWith('[ERR]')) {
      return <span className="text-red-600">{log}</span>;
    } else if (log.startsWith('[WARN]')) {
      return <span className="text-amber-600">{log}</span>;
    } else if (log.startsWith('[INFO]')) {
      return <span className="text-blue-600">{log}</span>;
    } else if (log.startsWith('[OUT]')) {
      return <span className="text-gray-700">{log}</span>;
    } else {
      return <span className="text-gray-500">{log}</span>;
    }
  };

  return (
    <div className={`fixed bottom-0 left-0 right-0 transition-all duration-300 z-30 ${isCollapsed ? '' : 'maid-log-paper'}`}>
      {/* 稿纸标题栏（可点击折叠/展开） */}
      <div 
        className={`maid-log-header flex items-center justify-between px-4 py-2.5 cursor-pointer transition-colors ${isCollapsed ? '' : 'maid-log-header-expanded'}`}
        onClick={() => setIsCollapsed(!isCollapsed)}
      >
        <div className="flex items-center gap-2">
          <span className="text-sm font-semibold text-gray-600">
            {isCollapsed ? '▶' : '▼'} {t('logs')}
          </span>
          <span className="text-xs px-2 py-0.5 bg-white/60 rounded-full text-gray-500">
            {logs.length} {t('log_lines')}
          </span>
        </div>
        <div className="flex items-center gap-2" onClick={(e) => e.stopPropagation()}>
          <button
            onClick={handleClearLogs}
            className="px-3 py-1 text-xs text-gray-500 bg-white/70 rounded-lg hover:bg-white transition-colors border border-gray-200"
            title={t('clear_logs')}
          >
            {t('clear')}
          </button>
        </div>
      </div>
      
      {/* 稿纸内容区域（可折叠） */}
      {!isCollapsed && (
        <div className="maid-log-content h-40 p-3 overflow-y-auto font-mono text-xs">
          {logs.length === 0 ? (
            <p className="text-gray-400 italic log-line">{t('no_logs')}</p>
          ) : (
            logs.map((log, index) => (
              <div key={index} className="log-line leading-6">
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
