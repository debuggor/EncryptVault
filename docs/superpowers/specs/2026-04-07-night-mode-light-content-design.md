# Night Mode Light Content Area Design

**Date:** 2026-04-07
**Status:** Approved

## Problem

In night mode the entire app — sidebar and content area — uses dark styling, driven by the `.dark` class on `<html>` activating all `dark:` Tailwind variants simultaneously. The goal is a mixed layout: dark sidebar + white content area in night mode.

## Goal

Night mode (and system mode when OS is dark): sidebar stays dark, content area is always white with light-mode styling. Light mode: unchanged.

## Approach

Strip all `dark:` class variants from the content area files. The base (light-mode) Tailwind values are already correct — only the `dark:` overrides need removing. The `<main>` wrapper background loses its `dark:bg-gray-900` and becomes `bg-white`. Since the page files are only ever rendered inside the content area, removing their `dark:` prefixes means they always render light regardless of whether `.dark` is on `<html>`.

The sidebar's `dark:` variants are untouched — they continue to make the sidebar dark in night mode.

## Scope

**Files modified:**
| File | Change |
|---|---|
| `src/App.tsx` | `<main>`: `bg-gray-50 dark:bg-gray-900` → `bg-white` |
| `src/pages/EncryptPage.tsx` | Strip all `dark:` class prefixes |
| `src/pages/VaultPage.tsx` | Strip all `dark:` class prefixes |
| `src/pages/WalletPage.tsx` | Strip all `dark:` class prefixes |
| `src/pages/QRPage.tsx` | Strip all `dark:` class prefixes |
| `src/pages/SettingsPage.tsx` | Strip all `dark:` class prefixes |
| `src/components/ErrorBanner.tsx` | Strip all `dark:` class prefixes |

**Out of scope:** `UnlockPage.tsx` (lock screen, hardcoded dark intentionally), `Sidebar.tsx` (keeps all dark: variants).

## Dark: Class Removal Rules

For every `dark:` class in the in-scope files:
- Remove the `dark:` variant entirely — keep only the base value
- Examples:
  - `bg-white dark:bg-gray-800` → `bg-white`
  - `text-gray-900 dark:text-gray-50` → `text-gray-900`
  - `border-gray-300 dark:border-gray-700` → `border-gray-300`
  - `bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600` → `bg-gray-200 text-gray-700 hover:bg-gray-300`
  - `bg-red-100 dark:bg-red-900/20 border-red-400 dark:border-red-800 text-red-800 dark:text-red-300` → `bg-red-100 border-red-400 text-red-800`
  - `border-blue-600 bg-blue-50 text-blue-700 dark:border-blue-400 dark:bg-blue-950 dark:text-blue-300` → `border-blue-600 bg-blue-50 text-blue-700`

## Success Criteria

- **Light mode:** content area unchanged (already light)
- **Night mode:** sidebar is dark gray (`bg-gray-900`), content area is white (`bg-white`) with dark text, light borders, light cards
- **System mode (OS dark):** same as night mode — dark sidebar, white content
- **System mode (OS light):** same as light mode
