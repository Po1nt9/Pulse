import React, { useState, useCallback } from 'react';
import { useUIStore } from '../store/uiStore';
import { useAddProvider } from '../hooks/useProviders';
import { useStoreApiKey } from '../hooks/useKeychain';
import { ProviderConfig } from '../types';
import { GlassPanel } from './GlassPanel';
import { X } from 'lucide-react';

const PROVIDER_TYPES = [
  { value: 'deepseek' as const, label: 'DeepSeek', url: 'https://api.deepseek.com' },
  { value: 'openai' as const, label: 'OpenAI', url: 'https://api.openai.com' },
  { value: 'anthropic' as const, label: 'Anthropic', url: 'https://api.anthropic.com' },
  { value: 'openrouter' as const, label: 'OpenRouter', url: 'https://openrouter.ai/api' },
  { value: 'custom' as const, label: '自定义', url: '' },
];

export function AddProviderModal() {
  const { closeAddProviderModal } = useUIStore();
  const addProvider = useAddProvider();
  const storeKey = useStoreApiKey();

  const [type, setType] = useState<ProviderConfig['provider_type']>('deepseek');
  const [name, setName] = useState('');
  const [url, setUrl] = useState('');
  const [apiKey, setApiKey] = useState('');
  const [errors, setErrors] = useState<string[]>([]);

  const selectedType = PROVIDER_TYPES.find((t) => t.value === type);

  const validate = useCallback(() => {
    const newErrors: string[] = [];
    if (type === 'custom' && !url) {
      newErrors.push('自定义供应商需要填写 API Base URL');
    }
    if (apiKey && !apiKey.startsWith('sk-') && !apiKey.startsWith('Bearer ')) {
      newErrors.push('API Key 格式可能不正确');
    }
    setErrors(newErrors);
    return newErrors.length === 0;
  }, [type, url, apiKey]);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;

    const id = `provider-${Date.now()}`;
    const provider: ProviderConfig = {
      id,
      name: name || selectedType?.label || 'Custom',
      provider_type: type,
      api_base_url: url || selectedType?.url || '',
      display_name: name || selectedType?.label || 'Custom',
      refresh_interval_seconds: 300,
      warning_threshold_percent: 30,
      enabled: true,
    };

    try {
      await addProvider.mutateAsync(provider);
      if (apiKey) {
        await storeKey.mutateAsync({ providerId: id, apiKey });
      }
      closeAddProviderModal();
    } catch (error) {
      console.error('Failed to add provider:', error);
      setErrors(['添加供应商失败，请检查输入']);
    }
  };

  const handleClose = useCallback(() => {
    closeAddProviderModal();
  }, [closeAddProviderModal]);

  // Close on Escape
  React.useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') handleClose();
    };
    document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [handleClose]);

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-50"
      onClick={handleClose}
    >
      <GlassPanel className="w-80 max-w-full mx-4" padding="lg" onClick={(e) => e.stopPropagation()}>
        <div className="flex items-center justify-between mb-4">
          <h3 className="text-lg font-semibold text-white/90">添加供应商</h3>
          <button onClick={handleClose} className="glass-button p-1" aria-label="关闭">
            <X className="w-4 h-4" />
          </button>
        </div>

        {errors.length > 0 && (
          <div className="mb-4 p-3 rounded-lg bg-status-danger/10 border border-status-danger/30">
            {errors.map((err, i) => (
              <p key={i} className="text-sm text-status-danger">{err}</p>
            ))}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="text-xs text-white/50 block mb-1">类型</label>
            <select
              className="glass-input"
              value={type}
              onChange={(e) => {
                const newType = e.target.value as ProviderConfig['provider_type'];
                setType(newType);
                const t = PROVIDER_TYPES.find((pt) => pt.value === newType);
                if (t?.url) setUrl(t.url);
              }}
            >
              {PROVIDER_TYPES.map((t) => (
                <option key={t.value} value={t.value}>{t.label}</option>
              ))}
            </select>
          </div>

          <div>
            <label className="text-xs text-white/50 block mb-1">显示名称</label>
            <input
              type="text"
              className="glass-input"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder={selectedType?.label}
            />
          </div>

          <div>
            <label className="text-xs text-white/50 block mb-1">API Base URL</label>
            <input
              type="text"
              className="glass-input"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              placeholder={selectedType?.url || 'https://api.example.com'}
            />
          </div>

          <div>
            <label className="text-xs text-white/50 block mb-1">API Key</label>
            <input
              type="password"
              className="glass-input"
              value={apiKey}
              onChange={(e) => setApiKey(e.target.value)}
              placeholder="sk-..."
            />
          </div>

          <button
            type="submit"
            className="w-full glass-button py-2.5 font-medium"
            disabled={addProvider.isPending}
          >
            {addProvider.isPending ? '添加中...' : '添加'}
          </button>
        </form>
      </GlassPanel>
    </div>
  );
}
