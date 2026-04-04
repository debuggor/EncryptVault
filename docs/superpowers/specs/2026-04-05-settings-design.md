# Settings Page & Reset Master Password — Design Spec

**Date:** 2026-04-05  
**Status:** Approved

---

## Overview

Add a dedicated **Settings** page to the EncryptVault sidebar. The initial feature is **Reset Master Password**: the user can change their master password while the vault is unlocked. On success, the vault is locked and the user must re-authenticate with the new password.

---

## Architecture

The feature spans two layers:

- **Backend (Rust/Tauri):** A new `reset_master_password` command in `src-tauri/src/commands/vault_cmd.rs` that re-encrypts the entire vault with the new password and commits atomically.
- **Frontend (React/TypeScript):** A new `SettingsPage` component, a sidebar nav entry, and small additions to `AppContext` and `App.tsx`.

No changes to the `password-vault` or `encrypt` crates are needed.

---

## Backend

### New Tauri command: `reset_master_password`

**Signature:**
```rust
pub fn reset_master_password(
    current_password: String,
    new_password: String,
    state: State<'_, AppState>,
) -> Result<(), String>
```

**Steps:**

1. **Verify current password.** Re-derive the key from `current_password` + the existing salt (read from `vault.salt`). Compare byte-for-byte with the key in `state.vault_key`. If they differ, return `Err("Current password is incorrect")`.

2. **Load all credentials.** Call `vault.list()` on the already-open vault in `state.vault` to get all credentials in memory.

3. **Generate new key.** Call `generate_salt()` for a fresh salt. Call `derive_key(&new_password, &new_salt)` to get the new 32-byte AES key.

4. **Re-encrypt to a temp db.** Open a new `SqliteStore` at `vault.db.tmp` with the new key. Upsert all credentials into it.

5. **Atomic commit.** Write new salt to `vault.salt.tmp`, then:
   - `rename("vault.salt.tmp", "vault.salt")`
   - `rename("vault.db.tmp", "vault.db")`

6. **Lock.** Call `state.lock()` to clear the vault and key from memory. Return `Ok(())`.

**Error handling:** If any step fails after step 4 has begun, the `.tmp` files are left behind but the live `vault.salt` and `vault.db` are untouched. The user can retry safely.

---

## Frontend

### AppContext (`src/context/AppContext.tsx`)

Add `"settings"` to the `Page` type:
```ts
type Page = "encrypt" | "vault" | "wallet" | "qr" | "settings";
```

### Sidebar (`src/components/Sidebar.tsx`)

Add a Settings entry to the `PAGES` array:
```ts
{ id: "settings" as const, label: "Settings", icon: "⚙️" }
```

### New page: `SettingsPage` (`src/pages/SettingsPage.tsx`)

A form with three fields:
- **Current password** (type=password)
- **New password** (type=password)
- **Confirm new password** (type=password)

**Behavior:**
- Client-side: validate that new password and confirm match before invoking the command.
- On submit: `invoke("reset_master_password", { currentPassword, newPassword })`.
- On success: call `setUnlocked(false)` — redirects to the unlock screen.
- On error: display the error string via the existing `ErrorBanner` component.

### App.tsx

Import `SettingsPage` and add to `PageRouter`:
```tsx
{currentPage === "settings" && <SettingsPage />}
```

---

## Data Flow

```
User fills form → client validates confirm match
  → invoke reset_master_password(currentPassword, newPassword)
    → verify current password (key comparison)
    → load all credentials
    → generate new salt + key
    → re-encrypt to vault.db.tmp
    → atomic rename salt + db
    → state.lock()
  → frontend: setUnlocked(false) → UnlockPage shown
```

---

## What Is Not In Scope

- Export/import vault data
- Any other settings (theme, auto-lock timeout, etc.)
- Password strength meter for the new password
