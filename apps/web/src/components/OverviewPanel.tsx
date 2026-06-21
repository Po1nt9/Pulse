import { useEffect } from 'react';
import { useQuery } from '@tanstack/react-query';
import { useProviders } from '../hooks/useProviders';
import { useRefreshAllBalances } from '../hooks/useBalance';
import { useUIStore } from '../store/uiStore';
import { ProviderBalance, ProviderConfig } from '../types';
import { ProviderCard } from './ProviderCard';
import { GlassPanel } from './GlassPanel';
import { RefreshCw, Settings, Plus } from 'lucide-react';

function ProviderRow({
  provider,
  onClick,
}: {
  provider: ProviderConfig;
  onClick: () => void;
}) {
  // 订阅 balance 缓存但不主动 fetch；refresh_all_balances 的 setQueryData 会触发重渲染。
  const { data: balance } = useQuery<ProviderBalance | undefined>({
    queryKey: ['balance', provider.id],
    queryFn: () => undefined,
    enabled: false,
    staleTime: Infinity,
  });

  const providerBalance: ProviderBalance =
    balance ?? {
      provider_id: provider.id,
      provider_name: provider.display_name || provider.name,
      balance: null,
      error: null,
      last_updated: null,
    };
  return <ProviderCard provider={providerBalance} onClick={onClick} />;
}

export function OverviewPanel() {
  const { data: providers, isLoading } = useProviders();
  const refreshMutation = useRefreshAllBalances();
  const navigateToDetail = useUIStore((s) => s.navigateToDetail);
  const openSettings = useUIStore((s) => s.openSettings);
  const openAddProviderModal = useUIStore((s) => s.openAddProviderModal);

  useEffect(() => {
    if (providers && providers.length > 0) {
      refreshMutation.mutate();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [providers?.length]);

  const handleRefresh = () => {
    refreshMutation.mutate();
  };

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between p-4">
        <div>
          <h1 className="text-lg font-semibold text-white/90">Pulse</h1>
          <p className="text-[11px] uppercase tracking-[0.08em] text-white/40">所有供应商</p>
        </div>
        <div className="flex gap-1">
          <button onClick={handleRefresh} className="glass-button p-2" disabled={refreshMutation.isPending} aria-label="刷新">
            <RefreshCw className={`w-4 h-4 ${refreshMutation.isPending ? 'animate-spin' : ''}`} />
          </button>
          <button onClick={openAddProviderModal} className="glass-button p-2" aria-label="添加供应商">
            <Plus className="w-4 h-4" />
          </button>
          <button onClick={openSettings} className="glass-button p-2" aria-label="设置">
            <Settings className="w-4 h-4" />
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto px-4 pb-4">
        <GlassPanel padding="sm">
          {isLoading && (
            <div className="text-center py-8 text-white/40">加载中...</div>
          )}

          {providers?.map((provider) => (
            <ProviderRow
              key={provider.id}
              provider={provider}
              onClick={() => navigateToDetail(provider.id)}
            />
          ))}

          {providers?.length === 0 && (
            <div className="text-center py-8">
              <p className="text-white/40 mb-3">暂无供应商</p>
              <button onClick={openAddProviderModal} className="glass-button">
                <Plus className="w-4 h-4 inline mr-1" />
                添加供应商
              </button>
            </div>
          )}
        </GlassPanel>
      </div>
    </div>
  );
}
