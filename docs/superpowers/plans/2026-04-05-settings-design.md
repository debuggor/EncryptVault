# Settings & Reset Master Password Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a Settings page with Reset Master Password functionality, backed by a new `crates/settings` Rust crate.

**Architecture:** A pure Rust `settings` crate owns all re-encryption logic; a new `settings_cmd.rs` Tauri command delegates to it; the frontend adds a `SettingsPage` and sidebar entry. Path helpers shared between `vault_cmd` and `settings_cmd` are extracted to a `paths.rs` module first.

**Tech Stack:** Rust (Argon2 via `encrypt` crate, SQLite via `password-vault` crate), Tauri 2, React/TypeScript, Tailwind CSS.

---

## File Map

| Action | Path | Responsibility |
|--------|------|----------------|
| Create | `src-tauri/src/paths.rs` | `vault_db_path()` and `vault_salt_path()` shared helpers |
| Modify | `src-tauri/src/commands/vault_cmd.rs` | Remove private path fns, import from `paths` |
| Modify | `src-tauri/src/main.rs` | Add `paths` module, import and register `reset_master_password` |
| Create | `crates/settings/Cargo.toml` | Settings crate manifest |
| Create | `crates/settings/src/lib.rs` | `pub mod master_password;` |
| Create | `crates/settings/src/master_password.rs` | `reset_master_password` + tests |
| Modify | `Cargo.toml` | Add `crates/settings` to workspace members |
| Modify | `src-tauri/Cargo.toml` | Add `settings` dependency |
| Create | `src-tauri/src/commands/settings_cmd.rs` | `reset_master_password` Tauri command |
| Modify | `src-tauri/src/commands/mod.rs` | Add `pub mod settings_cmd;` |
| Modify | `src/context/AppContext.tsx` | Add `"settings"` to `Page` type |
| Modify | `src/components/Sidebar.tsx` | Add Settings entry to `PAGES` array |
| Create | `src/pages/SettingsPage.tsx` | Reset Master Password form |
| Modify | `src/App.tsx` | Import `SettingsPage`, add to `PageRouter` |

---

### Task 1: Extract path helpers to `paths.rs`

**Files:**
- Create: `src-tauri/src/paths.rs`
- Modify: `src-tauri/src/commands/vault_cmd.rs`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: Create `src-tauri/src/paths.rs`**

```rust
pub fn vault_db_path() -> String {
    let mut path = dirs_next::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("EncryptVault");
    std::fs::create_dir_all(&path).ok();
    path.push("vault.db");
    path.to_string_lossy().to_string()
}

pub fn vault_salt_path() -> String {
    vault_db_path().replace(".db", ".salt")
}
```

- [ ] **Step 2: Remove private path helpers from `vault_cmd.rs` and import from `paths`**

Replace the top of `src-tauri/src/commands/vault_cmd.rs` — remove the two private `fn vault_db_path()` and `fn vault_salt_path()` functions that currently appear on lines 6–17, and add an import so the file's use section becomes:

```rust
use tauri::State;
use crate::state::AppState;
use crate::paths::{vault_db_path, vault_salt_path};
use encrypt::{derive_key, generate_salt};
use password_vault::{Credential, Vault};
```

- [ ] **Step 3: Expose `paths` module in `src-tauri/src/main.rs`**

Add `mod paths;` to `src-tauri/src/main.rs` (alongside `mod state;` and `mod commands;`):

```rust
mod state;
mod paths;
mod commands;
```

- [ ] **Step 4: Verify compilation**

```bash
cargo build -p app
```

Expected: compiles with no errors.

- [ ] **Step 5: Commit**

```bash
git add src-tauri/src/paths.rs src-tauri/src/commands/vault_cmd.rs src-tauri/src/main.rs
git commit -m "refactor: extract vault path helpers to paths.rs"
```

---

### Task 2: Create `crates/settings` crate with `master_password` module

**Files:**
- Create: `crates/settings/Cargo.toml`
- Create: `crates/settings/src/lib.rs`
- Create: `crates/settings/src/master_password.rs`
- Modify: `Cargo.toml` (workspace)

- [ ] **Step 1: Write the failing test first**

Create `crates/settings/src/master_password.rs` with just the test module (function stub returns `unimplemented!`):

