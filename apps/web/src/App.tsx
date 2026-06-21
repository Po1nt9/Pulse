import { useUIStore } from './store/uiStore';
import { OverviewPanel } from './components/OverviewPanel';
import { DetailPanel } from './components/DetailPanel';
import { SettingsPanel } from './components/SettingsPanel';
import { AddProviderModal } from './components/AddProviderModal';
import { NotificationToast } from './components/NotificationToast';

export function App() {
  const activePanel = useUIStore((state) => state.activePanel);
  const isAddProviderModalOpen = useUIStore((state) => state.isAddProviderModalOpen);

  return (
    <div className="w-full h-full bg-surface/80 backdrop-blur-xl">
      {activePanel === 'overview' && <OverviewPanel />}
      {activePanel === 'detail' && <DetailPanel />}
      {activePanel === 'settings' && <SettingsPanel />}

      {isAddProviderModalOpen && <AddProviderModal />}
      <NotificationToast />
    </div>
  );
}
