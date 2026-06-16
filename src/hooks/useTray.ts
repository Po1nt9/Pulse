import { useEffect, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useRefreshAllBalances } from './useBalance';
import { useSettings } from './useSettings';

export function useTray() {
  const refreshMutation = useRefreshAllBalances();
  const { data: settings } = useSettings();
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  // Listen for tray "refresh" menu click
  const refreshMutationRef = useRef(refreshMutation);

  useEffect(() => {
    refreshMutationRef.current = refreshMutation;
  }, [refreshMutation]);

  useEffect(() => {
    const unlisten = listen('refresh-requested', () => {
      refreshMutationRef.current.mutate();
    });

    return () => {
      unlisten.then((f) => f());
    };
  }, []);

  // Auto-refresh on interval from settings
  useEffect(() => {
    const intervalMs = (settings?.global_refresh_interval ?? 300) * 1000;

    // Clear previous timer
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }

    intervalRef.current = setInterval(() => {
      refreshMutationRef.current.mutate();
    }, intervalMs);

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
      }
    };
  }, [settings?.global_refresh_interval]);
}
