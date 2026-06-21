import { useMemo } from 'react';
import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer, Cell } from 'recharts';
import { UsageData } from '../types';

interface UsageChartProps {
  usage: UsageData | null;
}

export function UsageChart({ usage }: UsageChartProps) {
  const data = useMemo(() => {
    if (!usage || usage.points.length === 0) return [];
    return usage.points.map((point, index) => ({
      name: new Date(point.timestamp).toLocaleDateString('zh-CN', { month: 'short', day: 'numeric' }),
      cost: point.cost,
      tokens: point.tokens_input + point.tokens_output,
      isLast: index === usage.points.length - 1,
    }));
  }, [usage]);

  if (data.length === 0) {
    return (
      <div className="h-20 flex flex-col items-center justify-center text-white/30 text-xs gap-1">
        <span>暂无用量数据</span>
        <span className="text-white/20">部分供应商暂不支持用量查询</span>
      </div>
    );
  }

  return (
    <div className="h-20">
      <ResponsiveContainer width="100%" height="100%">
        <BarChart data={data} margin={{ top: 5, right: 5, bottom: 5, left: -20 }}>
          <XAxis
            dataKey="name"
            axisLine={false}
            tickLine={false}
            tick={{ fill: 'rgba(255,255,255,0.3)', fontSize: 11 }}
          />
          <YAxis
            axisLine={false}
            tickLine={false}
            tick={{ fill: 'rgba(255,255,255,0.3)', fontSize: 11 }}
          />
          <Tooltip
            contentStyle={{
              background: 'rgba(20, 24, 35, 0.95)',
              border: '1px solid rgba(255,255,255,0.1)',
              borderRadius: '8px',
              fontSize: '12px',
            }}
            labelStyle={{ color: 'rgba(255,255,255,0.5)' }}
          />
          <Bar dataKey="cost" radius={[6, 6, 0, 0]}>
            {data.map((entry, index) => (
              <Cell
                key={index}
                fill={entry.isLast ? '#0EA5E9' : 'rgba(14, 165, 233, 0.4)'}
              />
            ))}
          </Bar>
        </BarChart>
      </ResponsiveContainer>
    </div>
  );
}
