import { invoke } from "@tauri-apps/api/core";
import { useApp } from "../context/AppContext";

const PAGES = [
  { id: "encrypt"  as const, label: "Encrypt / Decrypt", icon: "🔐" },
  { id: "vault"    as const, label: "Password Vault",     icon: "🗝️" },
  { id: "wallet"   as const, label: "Wallet",             icon: "💼" },
  { id: "qr"       as const, label: "QR Code",            icon: "⬛" },
  { id: "settings" as const, label: "Settings",           icon: "⚙️" },
];

export default function Sidebar() {
  const { currentPage, setPage, setUnlocked } = useApp();

  async function handleLock() {
    await invoke("lock_vault");
    setUnlocked(false);
  }

  return (
    <aside className="w-52 bg-gray-900 text-white flex flex-col h-screen shrink-0">
      <div className="px-4 py-5 text-lg font-bold border-b border-gray-700">
        EncryptVault
      </div>
      <nav className="flex-1 py-4">
        {PAGES.map((p) => (
          <button
            key={p.id}
            onClick={() => setPage(p.id)}
            className={`w-full text-left px-4 py-3 flex items-center gap-3 hover:bg-gray-800 transition-colors ${
              currentPage === p.id ? "bg-gray-800 font-medium" : ""
            }`}
          >
            <span>{p.icon}</span>
            <span className="text-sm">{p.label}</span>
          </button>
        ))}
      </nav>
      <div className="p-4 border-t border-gray-700">
        <button
          onClick={handleLock}
          className="w-full text-sm text-gray-400 hover:text-white transition-colors"
        >
          Lock vault
        </button>
      </div>
    </aside>
  );
}
