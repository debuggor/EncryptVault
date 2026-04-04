# Settings Page & Reset Master Password — Design Spec

**Date:** 2026-04-05  
**Status:** Approved (updated: settings crate added)

---

## Overview

Add a dedicated **Settings** page to the EncryptVault sidebar, backed by a new `crates/settings` Rust crate. The initial feature is **Reset Master Password**: the user can change their master password while the vault is unlocked. On success, the vault is locked and the user must re-authenticate with the new password.

The `settings` crate is designed to grow — future settings (e.g., auto-lock timeout, theme preferences) will be added as new modules inside it.

---

## Architecture

Three layers of change:

- **`crates/settings` (new):** Pure Rust library that owns all settings logic. Initially contains a `master_password` module. No Tauri dependency.
- **Backend (Tauri):** A new `settings_cmd.rs` command file that delegates to the `settings` crate. The `settings` crate is added to `src-tauri/Cargo.toml` and `Cargo.toml` workspace.
- **Frontend (React/TypeScript):** A new `SettingsPage` component, a sidebar nav entry, and small additions to `AppContext` and `App.tsx`.

---

## `crates/settings` Crate

### Cargo.toml dependencies

```toml
[dependencies]
encrypt = { path = "../encrypt" }
password-vault = { path = "../password-vault" }
```

### File structure

```
crates/settings/
  Cargo.toml
  src/
    lib.rs           # pub mod master_password;
    master_password.rs
```

### `master_password.rs`

**Public function:**

```rust
pub fn reset_master_password(
    db_path: &str,
    salt_path: &str,
    current_password: &str,
    new_password: &str,
    current_key: &[u8; 32],
) -> Result<(), String>
```

**Steps:**

1. **Verify current password.** Read `salt_path` from disk. Re-derive key from `current_password` + salt. Compare byte-for-byte with `current_key`. If mismatch → return `Err("Current password is incorrect")`.

2. **Load all credentials.** Open `SqliteStore` at `db_path` with `current_key`. Call `load_all()` to get all credentials in memory.

3. **Generate new key.** Call `generate_salt()` for a fresh salt. Call `derive_key(new_password, &new_salt)` for the new 32-byte key.

4. **Re-encrypt to temp files.** Open a new `SqliteStore` at `{db_path}.tmp` with the new key. Upsert all credentials.

5. **Atomic commit:**
   - Write new salt to `{salt_path}.tmp`
   - `rename("{salt_path}.tmp", salt_path)`
   - `rename("{db_path}.tmp", db_path)`

6. Return `Ok(())`. (Locking the in-memory state is the caller's responsibility.)

**Error safety:** If any step fails after step 4 begins, the `.tmp` files are abandoned but the live `vault.salt` and `vault.db` are untouched. The user can retry safely.

---

## Backend (Tauri)

### Workspace (`Cargo.toml`)

Add `"crates/settings"` to the `[workspace] members` array.

### `src-tauri/Cargo.toml`

Add:
```toml
settings = { path = "../crates/settings" }
```

### New file: `src-tauri/src/commands/settings_cmd.rs`

```rust
#[tauri::command]
pub fn reset_master_password(
    current_password: String,
    new_password: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let current_key = state.vault_key.lock().unwrap()
        .ok_or("vault is locked")?;
    settings::master_password::reset_master_password(
        &vault_db_path(),
        &vault_salt_path(),
        &current_password,
        &new_password,
        &current_key,
    )?;
    state.lock();
    Ok(())
}
```

> `vault_db_path()` and `vault_salt_path()` are currently private helpers in `vault_cmd.rs` — they should be extracted to a shared `paths.rs` module in `src-tauri/src/` so both command files can use them.

### `src-tauri/src/commands/mod.rs`

Add `pub mod settings_cmd;` and register `reset_master_password` in the Tauri command handler.

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
- Client-side: validate new password and confirm match before invoking.
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
    → settings::master_password::reset_master_password(...)
        → verify current password (re-derive + compare)
        → load all credentials from vault.db
        → generate new salt + key
        → re-encrypt to vault.db.tmp
        → atomic rename salt + db
    → state.lock()
  → frontend: setUnlocked(false) → UnlockPage shown
```

---

## What Is Not In Scope

- Export/import vault data
- Any other settings (theme, auto-lock timeout, etc.) — future modules in `crates/settings`
- Password strength meter for the new password
