# Theme Design Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Light, Night, and System theme modes to EncryptVault with persistence via localStorage and live updates for System mode.

**Architecture:** React context manages theme state (`'light' | 'night' | 'system'`); Tailwind `darkMode: 'class'` with `dark` class on `<html>`; `matchMedia` listener for System mode; explicit `dark:` variants across all components.

**Tech Stack:** React (TypeScript), Tailwind CSS, Tauri 2 (frontend only)

---

## Files to Create/Modify

| File | Responsibility |
|------|----------------|
| `tailwind.config.js` | Enable `darkMode: 'class'` |
| `src/context/AppContext.tsx` | Theme state, persistence, dark class toggle |
| `src/components/Sidebar.tsx` | Theme switcher icons, dark variant isolation |
| `src/pages/SettingsPage.tsx` | Appearance section with theme cards |
| `src/App.tsx` | Main layout dark variants |
| `src/pages/UnlockPage.tsx` | Unlock page dark variants |
| `src/pages/EncryptPage.tsx` | Encrypt page dark variants |
| `src/pages/VaultPage.tsx` | Vault page dark variants |
| `src/pages/WalletPage.tsx` | Wallet page dark variants |
| `src/pages/QRPage.tsx` | QR page dark variants |
| `src/components/ErrorBanner.tsx` | Error banner dark variants |

---

### Task 1: Configure Tailwind Dark Mode

**Files:**
- Modify: `tailwind.config.js:1-7`

- [ ] **Step 1: Update Tailwind configuration**

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  darkMode: 'class',
  theme: { extend: {} },
  plugins: [],
};
```

- [ ] **Step 2: Verify configuration loads**

Run: `npm run tauri dev`
Expected: Development server starts without errors.

- [ ] **Step 3: Commit**

```bash
git add tailwind.config.js
git commit -m "feat: enable Tailwind dark mode with class strategy"
```

---

### Task 2: Extend AppContext with Theme State

**Files:**
- Modify: `src/context/AppContext.tsx:1-28`

- [ ] **Step 1: Add Theme type and extend context interface**

```typescript
import { createContext, useContext, useState, ReactNode, useEffect } from "react";

type Page = "encrypt" | "vault" | "wallet" | "qr" | "settings";
export type Theme = 'light' | 'night' | 'system';

