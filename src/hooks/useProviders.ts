import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { ProviderConfig } from '../types';
import { tauriInvoke } from '../utils/tauri';
import { BALANCE_KEY } from './useBalance';
import { useToastStore } from '../store/toastStore';

const PROVIDERS_KEY = 'providers';

export function useProviders() {
  return useQuery({
    queryKey: [PROVIDERS_KEY],
    queryFn: () => tauriInvoke<ProviderConfig[]>('list_providers'),
  });
}

export function useAddProvider() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (provider: ProviderConfig) => tauriInvoke<ProviderConfig>('add_provider', { provider }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: [PROVIDERS_KEY] }),
  });
}

export function useUpdateProvider() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, provider }: { id: string; provider: ProviderConfig }) =>
      tauriInvoke<ProviderConfig>('update_provider', { provider_id: id, updates: provider }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: [PROVIDERS_KEY] }),
    onError: (e) => useToastStore.getState().addToast(`更新供应商失败：${e}`, 'error'),
  });
}

export function useDeleteProvider() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => tauriInvoke<void>('delete_provider', { provider_id: id }),
    onSuccess: () => queryClient.invalidateQueries({ queryKey: [PROVIDERS_KEY] }),
  });
}

export function useToggleProvider() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ id, enabled }: { id: string; enabled: boolean }) =>
      tauriInvoke<void>('toggle_provider', { provider_id: id, enabled }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: [PROVIDERS_KEY] });
      queryClient.invalidateQueries({ queryKey: [BALANCE_KEY] });
    },
  });
}
