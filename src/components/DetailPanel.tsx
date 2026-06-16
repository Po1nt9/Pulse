import { useEffect } from 'react';
import { useUIStore } from '../store/uiStore';
import { useBalance } from '../hooks/useBalance';
import { useUsage } from '../hooks/useUsage';
import { BalanceDisplay } from './BalanceDisplay';
import { UsageChart } from './UsageChart';
import { MetricToggle } from './MetricToggle';
import { ProgressBar } from './ProgressBar';
import { GlassPanel } from './GlassPanel';
import { ArrowLeft, Settings } from 'lucide-react';

export function DetailPanel() {
  const selectedProviderId = useUIStore((state) => state.selectedProviderId);
  const selectedTimeRange = useUIStore((state) => state.selectedTimeRange);
  const setTimeRange = useUIStore((state) => state.setTimeRange);
  const navigateToOverview = useUIStore((state) => state.navigateToOverview);
  const openSettings = useUIStore((state) => state.openSettings);

  const { data: balance, isLoading: balanceLoading } = useBalance(selectedProviderId || '');
  const { data: usage } = useUsage(selectedProviderId || '', selectedTimeRange);

  useEffect(() => {
    if (!selectedProviderId) {
      navigateToOverview();
    }
  }, [selectedProviderId, navigateToOverview]);

  if (!selectedProviderId) return null;

  return (
    <div className="h-full flex flex-col animate-popup-in">
      {/* Header */}
      <div className="flex items-center justify-between p-4">
        <div className="flex items-center gap-2">
          <button onClick={navigateToOverview} className="glass-button p-2" aria-label="返回">
            <ArrowLeft className="w-4 h-4" />
          </button>
          <h2 className="text-lg font-semibold text-white/90">
            {balance?.provider_name || '供应商详情'}
          </h2>
        </div>
        <button onClick={openSettings} className="glass-button p-2" aria-label="设置">
          <Settings className="w-4 h-4" />
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-4 pb-4 space-y-4">
        {/* Balance */}
        <GlassPanel>
          <BalanceDisplay
            balance={balance?.balance || null}
            isLoading={balanceLoading}
          />
          {balance?.balance && (
            <div className="mt-4">
              <ProgressBar percentage={balance.balance.percentage_used} />
            </div>
          )}
        </GlassPanel>

        {/* Usage Chart */}
        <GlassPanel>
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-sm font-medium text-white/70">用量趋势</h3>
            <MetricToggle value={selectedTimeRange} onChange={setTimeRange} />
          </div>
          <UsageChart usage={usage?.usage || null} />
        </GlassPanel>

        {/* Error */}
        {balance?.error && (
          <GlassPanel className="border-status-danger/30">
            <p className="text-sm text-status-danger">{balance.error}</p>
          </GlassPanel>
        )}
      </div>

      {/* Footer */}
      <div className="p-4 border-t border-white/5">
        <div className="flex items-center justify-between text-xs text-white/30">
          <span>
            {balance?.last_updated
              ? `${new Date(balance.last_updated).toLocaleString('zh-CN')} 更新`
              : '未更新'}
          </span>
          <span>{balance?.balance ? '数据正常' : balance?.error || '等待配置'}</span>
        </div>
      </div>
    </div>
  );
}