```rust
pub fn reset_master_password(
    db_path: &str,
    salt_path: &str,
    current_password: &str,
    new_password: &str,
    current_key: &[u8; 32],
) -> Result<(), String> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use encrypt::{derive_key, generate_salt};
    use password_vault::{Credential, Vault};
    use std::env;

    fn temp_db() -> String {
        env::temp_dir()
            .join(format!(
                "settings_test_{}.db",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .subsec_nanos()
            ))
            .to_string_lossy()
            .to_string()
    }

    fn temp_salt() -> String {
        temp_db().replace(".db", ".salt")
    }

    fn mk_cred(name: &str) -> Credential {
        Credential {
            id: String::new(),
            name: name.to_string(),
            username: "user".to_string(),
            password: "pass".to_string(),
            url: String::new(),
            notes: String::new(),
            created_at: 0,
            updated_at: 0,
        }
    }

    #[test]
    fn test_reset_success_data_preserved() {
        let db_path = temp_db();
        let salt_path = temp_salt();
        let current_password = "hunter2";
        let new_password = "newpass123";

        // Set up initial vault with salt file and two credentials
        let salt = generate_salt();
        std::fs::write(&salt_path, salt).unwrap();
        let key = derive_key(current_password, &salt).unwrap();
        {
            let mut vault = Vault::open(&db_path, &key).unwrap();
            vault.add(mk_cred("GitHub")).unwrap();
            vault.add(mk_cred("Gmail")).unwrap();
        }

        // Reset the master password
        reset_master_password(&db_path, &salt_path, current_password, new_password, &key)
            .unwrap();

        // Read the new salt and derive new key
        let new_salt_bytes = std::fs::read(&salt_path).unwrap();
        let new_salt: [u8; 16] = new_salt_bytes.try_into().unwrap();
        let new_key = derive_key(new_password, &new_salt).unwrap();

        // Verify new key is different from old key
        assert_ne!(key, new_key);

        // Verify new vault opens with new key and contains both credentials
        let vault = Vault::open(&db_path, &new_key).unwrap();
        let mut names: Vec<&str> = vault.list().iter().map(|c| c.name.as_str()).collect();
        names.sort();
        assert_eq!(names, vec!["GitHub", "Gmail"]);
    }

    #[test]
    fn test_reset_wrong_current_password_fails() {
        let db_path = temp_db();
        let salt_path = temp_salt();
        let current_password = "correct_password";

        let salt = generate_salt();
        std::fs::write(&salt_path, salt).unwrap();
        let key = derive_key(current_password, &salt).unwrap();
        {
            Vault::open(&db_path, &key).unwrap();
        }

        let result =
            reset_master_password(&db_path, &salt_path, "wrong_password", "newpass", &key);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Current password is incorrect");

        // Original salt file is untouched (rename of .tmp never happened)
        let salt_on_disk = std::fs::read(&salt_path).unwrap();
        assert_eq!(salt_on_disk, salt.as_ref());
    }
}
```

- [ ] **Step 2: Create `crates/settings/src/lib.rs`**

```rust
pub mod master_password;
```

- [ ] **Step 3: Create `crates/settings/Cargo.toml`**

```toml
[package]
name = "settings"
version = "0.1.0"
edition = "2021"

[dependencies]
encrypt = { path = "../encrypt" }
password-vault = { path = "../password-vault" }
```

- [ ] **Step 4: Add `crates/settings` to workspace**

In the root `Cargo.toml`, add `"crates/settings"` to the members array:

```toml
[workspace]
resolver = "2"
members = [
    "src-tauri",
    "crates/encrypt",
    "crates/password-vault",
    "crates/wallet",
    "crates/qr-engine",
    "crates/settings",
]
```

- [ ] **Step 5: Run tests to confirm they fail (not panic on unimplemented)**

```bash
cargo test -p settings 2>&1 | head -30
```

Expected: tests run and panic with `not yet implemented` (not a compile error).

- [ ] **Step 6: Implement `reset_master_password`**

Replace the `unimplemented!()` stub in `crates/settings/src/master_password.rs` with the full implementation:

