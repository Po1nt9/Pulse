import { describe, it, expect, beforeEach } from 'vitest';
import { useToastStore } from './toastStore';

// Zustand stores are module-level singletons. Reset the toasts slice before
// each test so cases are fully isolated and order-independent.
beforeEach(() => {
  useToastStore.setState({ toasts: [] });
});

describe('useToastStore.addToast', () => {
  it('starts with an empty toast list', () => {
    expect(useToastStore.getState().toasts).toEqual([]);
  });

  it('appends a toast and returns its id', () => {
    const id = useToastStore.getState().addToast('Saved', 'success');

    expect(typeof id).toBe('string');
    expect(id.startsWith('toast-')).toBe(true);

    const { toasts } = useToastStore.getState();
    expect(toasts).toHaveLength(1);
    expect(toasts[0]).toEqual({ id, message: 'Saved', type: 'success' });
  });

  it('preserves the returned id on the appended toast', () => {
    // The id returned by addToast must match the toast actually added,
    // otherwise removeToast (which keys on id) would silently no-op.
    const id = useToastStore.getState().addToast('Failed', 'error');
    expect(useToastStore.getState().toasts[0].id).toBe(id);
  });

  it('appends in order when multiple toasts are added', () => {
    useToastStore.getState().addToast('first', 'info');
    useToastStore.getState().addToast('second', 'success');

    const { toasts } = useToastStore.getState();
    expect(toasts.map((t) => t.message)).toEqual(['first', 'second']);
  });

  it('accepts every toast type', () => {
    useToastStore.getState().addToast('s', 'success');
    useToastStore.getState().addToast('e', 'error');
    useToastStore.getState().addToast('i', 'info');

    const types = useToastStore.getState().toasts.map((t) => t.type);
    expect(types).toEqual(['success', 'error', 'info']);
  });
});

describe('useToastStore.removeToast', () => {
  it('removes only the toast matching the given id', () => {
    const keepId = useToastStore.getState().addToast('keep', 'info');
    const dropId = useToastStore.getState().addToast('drop', 'error');

    useToastStore.getState().removeToast(dropId);

    const { toasts } = useToastStore.getState();
    expect(toasts).toHaveLength(1);
    expect(toasts[0].id).toBe(keepId);
  });

  it('is a no-op for an unknown id', () => {
    useToastStore.getState().addToast('one', 'info');

    useToastStore.getState().removeToast('toast-does-not-exist');

    expect(useToastStore.getState().toasts).toHaveLength(1);
  });

  it('leaves an empty list untouched', () => {
    useToastStore.getState().removeToast('toast-anything');
    expect(useToastStore.getState().toasts).toEqual([]);
  });
});
