import { useQuery } from '@tanstack/react-query';
import { ProviderUsage, TimeRange } from '../types';
import { tauriInvoke } from '../utils/tauri';

const USAGE_KEY = 'usage';

export function useUsage(providerId: string, period: TimeRange) {
  return useQuery({
    queryKey: [USAGE_KEY, providerId, period],
    queryFn: () => tauriInvoke<ProviderUsage>('get_usage', { provider_id: providerId, period }),
    enabled: !!providerId,
    staleTime: 300000,
  });
}
