import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { N2NConfig } from '../types';

/**
 * 服务准备面板组件 - 粉色卡片风格 💖
 * 恩兔用来接收主人的工作指示
 */
interface SettingsProps {
  config: N2NConfig;
  onSave: (config: N2NConfig) => void;
  onCancel: () => void;
}

const Settings: React.FC<SettingsProps> = ({ config, onSave, onCancel }) => {
  const { t } = useTranslation();
  const [formData, setFormData] = useState<N2NConfig>(config);
  const [showAdvanced, setShowAdvanced] = useState(false);

  const handleChange = (field: keyof N2NConfig, value: any) => {
    setFormData(prev => ({
      ...prev,
      [field]: value
    }));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave(formData);
  };

  return (
    <div className="flex flex-col h-full min-h-0">
      <form onSubmit={handleSubmit} className="flex flex-col flex-1 min-h-0">
        <div className="flex-1 min-h-0 pb-6 space-y-5 overflow-y-auto">
          {/* 基本设置卡片 */}
          <div className="p-5 space-y-4 maid-card">
            <h2 className="text-lg font-bold text-gray-700">
              {t('settings')}
            </h2>
          
          {/* 总部地址 */}
          <div>
            <label htmlFor="supernode" className="block text-sm font-medium text-gray-600 mb-1.5">
              {t('supernode')} *
            </label>
            <input
              id="supernode"
              type="text"
              value={formData.supernode}
              onChange={e => handleChange('supernode', e.target.value)}
              placeholder="example.com:7777"
              className="maid-input"
              required
            />
            <p className="mt-1 text-xs text-gray-400">{t('supernode_desc')}</p>
          </div>

          {/* 工作暗号 */}
          <div>
            <label htmlFor="community" className="block text-sm font-medium text-gray-600 mb-1.5">
              {t('community')} *
            </label>
            <input
              id="community"
              type="text"
              value={formData.community}
              onChange={e => handleChange('community', e.target.value)}
              placeholder="my_community"
              className="maid-input"
              required
            />
            <p className="mt-1 text-xs text-gray-400">{t('community_desc')}</p>
          </div>

          {/* 我的工号 */}
          <div>
            <label htmlFor="username" className="block text-sm font-medium text-gray-600 mb-1.5">
              {t('username')}
            </label>
            <input
              id="username"
              type="text"
              value={formData.username}
              onChange={e => handleChange('username', e.target.value)}
              className="maid-input"
            />
            <p className="mt-1 text-xs text-gray-400">{t('username_desc')}</p>
          </div>

          {/* 保密密语 */}
          <div>
            <label htmlFor="encryption_key" className="block text-sm font-medium text-gray-600 mb-1.5">
              {t('encryption_key')}
            </label>
            <input
              id="encryption_key"
              type="password"
              value={formData.encryption_key}
              onChange={e => handleChange('encryption_key', e.target.value)}
              placeholder="********"
              className="maid-input"
            />
            <p className="mt-1 text-xs text-gray-400">{t('encryption_key_desc')}</p>
          </div>

          {/* 地址分配模式 */}
          <div>
            <label htmlFor="ip_mode" className="block text-sm font-medium text-gray-600 mb-1.5">
              {t('ip_mode')}
            </label>
            <select
              id="ip_mode"
              value={formData.ip_mode}
              onChange={e => handleChange('ip_mode', e.target.value)}
              className="maid-input"
            >
              <option value="dhcp">{t('dhcp')}</option>
              <option value="static">{t('static')}</option>
            </select>
            <p className="mt-1 text-xs text-gray-400">{t('ip_mode_desc')}</p>
          </div>

          {/* 静态 IP（仅在手动模式时显示） */}
          {formData.ip_mode === 'static' && (
            <div>
              <label htmlFor="static_ip" className="block text-sm font-medium text-gray-600 mb-1.5">
                {t('static_ip')}
              </label>
              <input
                id="static_ip"
                type="text"
                value={formData.static_ip || ''}
                onChange={e => handleChange('static_ip', e.target.value)}
                placeholder="10.0.0.2"
                className="maid-input"
              />
              <p className="mt-1 text-xs text-gray-400">{t('static_ip_desc')}</p>
            </div>
          )}
          </div>

          {/* 高级设置卡片 */}
          <div className="p-5 maid-card">
            <button
              type="button"
              onClick={() => setShowAdvanced(!showAdvanced)}
              className="flex items-center justify-between w-full font-semibold text-left text-gray-600"
            >
              <span>
                {t('advanced_settings')}
              </span>
              <span className="text-gray-400">{showAdvanced ? '▼' : '▶'}</span>
            </button>

            {showAdvanced && (
              <div className="pt-4 mt-4 space-y-4 border-t border-maid-pink">
              {/* 工具箱路径 */}
              <div>
                <label htmlFor="edge_path" className="block text-sm font-medium text-gray-600 mb-1.5">
                  {t('edge_path')}
                </label>
                <input
                  id="edge_path"
                  type="text"
                  value={formData.edge_path || ''}
                  onChange={e => handleChange('edge_path', e.target.value)}
                  placeholder="/usr/bin/edge"
                  className="font-mono text-sm maid-input"
                />
                <p className="mt-1 text-xs text-gray-400">{t('edge_path_desc')}</p>
              </div>

              {/* 设备名称 */}
              <div>
                <label htmlFor="tap_device" className="block text-sm font-medium text-gray-600 mb-1.5">
                  {t('tap_device')}
                </label>
                <input
                  id="tap_device"
                  type="text"
                  value={formData.tap_device || ''}
                  onChange={e => handleChange('tap_device', e.target.value)}
                  placeholder="edge0"
                  className="font-mono text-sm maid-input"
                />
                <p className="mt-1 text-xs text-gray-400">{t('tap_device_desc')}</p>
              </div>

              {/* MTU */}
              <div>
                <label htmlFor="mtu" className="block text-sm font-medium text-gray-600 mb-1.5">
                  {t('mtu')}
                </label>
                <input
                  id="mtu"
                  type="number"
                  value={formData.mtu || 1290}
                  onChange={e => handleChange('mtu', parseInt(e.target.value))}
                  className="maid-input"
                />
                <p className="mt-1 text-xs text-gray-400">{t('mtu_desc')}</p>
              </div>

              {/* 额外参数 */}
              <div>
                <label htmlFor="extra_args" className="block text-sm font-medium text-gray-600 mb-1.5">
                  {t('extra_args')}
                </label>
                <textarea
                  id="extra_args"
                  value={formData.extra_args || ''}
                  onChange={e => handleChange('extra_args', e.target.value)}
                  placeholder="-v -r -E"
                  rows={2}
                  className="font-mono text-sm resize-none maid-input"
                />
                <p className="mt-1 text-xs text-gray-400">{t('extra_args_desc')}</p>
              </div>
              </div>
            )}
          </div>
        </div>

        {/* 操作按钮：固定在可视底部 */}
        <div className="flex gap-3 pt-4">
          <button
            type="submit"
            className="flex-1 py-3 maid-button-primary"
          >
            {t('save')}
          </button>
          <button
            type="button"
            onClick={onCancel}
            className="flex-1 py-3 maid-button-secondary"
          >
            {t('cancel')}
          </button>
        </div>
      </form>
    </div>
  );
};

export default Settings;
