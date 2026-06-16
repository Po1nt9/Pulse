import { create } from 'zustand';
import { PanelView, TimeRange } from '../types';

interface UIState {
  activePanel: PanelView;
  selectedProviderId: string | null;
  selectedTimeRange: TimeRange;
  isAddProviderModalOpen: boolean;
  isSettingsOpen: boolean;

  setActivePanel: (panel: PanelView) => void;
  selectProvider: (id: string | null) => void;
  setTimeRange: (range: TimeRange) => void;
  openAddProviderModal: () => void;
  closeAddProviderModal: () => void;
  openSettings: () => void;
  closeSettings: () => void;
  navigateToDetail: (providerId: string) => void;
  navigateToOverview: () => void;
}

export const useUIStore = create<UIState>((set) => ({
  activePanel: 'overview',
  selectedProviderId: null,
  selectedTimeRange: 'today',
  isAddProviderModalOpen: false,
  isSettingsOpen: false,

  setActivePanel: (panel) => set({ activePanel: panel }),
  selectProvider: (id) => set({ selectedProviderId: id }),
  setTimeRange: (range) => set({ selectedTimeRange: range }),
  openAddProviderModal: () => set({ isAddProviderModalOpen: true }),
  closeAddProviderModal: () => set({ isAddProviderModalOpen: false }),
  openSettings: () => set({ isSettingsOpen: true, activePanel: 'settings' }),
  closeSettings: () => set({ isSettingsOpen: false, activePanel: 'overview' }),
  navigateToDetail: (providerId) => set({
    activePanel: 'detail',
    selectedProviderId: providerId,
  }),
  navigateToOverview: () => set({
    activePanel: 'overview',
    selectedProviderId: null,
  }),
}));
