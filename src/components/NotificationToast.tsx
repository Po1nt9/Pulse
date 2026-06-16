import { useEffect } from 'react';
import { useToastStore } from '../store/toastStore';
import { X, CheckCircle, AlertCircle, Info } from 'lucide-react';

const TOAST_LIFETIME_MS = 4000;
const SWEEP_INTERVAL_MS = 500;
const toastAddedAt = new Map<string, number>();

export function NotificationToast() {
  const toasts = useToastStore((state) => state.toasts);
  const removeToast = useToastStore((state) => state.removeToast);

  useEffect(() => {
    const now = Date.now();
    toasts.forEach((t) => {
      if (!toastAddedAt.has(t.id)) toastAddedAt.set(t.id, now);
    });
    const expired = toasts
      .filter((t) => now - (toastAddedAt.get(t.id) ?? now) >= TOAST_LIFETIME_MS)
      .map((t) => t.id);
    expired.forEach((id) => {
      toastAddedAt.delete(id);
      removeToast(id);
    });
  }, [toasts, removeToast]);

  useEffect(() => {
    const timer = setInterval(() => {
      const now = Date.now();
      const current = useToastStore.getState().toasts;
      current.forEach((t) => {
        if (!toastAddedAt.has(t.id)) toastAddedAt.set(t.id, now);
      });
      const expired = current
        .filter((t) => now - (toastAddedAt.get(t.id) ?? now) >= TOAST_LIFETIME_MS)
        .map((t) => t.id);
      expired.forEach((id) => {
        toastAddedAt.delete(id);
        useToastStore.getState().removeToast(id);
      });
    }, SWEEP_INTERVAL_MS);
    return () => clearInterval(timer);
  }, []);

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
