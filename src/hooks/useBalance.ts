import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { ProviderBalance } from '../types';
import { tauriInvoke } from '../utils/tauri';

export const BALANCE_KEY = 'balance';

export function useBalance(providerId: string) {
  return useQuery({
    queryKey: [BALANCE_KEY, providerId],
    queryFn: () => tauriInvoke<ProviderBalance>('get_balance', { provider_id: providerId }),
    enabled: !!providerId,
    staleTime: 60000,
  });
}

export function useRefreshAllBalances() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: () => tauriInvoke<ProviderBalance[]>('refresh_all_balances'),
    onSuccess: (data) => {
      data.forEach((balance) => {
        queryClient.setQueryData([BALANCE_KEY, balance.provider_id], balance);
      });
    },
  });
}
