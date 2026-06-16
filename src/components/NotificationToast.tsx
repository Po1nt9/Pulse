import { useToastStore } from '../store/toastStore';
import { X, CheckCircle, AlertCircle, Info } from 'lucide-react';

export function NotificationToast() {
  const toasts = useToastStore((state) => state.toasts);
  const removeToast = useToastStore((state) => state.removeToast);

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
