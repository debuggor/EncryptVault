import { AppProvider, useApp } from "./context/AppContext";
import Sidebar from "./components/Sidebar";
import UnlockPage from "./pages/UnlockPage";
import EncryptPage from "./pages/EncryptPage";
import VaultPage from "./pages/VaultPage";
import WalletPage from "./pages/WalletPage";
import QRPage from "./pages/QRPage";
import SettingsPage from "./pages/SettingsPage";

function PageRouter() {
  const { isUnlocked, currentPage } = useApp();
  if (!isUnlocked) return <UnlockPage />;
  return (
    <div className="flex h-screen bg-gray-50">
      <Sidebar />
      <main className="flex-1 overflow-auto p-8">
        {currentPage === "encrypt"  && <EncryptPage />}
        {currentPage === "vault"    && <VaultPage />}
        {currentPage === "wallet"   && <WalletPage />}
        {currentPage === "qr"       && <QRPage />}
        {currentPage === "settings" && <SettingsPage />}
      </main>
    </div>
  );
}

export default function App() {
  return (
    <AppProvider>
      <PageRouter />
    </AppProvider>
  );
}
