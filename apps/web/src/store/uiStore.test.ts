import { describe, it, expect, beforeEach, vi } from 'vitest';
import type { useUIStore as UseUIStore } from './uiStore';

// The UI store holds navigation state. Several actions perform compound
// transitions (setting both the active panel and the selected provider, or
// toggling the settings modal while resetting the panel). A regression that
// updates only one field would leave the UI in an inconsistent state, so
// these transitions are the primary regression target. Each test re-imports
// the module for a fresh, isolated store.

describe('useUIStore', () => {
  let useUIStore: typeof UseUIStore;

  beforeEach(async () => {
    vi.resetModules();
    ({ useUIStore } = await import('./uiStore'));
  });

  it('starts on the overview panel with nothing selected', () => {
    const s = useUIStore.getState();
    expect(s.activePanel).toBe('overview');
    expect(s.selectedProviderId).toBeNull();
    expect(s.selectedTimeRange).toBe('today');
    expect(s.isAddProviderModalOpen).toBe(false);
    expect(s.isSettingsOpen).toBe(false);
  });

  it('navigateToDetail sets both the detail panel and the selected provider', () => {
    useUIStore.getState().navigateToDetail('provider-42');

    const s = useUIStore.getState();
    expect(s.activePanel).toBe('detail');
    expect(s.selectedProviderId).toBe('provider-42');
  });

  it('navigateToOverview resets both the panel and the selection', () => {
    useUIStore.getState().navigateToDetail('provider-42');
    useUIStore.getState().navigateToOverview();

    const s = useUIStore.getState();
    expect(s.activePanel).toBe('overview');
    expect(s.selectedProviderId).toBeNull();
  });

  it('openSettings opens the modal and switches to the settings panel', () => {
    useUIStore.getState().openSettings();

    const s = useUIStore.getState();
    expect(s.isSettingsOpen).toBe(true);
    expect(s.activePanel).toBe('settings');
  });

  it('closeSettings closes the modal and returns to the overview panel', () => {
    useUIStore.getState().openSettings();
    useUIStore.getState().closeSettings();

    const s = useUIStore.getState();
    expect(s.isSettingsOpen).toBe(false);
    expect(s.activePanel).toBe('overview');
  });

  it('openAddProviderModal / closeAddProviderModal toggle only the modal flag', () => {
    useUIStore.getState().openAddProviderModal();
    expect(useUIStore.getState().isAddProviderModalOpen).toBe(true);

    useUIStore.getState().closeAddProviderModal();
    expect(useUIStore.getState().isAddProviderModalOpen).toBe(false);
  });

  it('setTimeRange / selectProvider / setActivePanel update their fields independently', () => {
    useUIStore.getState().setTimeRange('week');
    useUIStore.getState().selectProvider('abc');
    useUIStore.getState().setActivePanel('detail');

    const s = useUIStore.getState();
    expect(s.selectedTimeRange).toBe('week');
    expect(s.selectedProviderId).toBe('abc');
    expect(s.activePanel).toBe('detail');
  });
});
