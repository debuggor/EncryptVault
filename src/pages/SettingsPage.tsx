import { useState, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useApp, Theme } from "../context/AppContext";
import ErrorBanner from "../components/ErrorBanner";

export default function SettingsPage() {
  const { setUnlocked, theme, setTheme } = useApp();
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  interface ThemeOption {
    id: Theme;
    icon: string;
    label: string;
  }

  const themes = useMemo<ThemeOption[]>(
    () => [
      { id: "light", icon: "☀️", label: "Light" },
      { id: "night", icon: "🌙", label: "Night" },
      { id: "system", icon: "💻", label: "System" },
    ],
    [],
  );

  function validatePassword(): string | null {
    if (newPassword !== confirmPassword) {
      return "New passwords do not match";
    }
    if (newPassword.length < 8) {
      return "New password must be at least 8 characters";
    }
    return null;
  }

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    const validationError = validatePassword();
    if (validationError) {
      setError(validationError);
      return;
    }
    setLoading(true);
    setError(null);
    try {
      await invoke("reset_master_password", { currentPassword, newPassword });
      setUnlocked(false);
      setCurrentPassword("");
      setNewPassword("");
      setConfirmPassword("");
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="max-w-md">
      <h1 className="text-2xl font-bold text-gray-900 mb-1">Settings</h1>
      <p className="text-sm text-gray-500 mb-6">
        Appearance and security preferences.
      </p>

      {/* Appearance section */}
      <div className="mb-8">
        <h2 className="text-lg font-semibold text-gray-800 mb-4">Appearance</h2>
        <div className="flex flex-col sm:flex-row gap-4">
          {themes.map((t) => (
            <button
              key={t.id}
              onClick={() => setTheme(t.id)}
              aria-pressed={theme === t.id}
              aria-label={`Select ${t.label} theme`}
              className={`flex-1 flex flex-col items-center justify-center p-4 border rounded-lg transition-colors ${
                theme === t.id
                  ? "border-blue-600 bg-blue-50 text-blue-700"
                  : "border-gray-300 text-gray-700"
              }`}
            >
              <span className="text-2xl mb-2">{t.icon}</span>
              <span className="text-sm">{t.label}</span>
            </button>
          ))}
        </div>
      </div>

      <div className="border-t border-gray-300 pt-8">
        <h2 className="text-lg font-semibold text-gray-800 mb-4">
          Master password
        </h2>
        <ErrorBanner message={error} onDismiss={() => setError(null)} />
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Current password
            </label>
            <input
              type="password"
              value={currentPassword}
              onChange={(e) => setCurrentPassword(e.target.value)}
              className="w-full border border-gray-300 rounded-lg px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              required
              autoFocus
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              New password
            </label>
            <input
              type="password"
              value={newPassword}
              onChange={(e) => setNewPassword(e.target.value)}
              className="w-full border border-gray-300 rounded-lg px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              required
            />
          </div>
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-1">
              Confirm new password
            </label>
            <input
              type="password"
              value={confirmPassword}
              onChange={(e) => setConfirmPassword(e.target.value)}
              className="w-full border border-gray-300 rounded-lg px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
              required
            />
          </div>
          <button
            type="submit"
            disabled={loading}
            className="w-full bg-blue-600 text-white rounded-lg py-2 text-sm font-medium hover:bg-blue-700 disabled:opacity-50 transition-colors"
          >
            {loading ? "Resetting…" : "Reset master password"}
          </button>
        </form>
      </div>
    </div>
  );
}