```rust
pub fn reset_master_password(
    db_path: &str,
    salt_path: &str,
    current_password: &str,
    new_password: &str,
    current_key: &[u8; 32],
) -> Result<(), String> {
    // Step 1: Verify current password matches current_key
    let salt_bytes = std::fs::read(salt_path).map_err(|e| e.to_string())?;
    let salt: [u8; 16] = salt_bytes
        .try_into()
        .map_err(|_| "corrupt salt file".to_string())?;
    let derived = encrypt::derive_key(current_password, &salt)?;
    if derived != *current_key {
        return Err("Current password is incorrect".to_string());
    }

    // Step 2: Load all credentials from the current vault
    let vault = password_vault::Vault::open(db_path, current_key)?;
    let credentials: Vec<password_vault::Credential> =
        vault.list().into_iter().cloned().collect();

    // Step 3: Generate new salt and key
    let new_salt = encrypt::generate_salt();
    let new_key = encrypt::derive_key(new_password, &new_salt)?;

    // Step 4: Re-encrypt all credentials into a temp vault
    let db_tmp = format!("{db_path}.tmp");
    let mut vault_tmp = password_vault::Vault::open(&db_tmp, &new_key)?;
    for cred in credentials {
        vault_tmp.update(cred)?;
    }

    // Step 5: Atomic commit — rename temp files over live files
    let salt_tmp = format!("{salt_path}.tmp");
    std::fs::write(&salt_tmp, new_salt).map_err(|e| e.to_string())?;
    std::fs::rename(&salt_tmp, salt_path).map_err(|e| e.to_string())?;
    std::fs::rename(&db_tmp, db_path).map_err(|e| e.to_string())?;

    Ok(())
}
```

- [ ] **Step 7: Run tests to verify they pass**

```bash
cargo test -p settings -- --test-output immediate
```

Expected: `test_reset_success_data_preserved ... ok`, `test_reset_wrong_current_password_fails ... ok`.

- [ ] **Step 8: Commit**

```bash
git add crates/settings/ Cargo.toml
git commit -m "feat(settings): add settings crate with reset_master_password"
```

---

### Task 3: Wire settings crate into Tauri backend

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/commands/settings_cmd.rs`
- Modify: `src-tauri/src/commands/mod.rs`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: Add `settings` dependency to `src-tauri/Cargo.toml`**

In `src-tauri/Cargo.toml`, add after the `qr-engine` line:

```toml
settings = { path = "../crates/settings" }
```

- [ ] **Step 2: Create `src-tauri/src/commands/settings_cmd.rs`**

```rust
use tauri::State;
use crate::state::AppState;
use crate::paths::{vault_db_path, vault_salt_path};

