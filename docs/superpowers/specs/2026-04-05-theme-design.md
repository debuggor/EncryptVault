# Theme Design — Light / Night / System

**Date:** 2026-04-05
**Status:** Approved

## Overview

Add Light, Night, and System theme modes to EncryptVault. The selected mode persists across restarts and is accessible from both the sidebar (quick toggle) and the Settings page (labelled cards). No Rust backend changes are required — all state lives in the React layer.

## Modes

| Mode | Behaviour |
|---|---|
| **Light** | Always light palette |
| **Night** | Always dark palette |
| **System** | Follows macOS appearance via `prefers-color-scheme`; updates live if the OS setting changes while the app is open |

Default on first launch: **System**.

## Technical approach

**Tailwind `darkMode: 'class'`** — one line added to `tailwind.config.js`. A `dark` class on `<html>` activates all `dark:` variants. This is the standard Tailwind pattern; no custom CSS abstractions are introduced.

### State & persistence

`AppContext` gains two new values:

```ts
type Theme = 'light' | 'night' | 'system'
theme: Theme
setTheme: (t: Theme) => void
```

A single `useEffect` inside `AppProvider` is the authoritative place that:

1. Resolves the effective mode — for `'system'`, reads `matchMedia('(prefers-color-scheme: dark)')`.
2. Adds or removes `class="dark"` on `document.documentElement`.
3. Writes the choice to `localStorage` under key `encryptvault-theme`.

A `matchMedia` change listener (only active when `theme === 'system'`) keeps System mode live: if the user changes macOS appearance while the app is open, the dark class updates immediately without requiring a restart or manual action.

On first render, `AppProvider` reads `localStorage` to initialise `theme`; if nothing is stored it defaults to `'system'`.

## Colour tokens

All `dark:` variants use standard Tailwind grey/blue scale values. No custom CSS variables.

| Role | Light | Night (`dark:`) |
|---|---|---|
| Page background | `bg-gray-50` | `dark:bg-gray-900` |
| Card / input background | `bg-white` | `dark:bg-gray-800` |
| Border | `border-gray-300` | `dark:border-gray-700` |
| Heading text | `text-gray-900` | `dark:text-gray-50` |
| Label text | `text-gray-700` | `dark:text-gray-300` |
| Muted / placeholder text | `text-gray-500` | `dark:text-gray-400` |
| Primary accent | `bg-blue-600` | unchanged |
| Focus ring | `ring-blue-500` | unchanged |

The sidebar (`bg-gray-900`) is already dark; add explicit `dark:` variants to maintain its appearance in both light and night modes.

## Sidebar switcher

An icon row is added between the nav links and the "Lock vault" button, separated by the existing `border-t border-gray-700`:

```
☀️  🌙  💻
```

- Three icon buttons, centred, with `gap-3`
- Active icon: `bg-gray-600 rounded-lg` highlight
- Inactive icons: `opacity-40`
- Each button has a `title` attribute (`Light`, `Night`, `System`) for hover tooltip
- Clicking any icon calls `setTheme` immediately

## Settings page

A new **Appearance** section is inserted above the existing "Master password" form. The page subtitle changes to "Appearance and security preferences."

The section contains three equal-width card buttons (one per mode), each showing:
- The icon (☀️ / 🌙 / 💻)
- The mode name below it

Active card styling:
- Light mode: `border-blue-600 bg-blue-50 text-blue-700`
- Night mode: `dark:border-blue-400 dark:bg-blue-950 dark:text-blue-300`

Inactive cards: `border-gray-300 dark:border-gray-700` with default text colour.

The two sections (Appearance and Master password) are separated by a `border-t`.

## Clarifications

**Sidebar dark mode isolation:** The sidebar currently uses dark colors (`bg-gray-900`). To prevent unintended changes when the `dark` class is applied, add explicit `dark:` variants to all sidebar elements (e.g., `dark:bg-gray-900`, `dark:text-white`) to maintain the current appearance in both light and night modes.

**ErrorBanner colors:** Use Tailwind's built-in dark variants: `dark:bg-red-900/20 dark:border-red-800 dark:text-red-300`.

**Settings card layout:** On desktop, cards use `flex flex-row gap-4`; on mobile (`max-sm:`), they stack vertically with `flex-col`.

**Icon implementation:** Use emoji characters (☀️🌙💻) as shown. Add `title` attributes for tooltips: `"Light"`, `"Night"`, `"System"`.

**Type exports:** Export the `Theme` type from `AppContext` for use in `Sidebar` and `SettingsPage`.

**Invalid localStorage handling:** If the stored value is not `'light'`, `'night'`, or `'system'`, default to `'system'` and overwrite the invalid value.

**Default behavior:** On first launch (no stored preference), default to `'system'`.

## Files to change

| File | Change |
|---|---|
| `tailwind.config.js` | Add `darkMode: 'class'` |
| `src/context/AppContext.tsx` | Add `theme`, `setTheme`; `useEffect` for class toggle + localStorage; `matchMedia` listener |
| `src/components/Sidebar.tsx` | Add icon row; add explicit `dark:` variants to maintain current sidebar appearance |
| `src/App.tsx` | Add `dark:` variants to main layout wrapper |
| `src/pages/UnlockPage.tsx` | Add `dark:` variants |
| `src/pages/SettingsPage.tsx` | Add Appearance section; add `dark:` variants throughout |
| `src/pages/EncryptPage.tsx` | Add `dark:` variants |
| `src/pages/VaultPage.tsx` | Add `dark:` variants |
| `src/pages/WalletPage.tsx` | Add `dark:` variants |
| `src/pages/QRPage.tsx` | Add `dark:` variants |
| `src/components/ErrorBanner.tsx` | Add `dark:` variants |

## Out of scope

- Per-page theme overrides
- Animated theme transition (crossfade)
- Storing theme preference in the Rust backend
