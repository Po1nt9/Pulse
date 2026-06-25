import { describe, it, expect, beforeEach } from 'vitest';
import { useUIStore } from './uiStore';
import type { PanelView, TimeRange } from '../types';

const initialState = {
  activePanel: 'overview' as PanelView,
  selectedProviderId: null,
  selectedTimeRange: 'today' as TimeRange,
  isAddProviderModalOpen: false,
  isSettingsOpen: false,
};

describe('useUIStore', () => {
  beforeEach(() => {
    useUIStore.setState(initialState);
  });

  it('starts on the overview panel with no provider selected', () => {
    const s = useUIStore.getState();
    expect(s.activePanel).toBe('overview');
    expect(s.selectedProviderId).toBeNull();
    expect(s.selectedTimeRange).toBe('today');
    expect(s.isAddProviderModalOpen).toBe(false);
    expect(s.isSettingsOpen).toBe(false);
  });

  it('setActivePanel switches the active panel', () => {
    useUIStore.getState().setActivePanel('settings');
    expect(useUIStore.getState().activePanel).toBe('settings');
  });

  it('selectProvider updates the selected provider id', () => {
    useUIStore.getState().selectProvider('deepseek');
    expect(useUIStore.getState().selectedProviderId).toBe('deepseek');
  });

  it('setTimeRange updates the selected time range', () => {
    useUIStore.getState().setTimeRange('week');
    expect(useUIStore.getState().selectedTimeRange).toBe('week');
  });

  it('openAddProviderModal / closeAddProviderModal toggle the modal flag', () => {
    useUIStore.getState().openAddProviderModal();
    expect(useUIStore.getState().isAddProviderModalOpen).toBe(true);
    useUIStore.getState().closeAddProviderModal();
    expect(useUIStore.getState().isAddProviderModalOpen).toBe(false);
  });

  it('openSettings opens settings and switches panel to settings', () => {
    useUIStore.getState().openAddProviderModal();
    useUIStore.getState().openSettings();
    const s = useUIStore.getState();
    expect(s.isSettingsOpen).toBe(true);
    expect(s.activePanel).toBe('settings');
  });

  it('closeSettings closes settings and resets panel to overview', () => {
    useUIStore.getState().openSettings();
    useUIStore.getState().closeSettings();
    const s = useUIStore.getState();
    expect(s.isSettingsOpen).toBe(false);
    expect(s.activePanel).toBe('overview');
  });

  it('navigateToDetail sets the detail panel and selects the provider', () => {
    useUIStore.getState().navigateToDetail('anthropic');
    const s = useUIStore.getState();
    expect(s.activePanel).toBe('detail');
    expect(s.selectedProviderId).toBe('anthropic');
  });

  it('navigateToOverview resets to overview and clears provider selection', () => {
    useUIStore.getState().navigateToDetail('openai');
    useUIStore.getState().navigateToOverview();
    const s = useUIStore.getState();
    expect(s.activePanel).toBe('overview');
    expect(s.selectedProviderId).toBeNull();
  });

  it('clears provider selection when navigating back to overview after detail', () => {
    useUIStore.getState().navigateToDetail('openrouter');
    expect(useUIStore.getState().selectedProviderId).toBe('openrouter');
    useUIStore.getState().navigateToOverview();
    expect(useUIStore.getState().selectedProviderId).toBeNull();
    expect(useUIStore.getState().activePanel).toBe('overview');
  });
});
