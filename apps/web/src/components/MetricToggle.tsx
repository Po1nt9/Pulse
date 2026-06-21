import { TimeRange } from '../types';
import { timeRangeLabel } from '../utils/format';

interface MetricToggleProps {
  value: TimeRange;
  onChange: (range: TimeRange) => void;
}

const ranges: TimeRange[] = ['recent', 'today', 'week', 'month'];

export function MetricToggle({ value, onChange }: MetricToggleProps) {
  return (
    <div className="metric-toggle" role="tablist" aria-label="时间范围选择">
      {ranges.map((range) => (
        <button
          key={range}
          id={`tab-${range}`}
          role="tab"
          aria-selected={value === range}
          aria-controls={`panel-${range}`}
          tabIndex={value === range ? 0 : -1}
          className={value === range ? 'active' : ''}
          onClick={() => onChange(range)}
        >
          {timeRangeLabel(range)}
        </button>
      ))}
    </div>
  );
}
