import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { tauriInvoke } from '../utils/tauri';

const KEYCHAIN_KEY = 'keychain';

export function useStoreApiKey() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: ({ providerId, apiKey }: { providerId: string; apiKey: string }) =>
      tauriInvoke<void>('store_api_key', { provider_id: providerId, api_key: apiKey }),
    onSuccess: (_, variables) => {
      queryClient.invalidateQueries({ queryKey: [KEYCHAIN_KEY, variables.providerId] });
    },
  });
}

export function useRetrieveApiKey() {
  return useMutation({
    mutationFn: (providerId: string) => tauriInvoke<string>('retrieve_api_key', { provider_id: providerId }),
  });
}

export function useDeleteApiKey() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (providerId: string) => tauriInvoke<void>('delete_api_key', { provider_id: providerId }),
    onSuccess: (_, providerId) => {
      queryClient.invalidateQueries({ queryKey: [KEYCHAIN_KEY, providerId] });
    },
  });
}

export function useHasApiKey(providerId: string) {
  return useQuery({
    queryKey: [KEYCHAIN_KEY, providerId],
    queryFn: () => tauriInvoke<boolean>('has_api_key', { provider_id: providerId }),
    enabled: !!providerId,
  });
}