#[tauri::command]
pub fn reset_master_password(
    current_password: String,
    new_password: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let current_key = state
        .vault_key
        .lock()
        .unwrap()
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

- [ ] **Step 3: Register the module in `src-tauri/src/commands/mod.rs`**

```rust
pub mod encrypt_cmd;
pub mod vault_cmd;
pub mod wallet_cmd;
pub mod qr_cmd;
pub mod settings_cmd;
```

- [ ] **Step 4: Import and register command in `src-tauri/src/main.rs`**

Add the import:
```rust
use commands::settings_cmd::reset_master_password as cmd_reset_master_password;
```

Add to `invoke_handler!` macro (after `cmd_decode_qr_frame`):
```rust
cmd_reset_master_password,
```

The full updated `main.rs`:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod paths;
mod commands;

use state::AppState;
use commands::{
    encrypt_cmd::{cmd_decrypt_file, cmd_decrypt_text, cmd_encrypt_file, cmd_encrypt_text},
    qr_cmd::{cmd_decode_qr_file, cmd_decode_qr_frame, cmd_generate_qr},
    vault_cmd::{
        add_credential, delete_credential, is_unlocked, list_credentials, lock_vault,
        search_credentials, unlock_vault, update_credential,
    },
    wallet_cmd::{derive_btc_address, derive_eth_address, import_wallet, setup_wallet, sign_eth_tx},
    settings_cmd::reset_master_password,
};

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            is_unlocked,
            unlock_vault,
            lock_vault,
            add_credential,
            list_credentials,
            update_credential,
            delete_credential,
            search_credentials,
            cmd_encrypt_text,
            cmd_decrypt_text,
            cmd_encrypt_file,
            cmd_decrypt_file,
            setup_wallet,
            import_wallet,
            derive_eth_address,
            derive_btc_address,
            sign_eth_tx,
            cmd_generate_qr,
            cmd_decode_qr_file,
            cmd_decode_qr_frame,
            reset_master_password,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 5: Verify compilation**

```bash
cargo build -p app
```

Expected: compiles with no errors or warnings about unused imports.

- [ ] **Step 6: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/commands/settings_cmd.rs src-tauri/src/commands/mod.rs src-tauri/src/main.rs
git commit -m "feat(backend): add reset_master_password Tauri command"
```

---

### Task 4: Frontend — Settings page

**Files:**
- Modify: `src/context/AppContext.tsx`
- Modify: `src/components/Sidebar.tsx`
- Create: `src/pages/SettingsPage.tsx`
- Modify: `src/App.tsx`

- [ ] **Step 1: Add `"settings"` to the `Page` type in `AppContext.tsx`**

Change line 3 from:
```ts
type Page = "encrypt" | "vault" | "wallet" | "qr";
```
to:
```ts
type Page = "encrypt" | "vault" | "wallet" | "qr" | "settings";
```

- [ ] **Step 2: Add Settings entry to Sidebar `PAGES` array**

In `src/components/Sidebar.tsx`, change the `PAGES` array from:
```ts
const PAGES = [
  { id: "encrypt" as const, label: "Encrypt / Decrypt", icon: "🔐" },
  { id: "vault"   as const, label: "Password Vault",     icon: "🗝️" },
  { id: "wallet"  as const, label: "Wallet",              icon: "💼" },
  { id: "qr"      as const, label: "QR Code",             icon: "⬛" },
];
```
to:
```ts
const PAGES = [
  { id: "encrypt"  as const, label: "Encrypt / Decrypt", icon: "🔐" },
  { id: "vault"    as const, label: "Password Vault",     icon: "🗝️" },
  { id: "wallet"   as const, label: "Wallet",             icon: "💼" },
  { id: "qr"       as const, label: "QR Code",            icon: "⬛" },
  { id: "settings" as const, label: "Settings",           icon: "⚙️" },
];
```

- [ ] **Step 3: Create `src/pages/SettingsPage.tsx`**

```tsx
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

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    if (newPassword !== confirmPassword) {
      setError("New passwords do not match");
      return;
    }
    setLoading(true);
    setError(null);
    try {
      await invoke("reset_master_password", { currentPassword, newPassword });
      setUnlocked(false);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="max-w-md">
      <h1 className="text-2xl font-bold text-gray-900 mb-1">Settings</h1>
      <p className="text-sm text-gray-500 mb-6">
        Change your master password. The vault will lock after a successful reset.
      </p>
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
  );
}
```

- [ ] **Step 4: Wire `SettingsPage` into `App.tsx`**

```tsx
import { AppProvider, useApp } from "./context/AppContext";
import Sidebar from "./components/Sidebar";
import UnlockPage from "./pages/UnlockPage";
import EncryptPage from "./pages/EncryptPage";
import VaultPage from "./pages/VaultPage";
import WalletPage from "./pages/WalletPage";
import QRPage from "./pages/QRPage";
import SettingsPage from "./pages/SettingsPage";

function PageRouter() {
  const { isUnlocked, currentPage } = useApp();
  if (!isUnlocked) return <UnlockPage />;
  return (
    <div className="flex h-screen bg-gray-50">
      <Sidebar />
      <main className="flex-1 overflow-auto p-8">
        {currentPage === "encrypt"  && <EncryptPage />}
        {currentPage === "vault"    && <VaultPage />}
        {currentPage === "wallet"   && <WalletPage />}
        {currentPage === "qr"       && <QRPage />}
        {currentPage === "settings" && <SettingsPage />}
      </main>
    </div>
  );
}

export default function App() {
  return (
    <AppProvider>
      <PageRouter />
    </AppProvider>
  );
}
```

- [ ] **Step 5: Verify TypeScript compiles cleanly**

```bash
npm run build
```

Expected: exits 0, no TypeScript errors.

- [ ] **Step 6: Commit**

```bash
git add src/context/AppContext.tsx src/components/Sidebar.tsx src/pages/SettingsPage.tsx src/App.tsx
git commit -m "feat(frontend): add Settings page with reset master password form"
```
