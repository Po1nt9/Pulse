import { useEffect, useRef } from 'react';
import { useToastStore } from '../store/toastStore';
import { X, CheckCircle, AlertCircle, Info } from 'lucide-react';

const TOAST_LIFETIME_MS = 4000;
const SWEEP_INTERVAL_MS = 500;

export function NotificationToast() {
  const toasts = useToastStore((state) => state.toasts);
  const removeToast = useToastStore((state) => state.removeToast);
  const toastAddedAt = useRef<Map<string, number>>(new Map());

  useEffect(() => {
    const addedAtMap = toastAddedAt.current;
    
    const timer = setInterval(() => {
      const now = Date.now();
      const currentToasts = useToastStore.getState().toasts;

      // 记录新 toast 的添加时间
      currentToasts.forEach((t) => {
        if (!addedAtMap.has(t.id)) {
          addedAtMap.set(t.id, now);
        }
      });

      // 清理过期的 toast
      const expired = currentToasts
        .filter((t) => now - (addedAtMap.get(t.id) ?? now) >= TOAST_LIFETIME_MS)
        .map((t) => t.id);

      expired.forEach((id) => {
        addedAtMap.delete(id);
        useToastStore.getState().removeToast(id);
      });
    }, SWEEP_INTERVAL_MS);

    return () => {
      clearInterval(timer);
      // 清理已移除的 toast 的时间记录
      const currentToasts = useToastStore.getState().toasts;
      const currentIds = new Set(currentToasts.map((t) => t.id));
      addedAtMap.forEach((_, id) => {
        if (!currentIds.has(id)) {
          addedAtMap.delete(id);
        }
      });
    };
  }, [removeToast]);

  if (toasts.length === 0) return null;

  return (
    <div className="fixed bottom-4 left-4 right-4 flex flex-col gap-2 z-50" aria-live="polite" aria-atomic="true">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={`glass-panel px-4 py-3 flex items-center gap-3 animate-popup-in ${
            toast.type === 'error' ? 'border-status-danger/30' : ''
          }`}
        >
          {toast.type === 'success' && <CheckCircle className="w-4 h-4 text-status-ok" />}
          {toast.type === 'error' && <AlertCircle className="w-4 h-4 text-status-danger" />}
          {toast.type === 'info' && <Info className="w-4 h-4 text-accent" />}
          <span className="text-sm text-white/80 flex-1">{toast.message}</span>
          <button
            onClick={() => removeToast(toast.id)}
            className="text-white/40 hover:text-white/60"
            aria-label="关闭通知"
          >
            <X className="w-4 h-4" />
          </button>
        </div>
      ))}
    </div>
  );
}
