# Light Mode Sidebar Design

**Date:** 2026-04-07
**Status:** Approved

## Problem

In light mode the sidebar renders with hardcoded dark colors (`bg-gray-900`, `text-white`) regardless of theme. The main content area already responds correctly to light mode (`bg-gray-50 dark:bg-gray-900` in `App.tsx`). The sidebar does not.

## Goal

In light mode: sidebar uses a soft gray background (`bg-gray-100`) with dark text. In night mode and system mode: no change from current behavior.

## Scope

- **In scope:** `src/components/Sidebar.tsx` — all class pairs updated to `<light-value> dark:<dark-value>`
- **Out of scope:** `UnlockPage.tsx` lock screen (hardcoded dark, intentional — not part of this request)

## Approach

Direct Tailwind overrides. Each hardcoded dark class gets a light-mode default and retains its current value as the `dark:` variant. Consistent with how `App.tsx` already handles theming. No new abstractions.

## Class Changes

| Element | Before | After |
|---|---|---|
| `aside` background | `bg-gray-900 dark:bg-gray-900` | `bg-gray-100 dark:bg-gray-900` |
| `aside` text | `text-white dark:text-white` | `text-gray-900 dark:text-white` |
| All three divider borders | `border-gray-700 dark:border-gray-700` | `border-gray-200 dark:border-gray-700` |
| Nav button hover | `hover:bg-gray-800 dark:hover:bg-gray-800` | `hover:bg-gray-200 dark:hover:bg-gray-800` |
| Nav button active | `bg-gray-800 dark:bg-gray-800` | `bg-gray-200 dark:bg-gray-800` |
| Theme switcher active button | `bg-gray-600 rounded-lg` | `bg-gray-300 dark:bg-gray-600 rounded-lg` |
| Lock button text | `text-gray-400 dark:text-gray-400` | `text-gray-500 dark:text-gray-400` |
| Lock button hover | `hover:text-white dark:hover:text-white` | `hover:text-gray-900 dark:hover:text-white` |

## Success Criteria

- Light mode: sidebar is `bg-gray-100` with `text-gray-900`, all interactive states visible
- Night mode: sidebar unchanged (still `bg-gray-900` / `text-white`)
- System mode: follows OS preference, inherits whichever of the above applies
