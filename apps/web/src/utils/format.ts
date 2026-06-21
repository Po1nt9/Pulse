export function formatCurrency(value: number, currency: string = 'USD'): string {
  if (value >= 1000000) {
    return `${(value / 1000000).toFixed(2)}M ${currency}`;
  }
  if (value >= 1000) {
    return `${(value / 1000).toFixed(2)}K ${currency}`;
  }
  return `${value.toFixed(2)} ${currency}`;
}

export function formatPercentage(value: number): string {
  return `${value.toFixed(1)}%`;
}

export function formatNumber(value: number): string {
  if (value >= 1000000) {
    return `${(value / 1000000).toFixed(1)}M`;
  }
  if (value >= 1000) {
    return `${(value / 1000).toFixed(1)}K`;
  }
  return value.toString();
}

export function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  return date.toLocaleDateString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

export function timeRangeLabel(range: string): string {
  const labels: Record<string, string> = {
    recent: '近期',
    today: '今日',
    week: '本周',
    month: '本月',
  };
  return labels[range] || range;
}
