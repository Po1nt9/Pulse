import { useEffect, useState } from 'react';
import { ProviderConfig } from '../types';

interface ProviderSettingsProps {
  provider: ProviderConfig;
  onUpdate: (provider: ProviderConfig) => void;
}

interface FormState {
  display_name: string;
  api_base_url: string;
  warning_threshold_percent: number;
  refresh_interval_seconds: number;
}

function toForm(p: ProviderConfig): FormState {
  return {
    display_name: p.display_name,
    api_base_url: p.api_base_url,
    warning_threshold_percent: p.warning_threshold_percent,
    refresh_interval_seconds: p.refresh_interval_seconds,
  };
}

export function ProviderSettings({ provider, onUpdate }: ProviderSettingsProps) {
  const [form, setForm] = useState<FormState>(() => toForm(provider));

  useEffect(() => {
    setForm(toForm(provider));
  }, [provider]);

  const commit = (next: FormState) => {
    setForm(next);
    onUpdate({
      ...provider,
      display_name: next.display_name,
      api_base_url: next.api_base_url,
      warning_threshold_percent: next.warning_threshold_percent,
      refresh_interval_seconds: next.refresh_interval_seconds,
    });
  };

  return (
    <div className="space-y-4">
      <div>
        <label className="text-xs text-white/50 block mb-1">显示名称</label>
        <input
          type="text"
          className="glass-input"
          value={form.display_name}
          onChange={(e) => setForm((f) => ({ ...f, display_name: e.target.value }))}
          onBlur={() => commit(form)}
        />
      </div>
      <div>
        <label className="text-xs text-white/50 block mb-1">API Base URL</label>
        <input
          type="text"
          className="glass-input"
          value={form.api_base_url}
          onChange={(e) => setForm((f) => ({ ...f, api_base_url: e.target.value }))}
          onBlur={() => commit(form)}
        />
      </div>
      <div>
        <label className="text-xs text-white/50 block mb-1">警告阈值 (%)</label>
        <input
          type="number"
          className="glass-input"
          value={form.warning_threshold_percent}
          onChange={(e) => setForm((f) => ({ ...f, warning_threshold_percent: parseFloat(e.target.value) }))}
          onBlur={() => commit(form)}
          min={0}
          max={100}
        />
      </div>
      <div>
        <label className="text-xs text-white/50 block mb-1">刷新间隔 (秒)</label>
        <input
          type="number"
          className="glass-input"
          value={form.refresh_interval_seconds}
          onChange={(e) => setForm((f) => ({ ...f, refresh_interval_seconds: parseInt(e.target.value) }))}
          onBlur={() => commit(form)}
          min={60}
        />
      </div>
    </div>
  );
}
