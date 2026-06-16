export interface BalanceInfo {
  total: number;
  used: number;
  remaining: number;
  currency: string;
  percentage_used: number;
}

export interface UsagePoint {
  timestamp: string;
  cost: number;
  tokens_input: number;
  tokens_output: number;
  requests: number;
}

export interface UsageData {
  points: UsagePoint[];
  total_cost: number;
  total_tokens_input: number;
  total_tokens_output: number;
  total_requests: number;
  period: string;
}

export type ProviderType = 'deepseek' | 'openai' | 'anthropic' | 'openrouter' | 'custom';

export interface ProviderConfig {
  id: string;
  name: string;
  provider_type: ProviderType;
  api_base_url: string;
  display_name: string;
  refresh_interval_seconds: number;
  warning_threshold_percent: number;
  enabled: boolean;
}

export interface ProviderBalance {
  provider_id: string;
  provider_name: string;
  balance: BalanceInfo | null;
  error: string | null;
  last_updated: string | null;
}

export interface ProviderUsage {
  provider_id: string;
  provider_name: string;
  usage: UsageData | null;
  error: string | null;
}

export interface AppSettings {
  theme: string;
  auto_start: boolean;
  global_refresh_interval: number;
  show_notifications: boolean;
  window_position: [number, number] | null;
}

export type TimeRange = 'recent' | 'today' | 'week' | 'month';
export type PanelView = 'overview' | 'detail' | 'settings';

export interface Toast {
  id: string;
  message: string;
  type: 'success' | 'error' | 'info';
}