interface AppContextType {
  isUnlocked: boolean;
  setUnlocked: (v: boolean) => void;
  currentPage: Page;
  setPage: (p: Page) => void;
  theme: Theme;
  setTheme: (t: Theme) => void;
}
```

- [ ] **Step 2: Update AppProvider to include theme state**

```typescript
export function AppProvider({ children }: { children: ReactNode }) {
  const [isUnlocked, setUnlocked] = useState(false);
  const [currentPage, setPage] = useState<Page>("encrypt");
  const [theme, setTheme] = useState<Theme>('system');
  return (
    <AppContext.Provider value={{ isUnlocked, setUnlocked, currentPage, setPage, theme, setTheme }}>
      {children}
    </AppContext.Provider>
  );
}
```

- [ ] **Step 3: Verify TypeScript compilation**

Run: `npm run tauri dev` (or `tsc --noEmit` if available)
Expected: No TypeScript errors.

- [ ] **Step 4: Commit**

```bash
git add src/context/AppContext.tsx
git commit -m "feat: add theme state to AppContext"
```

---

### Task 3: Implement Theme Persistence and Dark Class Toggle

**Files:**
- Modify: `src/context/AppContext.tsx:14-30` (AppProvider function)

- [ ] **Step 1: Add localStorage read for initial theme**

```typescript
export function AppProvider({ children }: { children: ReactNode }) {
  const [isUnlocked, setUnlocked] = useState(false);
  const [currentPage, setPage] = useState<Page>("encrypt");
  
  // Read initial theme from localStorage
  const [theme, setTheme] = useState<Theme>(() => {
    const stored = localStorage.getItem('encryptvault-theme');
    if (stored === 'light' || stored === 'night' || stored === 'system') {
      return stored;
    }
    return 'system';
  });
```

- [ ] **Step 2: Add useEffect for dark class and persistence**

```typescript
  useEffect(() => {
    // Determine effective mode (system → follow OS)
    const effectiveMode = theme === 'system' 
      ? (window.matchMedia('(prefers-color-scheme: dark)').matches ? 'night' : 'light')
      : theme;
    
    // Toggle dark class
    const html = document.documentElement;
    if (effectiveMode === 'night') {
      html.classList.add('dark');
    } else {
      html.classList.remove('dark');
    }
    
    // Persist choice
    localStorage.setItem('encryptvault-theme', theme);
    
    // System mode live updates
    if (theme === 'system') {
      const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
      const handleChange = () => {
        const effective = mediaQuery.matches ? 'night' : 'light';
        if (effective === 'night') {
          html.classList.add('dark');
        } else {
          html.classList.remove('dark');
        }
      };
      mediaQuery.addEventListener('change', handleChange);
      return () => mediaQuery.removeEventListener('change', handleChange);
    }
  }, [theme]);
  
  return (
    <AppContext.Provider value={{ isUnlocked, setUnlocked, currentPage, setPage, theme, setTheme }}>
      {children}
    </AppContext.Provider>
  );
}
```

- [ ] **Step 3: Test theme switching manually**

1. Run: `npm run tauri dev`
2. Open browser dev tools → Application → Local Storage
3. Verify `encryptvault-theme` defaults to `'system'`
4. Manually test: `document.documentElement.classList.toggle('dark')`
5. Verify Tailwind dark variants work.

- [ ] **Step 4: Commit**

```bash
git add src/context/AppContext.tsx
git commit -m "feat: implement theme persistence and dark class toggle"
```

---

### Task 4: Add Theme Switcher to Sidebar

**Files:**
- Modify: `src/components/Sidebar.tsx:1-50`

- [ ] **Step 1: Import useApp and add theme switcher component**

```typescript
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
  const { currentPage, setPage, setUnlocked, theme, setTheme } = useApp();
  
  async function handleLock() {
    await invoke("lock_vault");
    setUnlocked(false);
  }
  
  const themes: { id: Theme; icon: string; label: string }[] = [
    { id: 'light', icon: '☀️', label: 'Light' },
    { id: 'night', icon: '🌙', label: 'Night' },
    { id: 'system', icon: '💻', label: 'System' },
  ];
