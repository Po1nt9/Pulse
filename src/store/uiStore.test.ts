import { describe, it, expect, beforeEach } from 'vitest';
import { useUIStore } from './uiStore';

// Zustand stores are module-level singletons. Reset to the documented initial
// state before each test so cases are fully isolated and order-independent.
beforeEach(() => {
  useUIStore.setState({
    activePanel: 'overview',
    selectedProviderId: null,
    selectedTimeRange: 'today',
    isAddProviderModalOpen: false,
    isSettingsOpen: false,
  });
});

describe('useUIStore initial state', () => {
  it('boots into the overview panel with no provider selected', () => {
    const s = useUIStore.getState();
    expect(s.activePanel).toBe('overview');
    expect(s.selectedProviderId).toBeNull();
    expect(s.selectedTimeRange).toBe('today');
    expect(s.isAddProviderModalOpen).toBe(false);
    expect(s.isSettingsOpen).toBe(false);
  });
});

describe('useUIStore panel navigation', () => {
  it('navigateToDetail sets the panel and selects the provider', () => {
    useUIStore.getState().navigateToDetail('openai');

    const s = useUIStore.getState();
    expect(s.activePanel).toBe('detail');
    expect(s.selectedProviderId).toBe('openai');
  });

  it('navigateToOverview resets both panel and selection', () => {
    useUIStore.getState().navigateToDetail('anthropic');
    useUIStore.getState().navigateToOverview();

    const s = useUIStore.getState();
    expect(s.activePanel).toBe('overview');
    expect(s.selectedProviderId).toBeNull();
  });

  it('selectProvider updates only the selection without changing the panel', () => {
    useUIStore.getState().selectProvider('deepseek');

    const s = useUIStore.getState();
    expect(s.selectedProviderId).toBe('deepseek');
    expect(s.activePanel).toBe('overview');
  });

  it('selectProvider accepts null to clear the selection', () => {
    useUIStore.getState().selectProvider('openrouter');
    useUIStore.getState().selectProvider(null);

    expect(useUIStore.getState().selectedProviderId).toBeNull();
  });

  it('setActivePanel switches the active panel', () => {
    useUIStore.getState().setActivePanel('settings');
    expect(useUIStore.getState().activePanel).toBe('settings');
  });
});

describe('useUIStore time range', () => {
  it('setTimeRange updates the selected time range', () => {
    useUIStore.getState().setTimeRange('week');
    expect(useUIStore.getState().selectedTimeRange).toBe('week');
  });
});

describe('useUIStore add-provider modal', () => {
  it('openAddProviderModal opens the modal without touching the panel', () => {
    useUIStore.getState().setActivePanel('detail');
    useUIStore.getState().openAddProviderModal();

    const s = useUIStore.getState();
    expect(s.isAddProviderModalOpen).toBe(true);
    expect(s.activePanel).toBe('detail');
  });

  it('closeAddProviderModal closes the modal', () => {
    useUIStore.getState().openAddProviderModal();
    useUIStore.getState().closeAddProviderModal();

    expect(useUIStore.getState().isAddProviderModalOpen).toBe(false);
  });
});

describe('useUIStore settings panel', () => {
  it('openSettings opens settings and switches the panel to settings', () => {
    useUIStore.getState().navigateToDetail('openai');
    useUIStore.getState().openSettings();

    const s = useUIStore.getState();
    expect(s.isSettingsOpen).toBe(true);
    expect(s.activePanel).toBe('settings');
  });

  it('closeSettings closes settings and returns to the overview panel', () => {
    useUIStore.getState().openSettings();
    useUIStore.getState().closeSettings();

    const s = useUIStore.getState();
    expect(s.isSettingsOpen).toBe(false);
    expect(s.activePanel).toBe('overview');
  });
});
