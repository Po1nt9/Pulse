
interface ProgressBarProps {
  percentage: number;
  showLabel?: boolean;
}

export function ProgressBar({ percentage, showLabel = true }: ProgressBarProps) {
  const clampedPercentage = Math.min(Math.max(percentage, 0), 100);
  const isDanger = clampedPercentage >= 90;
  const isWarning = clampedPercentage >= 70 && !isDanger;

  return (
    <div className="w-full">
      <div className="progress-bar">
        <div
          className={`progress-bar-fill ${isDanger ? 'danger' : ''} ${isWarning ? 'warning' : ''}`}
          style={{
            width: `${clampedPercentage}%`,
          }}
        />
      </div>
      {showLabel && (
        <div className="flex justify-between mt-1 text-xs text-white/40">
          <span>已用 {clampedPercentage.toFixed(1)}%</span>
          <span>{(100 - clampedPercentage).toFixed(1)}% 剩余</span>
        </div>
      )}
    </div>
  );
}
