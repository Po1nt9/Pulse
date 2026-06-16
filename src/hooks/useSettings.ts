import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { AppSettings } from '../types';
import { tauriInvoke } from '../utils/tauri';

const SETTINGS_KEY = 'settings';

export function useSettings() {
  return useQuery({
    queryKey: [SETTINGS_KEY],
    queryFn: () => tauriInvoke<AppSettings>('get_settings'),
  });
}

export function useUpdateSettings() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (settings: AppSettings) => tauriInvoke<AppSettings>('update_settings', { settings }),
    onSuccess: (data) => {
      queryClient.setQueryData([SETTINGS_KEY], data);
    },
  });
}
