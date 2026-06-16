import { ProviderBalance } from '../types';
import { StatusIndicator } from './StatusIndicator';
import { formatCurrency } from '../utils/format';
import { ChevronRight } from 'lucide-react';

interface ProviderCardProps {
  provider: ProviderBalance;
  onClick: () => void;
}

export function ProviderCard({ provider, onClick }: ProviderCardProps) {
  const hasError = !!provider.error;
  const hasBalance = !!provider.balance;
  const percentage = provider.balance?.percentage_used || 0;

  return (
    <div
      onClick={onClick}
      onKeyDown={(e) => {
        if (e.key === 'Enter' || e.key === ' ') {
          e.preventDefault();
          onClick();
        }
      }}
      tabIndex={0}
      role="button"
      aria-label={`查看 ${provider.provider_name} 详情`}
      className="flex items-center justify-between p-3 rounded-lg cursor-pointer hover:bg-white/[0.06] transition-colors group focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-accent"
    >
      <div className="flex items-center gap-2">
        <StatusIndicator percentage={percentage} />
        <span className="font-medium text-sm text-white/80">{provider.provider_name}</span>
      </div>
      <div className="flex items-center gap-2">
        {hasBalance && (
          <span
            className="font-mono text-sm text-white/70 group-hover:scale-105 transition-transform"
            style={{ fontVariantNumeric: 'tabular-nums' }}
          >
            {formatCurrency(provider.balance!.remaining, provider.balance!.currency)}
          </span>
        )}
        {hasError && (
          <span className="text-xs text-status-danger">错误</span>
        )}
        {!hasBalance && !hasError && (
          <span className="text-xs text-white/30">未配置</span>
        )}
        <ChevronRight className="w-4 h-4 text-white/20 group-hover:text-white/40 transition-colors" />
      </div>
    </div>
  );
}
