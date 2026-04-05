import { invoke } from "@tauri-apps/api/core";
import { useApp, Theme } from "../context/AppContext";

const PAGES = [
  { id: "encrypt" as const, label: "Encrypt / Decrypt", icon: "🔐" },
  { id: "vault" as const, label: "Password Vault", icon: "🗝️" },
  { id: "wallet" as const, label: "Wallet", icon: "💼" },
  { id: "qr" as const, label: "QR Code", icon: "⬛" },
  { id: "settings" as const, label: "Settings", icon: "⚙️" },
];

const THEMES: { id: Theme; icon: string; label: string }[] = [
  { id: "light", icon: "☀️", label: "Light" },
  { id: "night", icon: "🌙", label: "Night" },
  { id: "system", icon: "💻", label: "System" },
];

export default function Sidebar() {
  const { currentPage, setPage, setUnlocked, theme, setTheme } = useApp();

  async function handleLock() {
    try {
      await invoke("lock_vault");
      setUnlocked(false);
    } catch (error) {
      console.error("Failed to lock vault:", error);
      // In a real app, you might want to show an error message to the user
    }
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
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                setPage(p.id);
              }
            }}
            aria-current={currentPage === p.id ? "page" : undefined}
            tabIndex={0}
            className={`w-full text-left px-4 py-3 flex items-center gap-3 hover:bg-gray-800 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 ${
              currentPage === p.id ? "bg-gray-800 font-medium" : ""
            }`}
          >
            <span>{p.icon}</span>
            <span className="text-sm">{p.label}</span>
          </button>
        ))}
      </nav>

      {/* Theme switcher */}
      <div className="p-4 border-t border-gray-700">
        <div className="flex justify-center gap-3">
          {THEMES.map((t) => (
            <button
              key={t.id}
              onClick={() => setTheme(t.id)}
              onKeyDown={(e) => {
                if (e.key === "Enter" || e.key === " ") {
                  e.preventDefault();
                  setTheme(t.id);
                }
              }}
              title={t.label}
              aria-pressed={theme === t.id}
              tabIndex={0}
              className={`p-2 text-lg transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 ${
                theme === t.id
                  ? "bg-gray-600 rounded-lg"
                  : "opacity-40 hover:opacity-70"
              }`}
            >
              {t.icon}
            </button>
          ))}
        </div>
      </div>

      <div className="p-4 border-t border-gray-700">
        <button
          onClick={handleLock}
          onKeyDown={(e) => {
            if (e.key === "Enter" || e.key === " ") {
              e.preventDefault();
              handleLock();
            }
          }}
          tabIndex={0}
          className="w-full text-sm text-gray-400 hover:text-white transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500"
        >
          Lock vault
        </button>
      </div>
    </aside>
  );
}
