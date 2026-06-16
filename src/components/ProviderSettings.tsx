import { ProviderConfig } from '../types';

interface ProviderSettingsProps {
  provider: ProviderConfig;
  onUpdate: (provider: ProviderConfig) => void;
}

export function ProviderSettings({ provider, onUpdate }: ProviderSettingsProps) {
  return (
    <div className="space-y-4">
      <div>
        <label className="text-xs text-white/50 block mb-1">显示名称</label>
        <input
          type="text"
          className="glass-input"
          value={provider.display_name}
          onChange={(e) => onUpdate({ ...provider, display_name: e.target.value })}
        />
      </div>
      <div>
        <label className="text-xs text-white/50 block mb-1">API Base URL</label>
        <input
          type="text"
          className="glass-input"
          value={provider.api_base_url}
          onChange={(e) => onUpdate({ ...provider, api_base_url: e.target.value })}
        />
      </div>
      <div>
        <label className="text-xs text-white/50 block mb-1">警告阈值 (%)</label>
        <input
          type="number"
          className="glass-input"
          value={provider.warning_threshold_percent}
          onChange={(e) => onUpdate({ ...provider, warning_threshold_percent: parseFloat(e.target.value) })}
          min={0}
          max={100}
        />
      </div>
      <div>
        <label className="text-xs text-white/50 block mb-1">刷新间隔 (秒)</label>
        <input
          type="number"
          className="glass-input"
          value={provider.refresh_interval_seconds}
          onChange={(e) => onUpdate({ ...provider, refresh_interval_seconds: parseInt(e.target.value) })}
          min={60}
        />
      </div>
    </div>
  );
}
