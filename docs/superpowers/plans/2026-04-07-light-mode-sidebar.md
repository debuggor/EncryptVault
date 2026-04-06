# Light Mode Sidebar Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the sidebar respect light mode by replacing hardcoded dark Tailwind classes with light-mode defaults + `dark:` variants.

**Architecture:** Single file edit in `src/components/Sidebar.tsx`. Each hardcoded dark class (`bg-gray-900`, `text-white`, etc.) gains a light-mode default value and keeps its current value as the `dark:` variant. No new files, no new abstractions. Follows the same pattern already used in `App.tsx`.

**Tech Stack:** React, Tailwind CSS v3 (`darkMode: 'class'` — toggled via `.dark` on `<html>`)

---

### Task 1: Update sidebar wrapper and nav button classes

**Files:**
- Modify: `src/components/Sidebar.tsx`

- [ ] **Step 1: Open the file and locate the `aside` element (line 32)**

The current class string is:
```
w-52 bg-gray-900 dark:bg-gray-900 text-white dark:text-white flex flex-col h-screen shrink-0
```

Replace it with:
```
w-52 bg-gray-100 dark:bg-gray-900 text-gray-900 dark:text-white flex flex-col h-screen shrink-0
```

- [ ] **Step 2: Update the title border (line 33)**

Current:
```
px-4 py-5 text-lg font-bold border-b border-gray-700 dark:border-gray-700
```
Replace with:
```
px-4 py-5 text-lg font-bold border-b border-gray-200 dark:border-gray-700
```

- [ ] **Step 3: Update the nav button hover and active classes (lines 49–53)**

Current:
```tsx
className={`w-full text-left px-4 py-3 flex items-center gap-3 hover:bg-gray-800 dark:hover:bg-gray-800 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 ${
  currentPage === p.id
    ? "bg-gray-800 dark:bg-gray-800 font-medium"
    : ""
}`}
```
Replace with:
```tsx
className={`w-full text-left px-4 py-3 flex items-center gap-3 hover:bg-gray-200 dark:hover:bg-gray-800 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 ${
  currentPage === p.id
    ? "bg-gray-200 dark:bg-gray-800 font-medium"
    : ""
}`}
```

- [ ] **Step 4: Update the theme switcher border (line 62)**

Current:
```
p-4 border-t border-gray-700 dark:border-gray-700
```
Replace with:
```
p-4 border-t border-gray-200 dark:border-gray-700
```

- [ ] **Step 5: Update the theme switcher active button (line 79)**

Current:
```
theme === t.id
  ? "bg-gray-600 rounded-lg"
  : "opacity-40 hover:opacity-70"
```
Replace with:
```
theme === t.id
  ? "bg-gray-300 dark:bg-gray-600 rounded-lg"
  : "opacity-40 hover:opacity-70"
```

- [ ] **Step 6: Update the lock section border (line 89)**

Current:
```
p-4 border-t border-gray-700 dark:border-gray-700
```
Replace with:
```
p-4 border-t border-gray-200 dark:border-gray-700
```

- [ ] **Step 7: Update the lock button text and hover (line 99)**

Current:
```
w-full text-sm text-gray-400 dark:text-gray-400 hover:text-white dark:hover:text-white transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500
```
Replace with:
```
w-full text-sm text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500
```

- [ ] **Step 8: Verify the file looks correct**

Run:
```bash
grep -n "bg-gray-900\|text-white\|border-gray-700\|hover:bg-gray-800\|bg-gray-800\|bg-gray-600\|text-gray-400\|hover:text-white" src/components/Sidebar.tsx
```

Expected: every remaining hit has a `dark:` prefix (i.e., no bare hardcoded dark classes remain outside of `dark:` variants).

- [ ] **Step 9: Start dev server and visually verify**

Run:
```bash
npm run tauri dev
```

Check:
1. Switch to **light mode** (☀️ button in sidebar) — sidebar should be soft gray (`bg-gray-100`), text dark, borders light, active nav item highlighted in gray-200, lock button readable
2. Switch to **night mode** (🌙 button) — sidebar should be unchanged from before (dark gray background, white text)
3. Switch to **system mode** (💻 button) — sidebar should follow OS preference

- [ ] **Step 10: Commit**

```bash
git add src/components/Sidebar.tsx
git commit -m "feat: add light mode variants to sidebar"
```
