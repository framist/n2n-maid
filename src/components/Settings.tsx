import { useState } from 'react';
import { useTranslation } from 'react-i18next';
import { N2NConfig } from '../types';

/**
 * 服务准备面板组件
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

  const inputClassName = "w-full px-3 py-2 bg-white dark:bg-gray-700 rounded border border-gray-300 dark:border-gray-600 focus:border-blue-500 focus:outline-none transition-colors";

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
    <div className="container mx-auto px-6 py-8 max-w-2xl">
      <form onSubmit={handleSubmit} className="space-y-6">
        {/* 基本设置 */}
        <div className="bg-gray-100 dark:bg-gray-800 rounded-lg p-6 space-y-4 border border-gray-200 dark:border-gray-700">
          <h2 className="text-xl font-bold mb-4">{t('settings')}</h2>
          
          <div>
            <label className="block text-sm font-medium mb-2">
              {t('supernode')} *
            </label>
            <input
              type="text"
              value={formData.supernode}
              onChange={e => handleChange('supernode', e.target.value)}
              placeholder="example.com:7777"
              className={inputClassName}
              required
            />
            <p className="text-xs text-gray-400 mt-1">{t('supernode_desc')}</p>
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">
              {t('community')} *
            </label>
            <input
              type="text"
              value={formData.community}
              onChange={e => handleChange('community', e.target.value)}
              placeholder="my_community"
              className={inputClassName}
              required
            />
            <p className="text-xs text-gray-400 mt-1">{t('community_desc')}</p>
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">
              {t('username')}
            </label>
            <input
              type="text"
              value={formData.username}
              onChange={e => handleChange('username', e.target.value)}
              placeholder=""
              className={inputClassName}
            />
            <p className="text-xs text-gray-400 mt-1">{t('username_desc')}</p>
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">
              {t('encryption_key')}
            </label>
            <input
              type="password"
              value={formData.encryption_key}
              onChange={e => handleChange('encryption_key', e.target.value)}
              placeholder="********"
              className={inputClassName}
            />
            <p className="text-xs text-gray-400 mt-1">{t('encryption_key_desc')}</p>
          </div>

          <div>
            <label className="block text-sm font-medium mb-2">
              {t('ip_mode')}
            </label>
            <select
              value={formData.ip_mode}
              onChange={e => handleChange('ip_mode', e.target.value)}
              className={inputClassName}
            >
              <option value="dhcp">{t('dhcp')}</option>
              <option value="static">{t('static')}</option>
            </select>
            <p className="text-xs text-gray-400 mt-1">{t('ip_mode_desc')}</p>
          </div>

          {formData.ip_mode === 'static' && (
            <div>
              <label className="block text-sm font-medium mb-2">
                {t('static_ip')}
              </label>
              <input
                type="text"
                value={formData.static_ip || ''}
                onChange={e => handleChange('static_ip', e.target.value)}
                placeholder="10.0.0.2"
                className={inputClassName}
              />
              <p className="text-xs text-gray-400 mt-1">{t('static_ip_desc')}</p>
            </div>
          )}
        </div>

        {/* 高级设置 */}
        <div className="bg-gray-100 dark:bg-gray-800 rounded-lg p-6 border border-gray-200 dark:border-gray-700">
          <button
            type="button"
            onClick={() => setShowAdvanced(!showAdvanced)}
            className="w-full text-left font-semibold mb-4 flex justify-between items-center"
          >
            <span>{t('advanced_settings')}</span>
            <span>{showAdvanced ? '▼' : '▶'}</span>
          </button>

          {showAdvanced && (
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium mb-2">
                  {t('edge_path')}
                </label>
                <input
                  type="text"
                  value={formData.edge_path || ''}
                  onChange={e => handleChange('edge_path', e.target.value)}
                  placeholder="/usr/bin/edge"
                  className={inputClassName}
                />
                <p className="text-xs text-gray-400 mt-1">{t('edge_path_desc')}</p>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">
                  {t('tap_device')}
                </label>
                <input
                  type="text"
                  value={formData.tap_device || ''}
                  onChange={e => handleChange('tap_device', e.target.value)}
                  placeholder="edge0"
                  className={inputClassName}
                />
                <p className="text-xs text-gray-400 mt-1">{t('tap_device_desc')}</p>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">
                  {t('mtu')}
                </label>
                <input
                  type="number"
                  value={formData.mtu || 1290}
                  onChange={e => handleChange('mtu', parseInt(e.target.value))}
                  className={inputClassName}
                />
                <p className="text-xs text-gray-400 mt-1">{t('mtu_desc')}</p>
              </div>

              <div>
                <label className="block text-sm font-medium mb-2">
                  {t('extra_args')}
                </label>
                <textarea
                  value={formData.extra_args || ''}
                  onChange={e => handleChange('extra_args', e.target.value)}
                  placeholder="-v -r -E"
                  rows={3}
                  className="w-full px-3 py-2 bg-white dark:bg-gray-700 rounded border border-gray-300 dark:border-gray-600 focus:border-blue-500 focus:outline-none font-mono text-sm transition-colors"
                />
                <p className="text-xs text-gray-400 mt-1">{t('extra_args_desc')}</p>
              </div>
            </div>
          )}
        </div>

        {/* 按钮 */}
        <div className="flex gap-4">
          <button
            type="submit"
            className="flex-1 px-6 py-3 bg-blue-600 rounded-lg hover:bg-blue-700 font-semibold transition-colors text-white"
          >
            {t('save')}
          </button>
          <button
            type="button"
            onClick={onCancel}
            className="flex-1 px-6 py-3 bg-gray-300 dark:bg-gray-600 rounded-lg hover:bg-gray-400 dark:hover:bg-gray-700 font-semibold transition-colors"
          >
            {t('cancel')}
          </button>
        </div>
      </form>
    </div>
  );
};

export default Settings;
