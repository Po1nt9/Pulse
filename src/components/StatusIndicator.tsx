import { getStatusDotClass } from '../utils/colors';

interface StatusIndicatorProps {
  percentage: number;
  size?: 'sm' | 'md';
}

export function StatusIndicator({ percentage, size = 'sm' }: StatusIndicatorProps) {
  const sizeClasses = {
    sm: 'w-2 h-2',
    md: 'w-2.5 h-2.5',
  };

  return (
    <span
      className={`inline-block rounded-full ${sizeClasses[size]} ${getStatusDotClass(percentage)}`}
      style={{ boxShadow: `0 0 4px currentColor` }}
      title={`已用 ${percentage.toFixed(1)}%`}
    />
  );
}