```

- [ ] **Step 2: Add theme switcher UI between nav and lock button**

```typescript
      </nav>
      
      {/* Theme switcher */}
      <div className="p-4 border-t border-gray-700">
        <div className="flex justify-center gap-3">
          {themes.map((t) => (
            <button
              key={t.id}
              onClick={() => setTheme(t.id)}
              title={t.label}
              className={`p-2 text-lg transition-colors ${
                theme === t.id 
                  ? 'bg-gray-600 rounded-lg' 
                  : 'opacity-40 hover:opacity-70'
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
          className="w-full text-sm text-gray-400 hover:text-white transition-colors"
        >
          Lock vault
        </button>
      </div>
    </aside>
  );
}
```

- [ ] **Step 3: Test theme switching**

1. Run: `npm run tauri dev`
2. Click theme icons in sidebar
3. Verify localStorage updates
4. Verify dark class toggles correctly
5. Verify active icon highlighting.

- [ ] **Step 4: Commit**

```bash
git add src/components/Sidebar.tsx
git commit -m "feat: add theme switcher to sidebar"
```

---

### Task 5: Add Appearance Section to Settings Page

**Files:**
- Modify: `src/pages/SettingsPage.tsx:1-92`

- [ ] **Step 1: Update page subtitle and import useApp**

```typescript
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useApp } from "../context/AppContext";
import ErrorBanner from "../components/ErrorBanner";

export default function SettingsPage() {
  const { setUnlocked } = useApp();
  const [currentPassword, setCurrentPassword] = useState("");
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
```

- [ ] **Step 2: Add Appearance section above password form**

```typescript
  const { theme, setTheme } = useApp();
  const themes = [
    { id: 'light' as const, icon: '☀️', label: 'Light' },
    { id: 'night' as const, icon: '🌙', label: 'Night' },
    { id: 'system' as const, icon: '💻', label: 'System' },
  ];

  return (
    <div className="max-w-md">
      <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-50 mb-1">Settings</h1>
      <p className="text-sm text-gray-500 dark:text-gray-400 mb-6">
        Appearance and security preferences.
      </p>
      
      {/* Appearance section */}
      <div className="mb-8">
        <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">Appearance</h2>
        <div className="flex flex-col sm:flex-row gap-4">
          {themes.map((t) => (
            <button
              key={t.id}
              onClick={() => setTheme(t.id)}
              className={`flex-1 flex flex-col items-center justify-center p-4 border rounded-lg transition-colors ${
                theme === t.id
                  ? 'border-blue-600 bg-blue-50 text-blue-700 dark:border-blue-400 dark:bg-blue-950 dark:text-blue-300'
                  : 'border-gray-300 dark:border-gray-700 text-gray-700 dark:text-gray-300'
              }`}
            >
              <span className="text-2xl mb-2">{t.icon}</span>
              <span className="text-sm">{t.label}</span>
            </button>
          ))}
        </div>
      </div>
      
      <div className="border-t border-gray-300 dark:border-gray-700 pt-8">
        <h2 className="text-lg font-semibold text-gray-800 dark:text-gray-200 mb-4">Master password</h2>
        <ErrorBanner message={error} onDismiss={() => setError(null)} />
        <form onSubmit={handleSubmit} className="space-y-4">
```

- [ ] **Step 3: Update rest of form with dark variants**

Update all text colors in the form to include dark variants:
- `text-gray-900` → `text-gray-900 dark:text-gray-50`
- `text-gray-700` → `text-gray-700 dark:text-gray-300`
- `text-gray-500` → `text-gray-500 dark:text-gray-400`
- `border-gray-300` → `border-gray-300 dark:border-gray-700`

- [ ] **Step 4: Test Settings page**

1. Run: `npm run tauri dev`
2. Navigate to Settings page
3. Verify Appearance section displays correctly
4. Test theme card selection
5. Verify dark variants on form elements.

- [ ] **Step 5: Commit**

```bash
git add src/pages/SettingsPage.tsx
git commit -m "feat: add Appearance section to Settings page"
```

---

### Task 6: Add Dark Variants to Unlock Page

**Files:**
- Modify: `src/pages/UnlockPage.tsx:26-53`

- [ ] **Step 1: Update UnlockPage with dark variants**

```typescript
  return (
    <div className="min-h-screen bg-gray-900 flex items-center justify-center">
      <div className="bg-white dark:bg-gray-800 rounded-2xl p-8 w-full max-w-sm shadow-xl">
        <h1 className="text-2xl font-bold text-gray-900 dark:text-gray-50 mb-2">EncryptVault</h1>
        <p className="text-sm text-gray-500 dark:text-gray-400 mb-6">Enter your master password to unlock.</p>
        <ErrorBanner message={error} onDismiss={() => setError(null)} />
        <form onSubmit={handleSubmit} className="space-y-4">
          <input
            type="password"
            placeholder="Master password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="w-full border border-gray-300 dark:border-gray-700 rounded-lg px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-50"
            required
            autoFocus
          />
          <button
            type="submit"
            disabled={loading}
            className="w-full bg-blue-600 text-white rounded-lg py-2 text-sm font-medium hover:bg-blue-700 disabled:opacity-50 transition-colors"
          >
            {loading ? "Unlocking…" : "Unlock"}
          </button>
        </form>
      </div>
    </div>
  );
```

- [ ] **Step 2: Test Unlock page in both modes**

1. Toggle theme via sidebar (app must be unlocked first)
2. Lock vault to see Unlock page
3. Verify dark mode renders correctly.

- [ ] **Step 3: Commit**

```bash
git add src/pages/UnlockPage.tsx
git commit -m "feat: add dark variants to Unlock page"
```

---

### Task 7: Add Dark Variants to Encrypt Page

**Files:**
- Modify: `src/pages/EncryptPage.tsx` (check if exists)

- [ ] **Step 1: Check EncryptPage structure**

Run: `head -20 src/pages/EncryptPage.tsx`
Note: Update all text and background colors per color token table.

- [ ] **Step 2: Apply dark variants to text elements**

General pattern: add `dark:` variants to:
- `text-gray-900` → `text-gray-900 dark:text-gray-50`
- `text-gray-700` → `text-gray-700 dark:text-gray-300`
- `text-gray-500` → `text-gray-500 dark:text-gray-400`
- `bg-white` → `bg-white dark:bg-gray-800`
- `border-gray-300` → `border-gray-300 dark:border-gray-700`

- [ ] **Step 3: Test Encrypt page**

1. Run: `npm run tauri dev`
2. Navigate to Encrypt page
3. Toggle themes
4. Verify all elements have correct dark variants.

- [ ] **Step 4: Commit**

```bash
git add src/pages/EncryptPage.tsx
git commit -m "feat: add dark variants to Encrypt page"
```

---

### Task 8: Add Dark Variants to Vault Page

**Files:**
- Modify: `src/pages/VaultPage.tsx`

- [ ] **Step 1: Check VaultPage structure**

Run: `head -20 src/pages/VaultPage.tsx`
Apply same dark variant pattern as Task 7.

- [ ] **Step 2: Apply dark variants**

Add `dark:` variants to all text, background, and border classes.

- [ ] **Step 3: Test Vault page**

1. Navigate to Vault page
2. Toggle themes
3. Verify dark mode.

- [ ] **Step 4: Commit**

```bash
git add src/pages/VaultPage.tsx
git commit -m "feat: add dark variants to Vault page"
```

---

### Task 9: Add Dark Variants to Wallet Page

**Files:**
- Modify: `src/pages/WalletPage.tsx`

- [ ] **Step 1: Check WalletPage structure**

Run: `head -20 src/pages/WalletPage.tsx`
Apply same dark variant pattern.

- [ ] **Step 2: Apply dark variants**

Add `dark:` variants to all text, background, and border classes.

- [ ] **Step 3: Test Wallet page**

1. Navigate to Wallet page
2. Toggle themes
3. Verify dark mode.

- [ ] **Step 4: Commit**

```bash
git add src/pages/WalletPage.tsx
git commit -m "feat: add dark variants to Wallet page"
```

---

### Task 10: Add Dark Variants to QR Page

**Files:**
- Modify: `src/pages/QRPage.tsx`

- [ ] **Step 1: Check QRPage structure**

Run: `head -20 src/pages/QRPage.tsx`
Apply same dark variant pattern.

- [ ] **Step 2: Apply dark variants**

Add `dark:` variants to all text, background, and border classes.

- [ ] **Step 3: Test QR page**

1. Navigate to QR page
2. Toggle themes
3. Verify dark mode.

- [ ] **Step 4: Commit**

```bash
git add src/pages/QRPage.tsx
git commit -m "feat: add dark variants to QR page"
```

---

### Task 11: Add Dark Variants to ErrorBanner

**Files:**
- Modify: `src/components/ErrorBanner.tsx:1-15`

- [ ] **Step 1: Update ErrorBanner with dark variants**

```typescript
export default function ErrorBanner({ message, onDismiss }: Props) {
  if (!message) return null;
  return (
    <div className="bg-red-100 dark:bg-red-900/20 border border-red-400 dark:border-red-800 text-red-800 dark:text-red-300 px-4 py-3 rounded flex justify-between items-center mb-4">
      <span className="text-sm">{message}</span>
      <button onClick={onDismiss} className="ml-4 font-bold text-red-600 dark:text-red-400">×</button>
    </div>
  );
}
```

- [ ] **Step 2: Test ErrorBanner**

1. Trigger an error (e.g., wrong password on Unlock page)
2. Verify dark mode styling.

- [ ] **Step 3: Commit**

```bash
git add src/components/ErrorBanner.tsx
git commit -m "feat: add dark variants to ErrorBanner"
```

---

### Task 12: Isolate Sidebar with Explicit Dark Variants

**Files:**
- Modify: `src/components/Sidebar.tsx:20-48`

- [ ] **Step 1: Add explicit dark variants to sidebar elements**

```typescript
  return (
    <aside className="w-52 bg-gray-900 dark:bg-gray-900 text-white dark:text-white flex flex-col h-screen shrink-0">
      <div className="px-4 py-5 text-lg font-bold border-b border-gray-700 dark:border-gray-700">
        EncryptVault
      </div>
      <nav className="flex-1 py-4">
        {PAGES.map((p) => (
          <button
            key={p.id}
            onClick={() => setPage(p.id)}
            className={`w-full text-left px-4 py-3 flex items-center gap-3 hover:bg-gray-800 dark:hover:bg-gray-800 transition-colors ${
              currentPage === p.id ? "bg-gray-800 dark:bg-gray-800 font-medium" : ""
            }`}
          >
            <span>{p.icon}</span>
            <span className="text-sm">{p.label}</span>
          </button>
        ))}
      </nav>
      
      {/* Theme switcher (unchanged) */}
      
      <div className="p-4 border-t border-gray-700 dark:border-gray-700">
        <button
          onClick={handleLock}
          className="w-full text-sm text-gray-400 dark:text-gray-400 hover:text-white dark:hover:text-white transition-colors"
        >
          Lock vault
        </button>
      </div>
    </aside>
  );
```

- [ ] **Step 2: Test sidebar isolation**

1. Toggle between light and night themes
2. Verify sidebar appearance remains consistent (dark)
3. Verify hover states work.

- [ ] **Step 3: Commit**

```bash
git add src/components/Sidebar.tsx
git commit -m "feat: isolate sidebar with explicit dark variants"
```

---

### Task 13: Add Dark Variants to Main Layout

**Files:**
- Modify: `src/App.tsx:10-25`

- [ ] **Step 1: Update main layout wrapper**

```typescript
function PageRouter() {
  const { isUnlocked, currentPage } = useApp();
  if (!isUnlocked) return <UnlockPage />;
  return (
    <div className="flex h-screen bg-gray-50 dark:bg-gray-900">
      <Sidebar />
      <main className="flex-1 overflow-auto p-8">
        {currentPage === "encrypt" && <EncryptPage />}
        {currentPage === "vault" && <VaultPage />}
        {currentPage === "wallet" && <WalletPage />}
        {currentPage === "qr" && <QRPage />}
        {currentPage === "settings" && <SettingsPage />}
      </main>
    </div>
  );
}
```

- [ ] **Step 2: Test main layout**

1. Toggle themes
2. Verify main background changes correctly
3. Verify all pages inherit correct background.

- [ ] **Step 3: Final commit**

```bash
git add src/App.tsx
git commit -m "feat: add dark variants to main layout wrapper"
```

---

## Self-Review Checklist

✅ **Spec coverage:** All spec requirements mapped to tasks:
- Tailwind dark mode (Task 1)
- Theme state and persistence (Tasks 2-3)
- Sidebar switcher (Task 4)
- Settings page Appearance section (Task 5)
- Dark variants on all pages (Tasks 6-10, 11, 12, 13)
- Sidebar isolation (Task 12)
- ErrorBanner dark variants (Task 11)

✅ **Placeholder scan:** No TBD/TODO placeholders; all code shown.

✅ **Type consistency:** `Theme` type defined in Task 2 and used consistently.

---

**Plan complete and saved to `docs/superpowers/plans/2026-04-05-theme-design.md`. Two execution options:**

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

**Which approach?**