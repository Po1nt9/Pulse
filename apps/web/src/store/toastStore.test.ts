import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';

// The toast store generates an id from Date.now() + Math.random() and returns
// it so callers can later dismiss the toast. Mocking both keeps the ids
// deterministic. Each test re-imports the module (vi.resetModules) for a
// fresh, isolated store instance.

describe('useToastStore', () => {
  let useToastStore: typeof import('./toastStore').useToastStore;

  beforeEach(async () => {
    vi.resetModules();
    vi.spyOn(Date, 'now').mockReturnValue(1700000000000);
    ({ useToastStore } = await import('./toastStore'));
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it('starts with no toasts', () => {
    expect(useToastStore.getState().toasts).toEqual([]);
  });

  it('adds a toast and returns its deterministic id', () => {
    vi.spyOn(Math, 'random').mockReturnValue(0.5);
    const id = useToastStore.getState().addToast('hello', 'success');

    expect(id).toBe(`toast-1700000000000-${(0.5).toString(36).slice(2, 11)}`);
    const { toasts } = useToastStore.getState();
    expect(toasts).toHaveLength(1);
    expect(toasts[0]).toEqual({ id, message: 'hello', type: 'success' });
  });

  it('removes only the toast matching the given id', () => {
    vi.spyOn(Math, 'random')
      .mockReturnValueOnce(0.1)
      .mockReturnValueOnce(0.2);

    const id1 = useToastStore.getState().addToast('first', 'info');
    const id2 = useToastStore.getState().addToast('second', 'error');
    expect(id1).not.toBe(id2);

    useToastStore.getState().removeToast(id1);

    const { toasts } = useToastStore.getState();
    expect(toasts).toHaveLength(1);
    expect(toasts[0].id).toBe(id2);
    expect(toasts[0].message).toBe('second');
  });

  it('is a no-op when removing an unknown id', () => {
    vi.spyOn(Math, 'random').mockReturnValueOnce(0.3);
    useToastStore.getState().addToast('keep', 'success');

    useToastStore.getState().removeToast('does-not-exist');

    expect(useToastStore.getState().toasts).toHaveLength(1);
    expect(useToastStore.getState().toasts[0].message).toBe('keep');
  });
});
