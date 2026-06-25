import { describe, it, expect, beforeEach } from 'vitest';
import { useToastStore } from './toastStore';

describe('useToastStore', () => {
  beforeEach(() => {
    useToastStore.setState({ toasts: [] });
  });

  it('starts with no toasts', () => {
    expect(useToastStore.getState().toasts).toEqual([]);
  });

  it('addToast returns an id and appends a toast with message and type', () => {
    const id = useToastStore.getState().addToast('saved', 'success');
    expect(typeof id).toBe('string');
    expect(id.length).toBeGreaterThan(0);

    const toasts = useToastStore.getState().toasts;
    expect(toasts).toHaveLength(1);
    expect(toasts[0]).toEqual({ id, message: 'saved', type: 'success' });
  });

  it('addToast generates unique ids across calls', () => {
    const id1 = useToastStore.getState().addToast('a', 'info');
    const id2 = useToastStore.getState().addToast('b', 'error');
    expect(id1).not.toBe(id2);
    expect(useToastStore.getState().toasts).toHaveLength(2);
  });

  it('preserves insertion order across multiple additions', () => {
    useToastStore.getState().addToast('first', 'info');
    useToastStore.getState().addToast('second', 'info');
    useToastStore.getState().addToast('third', 'info');
    const messages = useToastStore.getState().toasts.map((t) => t.message);
    expect(messages).toEqual(['first', 'second', 'third']);
  });

  it('removeToast removes the toast matching the returned id', () => {
    const id = useToastStore.getState().addToast('hello', 'success');
    useToastStore.getState().removeToast(id);
    expect(useToastStore.getState().toasts).toEqual([]);
  });

  it('removeToast only removes the targeted toast', () => {
    const keepId = useToastStore.getState().addToast('keep', 'info');
    const removeId = useToastStore.getState().addToast('drop', 'error');
    useToastStore.getState().removeToast(removeId);
    const toasts = useToastStore.getState().toasts;
    expect(toasts).toHaveLength(1);
    expect(toasts[0].id).toBe(keepId);
  });

  it('removeToast with an unknown id leaves toasts unchanged', () => {
    useToastStore.getState().addToast('one', 'info');
    useToastStore.getState().removeToast('does-not-exist');
    expect(useToastStore.getState().toasts).toHaveLength(1);
  });
});
