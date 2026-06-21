import { BalanceInfo } from '../types';
import { formatCurrency } from '../utils/format';
import { RefreshCw } from 'lucide-react';

interface BalanceDisplayProps {
  balance: BalanceInfo | null;
  isLoading?: boolean;
  onRefresh?: () => void;
}

export function BalanceDisplay({ balance, isLoading, onRefresh }: BalanceDisplayProps) {
  if (!balance) {
    return (
      <div className="text-center py-8 text-white/40">
        <p>未配置 API Key</p>
      </div>
    );
  }

  return (
    <div className="text-center py-4">
      <div className="flex items-center justify-center gap-2 mb-2">
        <span className="text-sm text-white/50">账户余额</span>
        {onRefresh && (
          <button
            onClick={onRefresh}
            disabled={isLoading}
            className="glass-button p-1"
            aria-label="刷新余额"
          >
            <RefreshCw className={`w-3 h-3 ${isLoading ? 'animate-spin' : ''}`} />
          </button>
        )}
      </div>
      <div
        className="font-mono text-[36px] font-semibold text-white/90 leading-tight"
        style={{ fontVariantNumeric: 'proportional-nums', letterSpacing: '-0.02em' }}
      >
        {balance.remaining.toFixed(2)}
        <span className="text-[20px] ml-1 font-medium">{balance.currency}</span>
      </div>
      <div className="text-xs text-white/40 mt-1">
        总额 {formatCurrency(balance.total, balance.currency)} · 已用 {formatCurrency(balance.used, balance.currency)}
      </div>
    </div>
  );
}
