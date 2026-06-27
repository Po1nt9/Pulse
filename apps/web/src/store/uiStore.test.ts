import { describe, it, expect, beforeEach } from 'vitest';
import { useUIStore } from './uiStore';

// zustand stores are module-level singletons; reset to a known initial state
// before every case so each test is fully isolated and deterministic.
const INITIAL_STATE = {
  activePanel: 'overview' as const,
  selectedProviderId: null,
  selectedTimeRange: 'today' as const,
  isAddProviderModalOpen: false,
  isSettingsOpen: false,
};

describe('useUIStore', () => {
  beforeEach(() => {
    useUIStore.setState(INITIAL_STATE);
  });

  it('starts in the overview panel with no provider selected and a today range', () => {
    const s = useUIStore.getState();
    expect(s.activePanel).toBe('overview');
    expect(s.selectedProviderId).toBeNull();
    expect(s.selectedTimeRange).toBe('today');
    expect(s.isAddProviderModalOpen).toBe(false);
    expect(s.isSettingsOpen).toBe(false);
  });

  it('setActivePanel updates only the active panel', () => {
    useUIStore.getState().setActivePanel('detail');
    expect(useUIStore.getState().activePanel).toBe('detail');
    // unchanged
    expect(useUIStore.getState().selectedProviderId).toBeNull();
    expect(useUIStore.getState().isSettingsOpen).toBe(false);
  });

  it('selectProvider updates only the selected provider id', () => {
    useUIStore.getState().selectProvider('anthropic');
    expect(useUIStore.getState().selectedProviderId).toBe('anthropic');
    expect(useUIStore.getState().activePanel).toBe('overview');
  });

  it('selectProvider(null) clears the selection', () => {
    useUIStore.getState().selectProvider('openai');
    useUIStore.getState().selectProvider(null);
    expect(useUIStore.getState().selectedProviderId).toBeNull();
  });

  it('setTimeRange updates only the time range', () => {
    useUIStore.getState().setTimeRange('week');
    expect(useUIStore.getState().selectedTimeRange).toBe('week');
    expect(useUIStore.getState().activePanel).toBe('overview');
  });

  it('openAddProviderModal / closeAddProviderModal toggle only the modal flag', () => {
    useUIStore.getState().openAddProviderModal();
    expect(useUIStore.getState().isAddProviderModalOpen).toBe(true);
    expect(useUIStore.getState().activePanel).toBe('overview');

    useUIStore.getState().closeAddProviderModal();
    expect(useUIStore.getState().isAddProviderModalOpen).toBe(false);
  });

  it('openSettings opens settings AND switches to the settings panel', () => {
    useUIStore.getState().setActivePanel('detail');
    useUIStore.getState().openSettings();
    const s = useUIStore.getState();
    expect(s.isSettingsOpen).toBe(true);
    expect(s.activePanel).toBe('settings');
  });

  it('closeSettings closes settings AND returns to the overview panel', () => {
    useUIStore.getState().openSettings();
    useUIStore.getState().closeSettings();
    const s = useUIStore.getState();
    expect(s.isSettingsOpen).toBe(false);
    expect(s.activePanel).toBe('overview');
  });

  it('navigateToDetail sets the detail panel and the selected provider together', () => {
    useUIStore.getState().navigateToDetail('deepseek');
    const s = useUIStore.getState();
    expect(s.activePanel).toBe('detail');
    expect(s.selectedProviderId).toBe('deepseek');
  });

  it('navigateToOverview resets to the overview panel and clears the selection', () => {
    useUIStore.getState().navigateToDetail('deepseek');
    useUIStore.getState().navigateToOverview();
    const s = useUIStore.getState();
    expect(s.activePanel).toBe('overview');
    expect(s.selectedProviderId).toBeNull();
  });

  it('does not leak state between navigation sequences (isolation check)', () => {
    // Full round-trip: detail -> overview -> settings -> close -> detail.
    useUIStore.getState().navigateToDetail('openrouter');
    useUIStore.getState().navigateToOverview();
    useUIStore.getState().openSettings();
    useUIStore.getState().closeSettings();
    useUIStore.getState().navigateToDetail('anthropic');
    const s = useUIStore.getState();
    expect(s.activePanel).toBe('detail');
    expect(s.selectedProviderId).toBe('anthropic');
    expect(s.isSettingsOpen).toBe(false);
    expect(s.isAddProviderModalOpen).toBe(false);
  });
});
