import { useUIStore } from '../store/uiStore';
import { useSettings, useUpdateSettings } from '../hooks/useSettings';
import { useProviders, useToggleProvider } from '../hooks/useProviders';
import { GlassPanel } from './GlassPanel';
import { ArrowLeft } from 'lucide-react';

export function SettingsPanel() {
  const navigateToOverview = useUIStore((state) => state.navigateToOverview);
  const { data: settings } = useSettings();
  const { data: providers } = useProviders();
  const updateSettings = useUpdateSettings();
  const toggleProvider = useToggleProvider();

  const handleToggleNotifications = () => {
    if (settings) {
      updateSettings.mutate({ ...settings, show_notifications: !settings.show_notifications });
    }
  };

  const handleToggleProvider = (providerId: string, enabled: boolean) => {
    toggleProvider.mutate({ id: providerId, enabled });
  };

  return (
    <div className="h-full flex flex-col animate-popup-in">
      {/* Header */}
      <div className="flex items-center justify-between p-4">
        <div className="flex items-center gap-2">
          <button onClick={navigateToOverview} className="glass-button p-2" aria-label="返回">
            <ArrowLeft className="w-4 h-4" />
          </button>
          <h2 className="text-lg font-semibold text-white/90">设置</h2>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-4 pb-4 space-y-4">
        {/* Global Settings */}
        <GlassPanel>
          <h3 className="text-sm font-medium text-white/70 mb-4">全局设置</h3>
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <span className="text-sm text-white/70">显示通知</span>
              <button
                onClick={handleToggleNotifications}
                className={`w-10 h-5 rounded-full relative transition-colors ${
                  settings?.show_notifications ? 'bg-accent' : 'bg-white/10'
                }`}
                role="switch"
                aria-checked={settings?.show_notifications}
              >
                <div
                  className={`w-4 h-4 rounded-full bg-white absolute top-0.5 transition-transform ${
                    settings?.show_notifications ? 'translate-x-5' : 'translate-x-0.5'
                  }`}
                />
              </button>
            </div>
            <div>
              <label className="text-xs text-white/50 block mb-1">自动刷新间隔（秒）</label>
              <input
                type="number"
                className="glass-input"
                value={settings?.global_refresh_interval || 300}
                readOnly
              />
            </div>
          </div>
        </GlassPanel>

        {/* Provider Settings */}
        <GlassPanel>
          <h3 className="text-sm font-medium text-white/70 mb-4">供应商设置</h3>
          <div className="space-y-4">
            {providers?.map((provider) => (
              <div key={provider.id} className="border-b border-white/5 pb-4 last:border-0">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-sm font-medium text-white/80">{provider.display_name}</span>
                  <button
                    onClick={() => handleToggleProvider(provider.id, !provider.enabled)}
                    className={`w-8 h-4 rounded-full ${provider.enabled ? 'bg-accent' : 'bg-white/10'} relative`}
                    role="switch"
                    aria-checked={provider.enabled}
                    aria-label={`${provider.display_name} 启用状态`}
                  >
                    <div className={`w-3 h-3 rounded-full bg-white absolute top-0.5 transition-transform ${provider.enabled ? 'translate-x-4' : 'translate-x-0.5'}`} />
                  </button>
                </div>
                <div className="text-xs text-white/40">
                  刷新间隔: {provider.refresh_interval_seconds}秒 · 阈值: {provider.warning_threshold_percent}%
                </div>
              </div>
            ))}
          </div>
        </GlassPanel>
      </div>
    </div>
  );
}
