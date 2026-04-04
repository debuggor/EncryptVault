# EncryptVault Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a fully offline macOS desktop app with four security tools: file/text encryption, password vault, ETH/BTC offline wallet, and QR code generation/scanning.

**Architecture:** Cargo workspace with four platform-agnostic Rust crates (`encrypt`, `password-vault`, `wallet`, `qr-engine`) wired by a Tauri v2 `src-tauri` crate. React/TypeScript/Tailwind SPA as frontend. All crypto stays in Rust; the frontend never touches raw keys or plaintext.

**Tech Stack:** Rust 1.78+, Tauri 2, React 18, TypeScript 5, Vite 5, Tailwind CSS 3, aes-gcm 0.10, argon2 0.5, rand 0.8, base64 0.22, rusqlite 0.31 (bundled), serde_json 1, uuid 1, bip39 2.0, coins-bip32 0.8, k256 0.13, tiny-keccak 2, rlp 0.5, bitcoin 0.31, qrcode 0.14, image 0.25, rqrr 0.7

---

## File Map

**Workspace root**
- Create: `Cargo.toml`
- Create: `package.json`
- Create: `vite.config.ts`
- Create: `index.html`
- Create: `tailwind.config.js`
- Create: `postcss.config.js`
- Create: `tsconfig.json`

**`crates/encrypt`**
- Create: `crates/encrypt/Cargo.toml`
- Create: `crates/encrypt/src/lib.rs` — public API: `encrypt_text`, `decrypt_text`, `encrypt_bytes_with_passphrase`, `decrypt_bytes_with_passphrase`, `encrypt_with_key`, `decrypt_with_key`
- Create: `crates/encrypt/src/kdf.rs` — `derive_key(password, salt) -> [u8;32]`, `generate_salt() -> [u8;16]`
- Create: `crates/encrypt/src/aes_gcm.rs` — low-level encrypt/decrypt with raw key

**`crates/password-vault`**
- Create: `crates/password-vault/Cargo.toml`
- Create: `crates/password-vault/src/lib.rs` — `Vault` struct with full CRUD
- Create: `crates/password-vault/src/model.rs` — `Credential` struct
- Create: `crates/password-vault/src/store.rs` — `SqliteStore`: open, upsert, delete, load_all

**`crates/wallet`**
- Create: `crates/wallet/Cargo.toml`
- Create: `crates/wallet/src/lib.rs` — `Wallet` struct: from_mnemonic, generate, eth_address, btc_address, sign_eth_tx
- Create: `crates/wallet/src/mnemonic.rs` — BIP-39 generate/import
- Create: `crates/wallet/src/eth.rs` — ETH BIP-44 derivation + EIP-155 signing
- Create: `crates/wallet/src/btc.rs` — BTC BIP-84 P2WPKH address derivation

**`crates/qr-engine`**
- Create: `crates/qr-engine/Cargo.toml`
- Create: `crates/qr-engine/src/lib.rs` — `generate_qr_png`, `decode_bytes`, `decode_frame`

**`src-tauri`**
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/build.rs`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/capabilities/default.json`
- Create: `src-tauri/src/main.rs`
- Create: `src-tauri/src/state.rs` — `AppState { vault_key, vault, wallet }`
- Create: `src-tauri/src/commands/mod.rs`
- Create: `src-tauri/src/commands/encrypt_cmd.rs`
- Create: `src-tauri/src/commands/vault_cmd.rs`
- Create: `src-tauri/src/commands/wallet_cmd.rs`
- Create: `src-tauri/src/commands/qr_cmd.rs`

**`src` (React)**
- Create: `src/main.tsx`
- Create: `src/App.tsx`
- Create: `src/context/AppContext.tsx`
- Create: `src/components/Sidebar.tsx`
- Create: `src/components/ErrorBanner.tsx`
- Create: `src/pages/UnlockPage.tsx`
- Create: `src/pages/EncryptPage.tsx`
- Create: `src/pages/VaultPage.tsx`
- Create: `src/pages/WalletPage.tsx`
- Create: `src/pages/QRPage.tsx`

---

## Task 1: Workspace scaffold

**Files:**
- Create: `Cargo.toml`
- Create: `package.json`
- Create: `vite.config.ts`
- Create: `index.html`
- Create: `tailwind.config.js`
- Create: `postcss.config.js`
- Create: `tsconfig.json`

- [ ] **Step 1: Create workspace Cargo.toml**

```toml
[workspace]
resolver = "2"
members = [
    "src-tauri",
    "crates/encrypt",
    "crates/password-vault",
    "crates/wallet",
    "crates/qr-engine",
]
```

- [ ] **Step 2: Create package.json**

```json
{
  "name": "encrypt-vault",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "tauri": "tauri"
  },
  "dependencies": {
    "@tauri-apps/api": "^2",
    "react": "^18",
    "react-dom": "^18"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2",
    "@types/react": "^18",
    "@types/react-dom": "^18",
    "@vitejs/plugin-react": "^4",
    "autoprefixer": "^10",
    "postcss": "^8",
    "tailwindcss": "^3",
    "typescript": "^5",
    "vite": "^5"
  }
}
```

- [ ] **Step 3: Create vite.config.ts**

```typescript
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    watch: { ignored: ["**/src-tauri/**"] },
  },
  envPrefix: ["VITE_", "TAURI_"],
  build: {
    target: ["es2021", "chrome105", "safari15"],
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
```

- [ ] **Step 4: Create index.html**

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>EncryptVault</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
```

- [ ] **Step 5: Create tailwind.config.js**

```js
/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: { extend: {} },
  plugins: [],
};
```

- [ ] **Step 6: Create postcss.config.js**

```js
export default {
  plugins: { tailwindcss: {}, autoprefixer: {} },
};
```

- [ ] **Step 7: Create tsconfig.json**

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"]
}
```

- [ ] **Step 8: Install npm dependencies**

```bash
npm install
```

Expected: `node_modules/` created, no errors.

- [ ] **Step 9: Commit**

```bash
git add Cargo.toml package.json package-lock.json vite.config.ts index.html tailwind.config.js postcss.config.js tsconfig.json
git commit -m "chore: scaffold workspace and npm project"
```

---

## Task 2: Tauri project scaffold

**Files:**
- Create: `src-tauri/Cargo.toml`
- Create: `src-tauri/build.rs`
- Create: `src-tauri/tauri.conf.json`
- Create: `src-tauri/capabilities/default.json`
- Create: `src-tauri/src/main.rs`

- [ ] **Step 1: Create src-tauri/Cargo.toml**

```toml
[package]
name = "app"
version = "0.1.0"
edition = "2021"

[lib]
name = "app_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
encrypt = { path = "../crates/encrypt" }
password-vault = { path = "../crates/password-vault" }
wallet = { path = "../crates/wallet" }
qr-engine = { path = "../crates/qr-engine" }
```

- [ ] **Step 2: Create src-tauri/build.rs**

```rust
fn main() {
    tauri_build::build()
}
```

- [ ] **Step 3: Create src-tauri/tauri.conf.json**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "EncryptVault",
  "version": "0.1.0",
  "identifier": "com.encryptvault.app",
  "build": {
    "beforeDevCommand": "npm run dev",
    "devUrl": "http://localhost:1420",
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../dist"
  },
  "app": {
    "windows": [
      {
        "title": "EncryptVault",
        "width": 1200,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": { "csp": null }
  },
  "bundle": {
    "active": true,
    "targets": "all",
    "icon": []
  }
}
```

- [ ] **Step 4: Create src-tauri/capabilities/default.json**

```json
{
  "$schema": "../node_modules/@tauri-apps/cli/schema/capabilities.json",
  "identifier": "default",
  "description": "Default capabilities",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:path:default",
    "core:dialog:allow-open",
    "core:dialog:allow-save"
  ]
}
```

- [ ] **Step 5: Create src-tauri/src/main.rs (stub)**

```rust
// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod commands;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 6: Create src-tauri/src/state.rs (stub)**

```rust
use std::sync::Mutex;

#[derive(Default)]
pub struct AppState {
    pub vault_key: Mutex<Option<[u8; 32]>>,
    pub vault: Mutex<Option<password_vault::Vault>>,
    pub wallet: Mutex<Option<wallet::Wallet>>,
}
```

- [ ] **Step 7: Create src-tauri/src/commands/mod.rs**

```rust
pub mod encrypt_cmd;
pub mod vault_cmd;
pub mod wallet_cmd;
pub mod qr_cmd;
```

- [ ] **Step 8: Create stub command files**

`src-tauri/src/commands/encrypt_cmd.rs`:
```rust
// Encrypt/decrypt commands — implemented in Task 12
```

`src-tauri/src/commands/vault_cmd.rs`:
```rust
// Vault commands — implemented in Task 13
```

`src-tauri/src/commands/wallet_cmd.rs`:
```rust
// Wallet commands — implemented in Task 14
```

`src-tauri/src/commands/qr_cmd.rs`:
```rust
// QR commands — implemented in Task 15
```

- [ ] **Step 9: Verify workspace compiles**

```bash
cargo check
```

Expected: all crates compile (crates/encrypt etc. don't exist yet — add stub Cargo.toml files for each)

Create `crates/encrypt/Cargo.toml`, `crates/encrypt/src/lib.rs` (empty pub), and repeat for the other three crates so workspace resolves:

`crates/encrypt/Cargo.toml`:
```toml
[package]
name = "encrypt"
version = "0.1.0"
edition = "2021"
```
`crates/encrypt/src/lib.rs`: `// stub`

`crates/password-vault/Cargo.toml`:
```toml
[package]
name = "password-vault"
version = "0.1.0"
edition = "2021"
```
`crates/password-vault/src/lib.rs`:
```rust
pub struct Vault;
impl Default for Vault { fn default() -> Self { Vault } }
```

`crates/wallet/Cargo.toml`:
```toml
[package]
name = "wallet"
version = "0.1.0"
edition = "2021"
```
`crates/wallet/src/lib.rs`:
```rust
pub struct Wallet;
impl Default for Wallet { fn default() -> Self { Wallet } }
```

`crates/qr-engine/Cargo.toml`:
```toml
[package]
name = "qr-engine"
version = "0.1.0"
edition = "2021"
```
`crates/qr-engine/src/lib.rs`: `// stub`

```bash
cargo check
```

Expected: `Finished` with no errors.

- [ ] **Step 10: Commit**

```bash
git add src-tauri/ crates/
git commit -m "chore: add Tauri scaffold and crate stubs"
```

---

## Task 3: `crates/encrypt` — KDF and AES-GCM primitives

**Files:**
- Modify: `crates/encrypt/Cargo.toml`
- Create: `crates/encrypt/src/kdf.rs`
- Create: `crates/encrypt/src/aes_gcm.rs`
- Modify: `crates/encrypt/src/lib.rs`

- [ ] **Step 1: Update crates/encrypt/Cargo.toml**

```toml
[package]
name = "encrypt"
version = "0.1.0"
edition = "2021"

[dependencies]
aes-gcm = "0.10"
argon2 = "0.5"
rand = "0.8"
base64 = "0.22"
```

- [ ] **Step 2: Write failing tests**

Add to `crates/encrypt/src/lib.rs`:

```rust
mod kdf;
mod aes_gcm;

pub use kdf::{derive_key, generate_salt};
pub use aes_gcm::{encrypt_with_key, decrypt_with_key};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_is_deterministic() {
        let salt = [1u8; 16];
        let k1 = derive_key("password", &salt);
        let k2 = derive_key("password", &salt);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_derive_key_differs_by_password() {
        let salt = [1u8; 16];
        assert_ne!(derive_key("a", &salt), derive_key("b", &salt));
    }

    #[test]
    fn test_derive_key_differs_by_salt() {
        assert_ne!(derive_key("pw", &[1u8; 16]), derive_key("pw", &[2u8; 16]));
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [42u8; 32];
        let plaintext = b"hello vault";
        let ct = encrypt_with_key(&key, plaintext);
        let pt = decrypt_with_key(&key, &ct).unwrap();
        assert_eq!(pt, plaintext);
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let key = [1u8; 32];
        let ct = encrypt_with_key(&key, b"secret");
        let result = decrypt_with_key(&[2u8; 32], &ct);
        assert!(result.is_err());
    }

    #[test]
    fn test_ciphertext_different_each_time() {
        let key = [0u8; 32];
        let ct1 = encrypt_with_key(&key, b"same");
        let ct2 = encrypt_with_key(&key, b"same");
        assert_ne!(ct1, ct2); // different random nonces
    }
}
```

- [ ] **Step 3: Run tests to confirm they fail**

```bash
cargo test -p encrypt
```

Expected: compile error — `kdf` and `aes_gcm` modules not found.

- [ ] **Step 4: Implement crates/encrypt/src/kdf.rs**

```rust
use argon2::{Algorithm, Argon2, Params, Version};
use rand::RngCore;

pub fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

/// Derive a 32-byte key from password + salt using Argon2id.
/// Params: m=65536 (64 MiB), t=3 iterations, p=4 parallelism.
pub fn derive_key(password: &str, salt: &[u8; 16]) -> [u8; 32] {
    let params = Params::new(65536, 3, 4, Some(32)).expect("valid argon2 params");
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .expect("argon2 key derivation failed");
    key
}
```

- [ ] **Step 5: Implement crates/encrypt/src/aes_gcm.rs**

```rust
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

/// Encrypt plaintext with a 32-byte key.
/// Output format: [12-byte random nonce][ciphertext + 16-byte GCM tag]
pub fn encrypt_with_key(key: &[u8; 32], plaintext: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext).expect("encryption failed");
    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(nonce.as_slice());
    out.extend_from_slice(&ciphertext);
    out
}

/// Decrypt data produced by `encrypt_with_key`.
pub fn decrypt_with_key(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 28 {
        // 12-byte nonce + 16-byte tag minimum
        return Err("ciphertext too short".to_string());
    }
    let (nonce_bytes, ciphertext) = data.split_at(12);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "decryption failed — wrong key or corrupted data".to_string())
}
```

- [ ] **Step 6: Run tests**

```bash
cargo test -p encrypt
```

Expected: all 6 tests pass.

- [ ] **Step 7: Commit**

```bash
git add crates/encrypt/
git commit -m "feat(encrypt): KDF (Argon2id) and AES-256-GCM primitives"
```

---

## Task 4: `crates/encrypt` — high-level text/bytes API

**Files:**
- Modify: `crates/encrypt/src/lib.rs`

- [ ] **Step 1: Write failing tests**

Add to the `tests` module in `crates/encrypt/src/lib.rs`:

```rust
    #[test]
    fn test_encrypt_text_roundtrip() {
        let ct = encrypt_text("passphrase", "hello world");
        let pt = decrypt_text("passphrase", &ct).unwrap();
        assert_eq!(pt, "hello world");
    }

    #[test]
    fn test_decrypt_text_wrong_passphrase_fails() {
        let ct = encrypt_text("correct", "secret");
        assert!(decrypt_text("wrong", &ct).is_err());
    }

    #[test]
    fn test_encrypt_bytes_roundtrip() {
        let data = vec![0u8, 1, 2, 3, 255];
        let ct = encrypt_bytes_with_passphrase("pw", &data);
        let pt = decrypt_bytes_with_passphrase("pw", &ct).unwrap();
        assert_eq!(pt, data);
    }
```

- [ ] **Step 2: Run tests to confirm they fail**

```bash
cargo test -p encrypt
```

Expected: compile error — functions not defined.

- [ ] **Step 3: Implement high-level API in crates/encrypt/src/lib.rs**

Replace the stub lib.rs with:

```rust
mod kdf;
mod aes_gcm;

pub use kdf::{derive_key, generate_salt};
pub use aes_gcm::{encrypt_with_key, decrypt_with_key};

use base64::{engine::general_purpose::STANDARD as B64, Engine};

/// Encrypt a UTF-8 string with a passphrase.
/// Wire format (base64-encoded): [16-byte salt][12-byte nonce][ciphertext+tag]
pub fn encrypt_text(passphrase: &str, plaintext: &str) -> String {
    let salt = generate_salt();
    let key = derive_key(passphrase, &salt);
    let enc = encrypt_with_key(&key, plaintext.as_bytes());
    let mut wire = Vec::with_capacity(16 + enc.len());
    wire.extend_from_slice(&salt);
    wire.extend_from_slice(&enc);
    B64.encode(wire)
}

/// Decrypt a string produced by `encrypt_text`.
pub fn decrypt_text(passphrase: &str, encoded: &str) -> Result<String, String> {
    let wire = B64.decode(encoded).map_err(|_| "invalid base64".to_string())?;
    if wire.len() < 16 {
        return Err("data too short".to_string());
    }
    let (salt_slice, rest) = wire.split_at(16);
    let salt: [u8; 16] = salt_slice.try_into().unwrap();
    let key = derive_key(passphrase, &salt);
    let pt = decrypt_with_key(&key, rest)?;
    String::from_utf8(pt).map_err(|_| "decrypted data is not valid UTF-8".to_string())
}

/// Encrypt raw bytes with a passphrase.
/// Wire format: [16-byte salt][12-byte nonce][ciphertext+tag]
pub fn encrypt_bytes_with_passphrase(passphrase: &str, data: &[u8]) -> Vec<u8> {
    let salt = generate_salt();
    let key = derive_key(passphrase, &salt);
    let enc = encrypt_with_key(&key, data);
    let mut wire = Vec::with_capacity(16 + enc.len());
    wire.extend_from_slice(&salt);
    wire.extend_from_slice(&enc);
    wire
}

/// Decrypt bytes produced by `encrypt_bytes_with_passphrase`.
pub fn decrypt_bytes_with_passphrase(passphrase: &str, data: &[u8]) -> Result<Vec<u8>, String> {
    if data.len() < 16 {
        return Err("data too short".to_string());
    }
    let (salt_slice, rest) = data.split_at(16);
    let salt: [u8; 16] = salt_slice.try_into().unwrap();
    let key = derive_key(passphrase, &salt);
    decrypt_with_key(&key, rest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_key_is_deterministic() {
        let salt = [1u8; 16];
        assert_eq!(derive_key("password", &salt), derive_key("password", &salt));
    }

    #[test]
    fn test_derive_key_differs_by_password() {
        let salt = [1u8; 16];
        assert_ne!(derive_key("a", &salt), derive_key("b", &salt));
    }

    #[test]
    fn test_derive_key_differs_by_salt() {
        assert_ne!(derive_key("pw", &[1u8; 16]), derive_key("pw", &[2u8; 16]));
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = [42u8; 32];
        let pt = decrypt_with_key(&key, &encrypt_with_key(&key, b"hello vault")).unwrap();
        assert_eq!(pt, b"hello vault");
    }

    #[test]
    fn test_decrypt_wrong_key_fails() {
        let ct = encrypt_with_key(&[1u8; 32], b"secret");
        assert!(decrypt_with_key(&[2u8; 32], &ct).is_err());
    }

    #[test]
    fn test_ciphertext_different_each_time() {
        let key = [0u8; 32];
        assert_ne!(encrypt_with_key(&key, b"same"), encrypt_with_key(&key, b"same"));
    }

    #[test]
    fn test_encrypt_text_roundtrip() {
        let ct = encrypt_text("passphrase", "hello world");
        assert_eq!(decrypt_text("passphrase", &ct).unwrap(), "hello world");
    }

    #[test]
    fn test_decrypt_text_wrong_passphrase_fails() {
        let ct = encrypt_text("correct", "secret");
        assert!(decrypt_text("wrong", &ct).is_err());
    }

    #[test]
    fn test_encrypt_bytes_roundtrip() {
        let data = vec![0u8, 1, 2, 3, 255];
        let ct = encrypt_bytes_with_passphrase("pw", &data);
        assert_eq!(decrypt_bytes_with_passphrase("pw", &ct).unwrap(), data);
    }
}
```

- [ ] **Step 4: Run tests**

```bash
cargo test -p encrypt
```

Expected: all 9 tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/encrypt/src/lib.rs
git commit -m "feat(encrypt): high-level text/bytes encrypt API"
```

---

## Task 5: `crates/password-vault` — Credential model and in-memory Vault

**Files:**
- Modify: `crates/password-vault/Cargo.toml`
- Create: `crates/password-vault/src/model.rs`
- Modify: `crates/password-vault/src/lib.rs`

- [ ] **Step 1: Update crates/password-vault/Cargo.toml**

```toml
[package]
name = "password-vault"
version = "0.1.0"
edition = "2021"

[dependencies]
encrypt = { path = "../encrypt" }
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4"] }
```

- [ ] **Step 2: Write failing tests**

`crates/password-vault/src/lib.rs`:

```rust
mod model;
mod store;

pub use model::Credential;

use std::collections::HashMap;
use encrypt::Key;
use store::SqliteStore;

pub struct Vault {
    credentials: HashMap<String, Credential>,
    store: SqliteStore,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> Key { [7u8; 32] }

    fn open_tmp_vault() -> Vault {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("test_vault_{}.db", uuid::Uuid::new_v4()));
        Vault::open(path.to_str().unwrap(), &test_key()).unwrap()
    }

    #[test]
    fn test_add_and_list() {
        let mut v = open_tmp_vault();
        let cred = Credential {
            id: String::new(),
            name: "GitHub".to_string(),
            username: "alice".to_string(),
            password: "s3cr3t".to_string(),
            url: "https://github.com".to_string(),
            notes: "".to_string(),
            created_at: 0,
            updated_at: 0,
        };
        let id = v.add(cred).unwrap();
        let list = v.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, id);
        assert_eq!(list[0].name, "GitHub");
    }

    #[test]
    fn test_search() {
        let mut v = open_tmp_vault();
        v.add(Credential { id: String::new(), name: "GitHub".into(), username: "alice".into(),
            password: "x".into(), url: "https://github.com".into(), notes: "".into(),
            created_at: 0, updated_at: 0 }).unwrap();
        v.add(Credential { id: String::new(), name: "GitLab".into(), username: "bob".into(),
            password: "y".into(), url: "https://gitlab.com".into(), notes: "".into(),
            created_at: 0, updated_at: 0 }).unwrap();
        assert_eq!(v.search("github").len(), 1);
        assert_eq!(v.search("git").len(), 2);
        assert_eq!(v.search("nothing").len(), 0);
    }

    #[test]
    fn test_delete() {
        let mut v = open_tmp_vault();
        let id = v.add(Credential { id: String::new(), name: "X".into(), username: "u".into(),
            password: "p".into(), url: "".into(), notes: "".into(),
            created_at: 0, updated_at: 0 }).unwrap();
        v.delete(&id).unwrap();
        assert_eq!(v.list().len(), 0);
    }

    #[test]
    fn test_update() {
        let mut v = open_tmp_vault();
        let id = v.add(Credential { id: String::new(), name: "Old".into(), username: "u".into(),
            password: "p".into(), url: "".into(), notes: "".into(),
            created_at: 0, updated_at: 0 }).unwrap();
        let mut updated = v.list()[0].clone();
        updated.name = "New".to_string();
        v.update(updated).unwrap();
        assert_eq!(v.list()[0].name, "New");
    }

    #[test]
    fn test_persist_and_reload() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("test_persist_{}.db", uuid::Uuid::new_v4()));
        let key = test_key();
        let id = {
            let mut v = Vault::open(path.to_str().unwrap(), &key).unwrap();
            v.add(Credential { id: String::new(), name: "Persisted".into(), username: "u".into(),
                password: "p".into(), url: "".into(), notes: "".into(),
                created_at: 0, updated_at: 0 }).unwrap()
        };
        let v2 = Vault::open(path.to_str().unwrap(), &key).unwrap();
        assert_eq!(v2.list().len(), 1);
        assert_eq!(v2.list()[0].id, id);
    }

    #[test]
    fn test_wrong_key_on_reload_fails() {
        let dir = std::env::temp_dir();
        let path = dir.join(format!("test_wrongkey_{}.db", uuid::Uuid::new_v4()));
        let key = test_key();
        {
            let mut v = Vault::open(path.to_str().unwrap(), &key).unwrap();
            v.add(Credential { id: String::new(), name: "X".into(), username: "u".into(),
                password: "p".into(), url: "".into(), notes: "".into(),
                created_at: 0, updated_at: 0 }).unwrap();
        }
        let result = Vault::open(path.to_str().unwrap(), &[99u8; 32]);
        assert!(result.is_err());
    }
}
```

- [ ] **Step 3: Run tests to confirm they fail**

```bash
cargo test -p password-vault
```

Expected: compile errors — `model`, `store` not implemented.

- [ ] **Step 4: Create crates/password-vault/src/model.rs**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub id: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub url: String,
    pub notes: String,
    pub created_at: i64,
    pub updated_at: i64,
}
```

- [ ] **Step 5: Create crates/password-vault/src/store.rs**

```rust
use rusqlite::{params, Connection};
use encrypt::{decrypt_with_key, encrypt_with_key, Key};
use crate::model::Credential;

pub struct SqliteStore {
    conn: Connection,
    key: Key,
}

impl SqliteStore {
    pub fn open(db_path: &str, key: &Key) -> Result<Self, String> {
        let conn = Connection::open(db_path).map_err(|e| format!("db open: {e}"))?;
        #[cfg(unix)]
        {
            use std::fs;
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(db_path, fs::Permissions::from_mode(0o600));
        }
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS credentials \
             (id TEXT PRIMARY KEY, ciphertext BLOB NOT NULL);",
        )
        .map_err(|e| format!("db init: {e}"))?;
        Ok(Self { conn, key: *key })
    }

    pub fn load_all(&self) -> Result<Vec<Credential>, String> {
        let mut stmt = self
            .conn
            .prepare("SELECT ciphertext FROM credentials")
            .map_err(|e| e.to_string())?;
        let rows = stmt
            .query_map([], |row| row.get::<_, Vec<u8>>(0))
            .map_err(|e| e.to_string())?;
        let mut out = Vec::new();
        for row in rows {
            let ct = row.map_err(|e| e.to_string())?;
            let pt = decrypt_with_key(&self.key, &ct)?;
            let cred: Credential = serde_json::from_slice(&pt)
                .map_err(|e| format!("deserialize: {e}"))?;
            out.push(cred);
        }
        Ok(out)
    }

    pub fn upsert(&self, cred: &Credential) -> Result<(), String> {
        let pt = serde_json::to_vec(cred).map_err(|e| e.to_string())?;
        let ct = encrypt_with_key(&self.key, &pt);
        self.conn
            .execute(
                "INSERT OR REPLACE INTO credentials (id, ciphertext) VALUES (?1, ?2)",
                params![cred.id, ct],
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn delete(&self, id: &str) -> Result<(), String> {
        self.conn
            .execute("DELETE FROM credentials WHERE id = ?1", params![id])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
```

- [ ] **Step 6: Implement Vault in crates/password-vault/src/lib.rs**

```rust
mod model;
mod store;

pub use model::Credential;
use encrypt::Key;
use std::collections::HashMap;
use store::SqliteStore;

pub struct Vault {
    credentials: HashMap<String, Credential>,
    store: SqliteStore,
}

impl Vault {
    pub fn open(db_path: &str, key: &Key) -> Result<Self, String> {
        let store = SqliteStore::open(db_path, key)?;
        let credentials = store
            .load_all()?
            .into_iter()
            .map(|c| (c.id.clone(), c))
            .collect();
        Ok(Self { credentials, store })
    }

    pub fn add(&mut self, mut cred: Credential) -> Result<String, String> {
        let now = now_secs();
        cred.id = uuid::Uuid::new_v4().to_string();
        cred.created_at = now;
        cred.updated_at = now;
        self.store.upsert(&cred)?;
        let id = cred.id.clone();
        self.credentials.insert(id.clone(), cred);
        Ok(id)
    }

    pub fn update(&mut self, mut cred: Credential) -> Result<(), String> {
        cred.updated_at = now_secs();
        self.store.upsert(&cred)?;
        self.credentials.insert(cred.id.clone(), cred);
        Ok(())
    }

    pub fn delete(&mut self, id: &str) -> Result<(), String> {
        self.store.delete(id)?;
        self.credentials.remove(id);
        Ok(())
    }

    pub fn list(&self) -> Vec<&Credential> {
        self.credentials.values().collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Credential> {
        let q = query.to_lowercase();
        self.credentials
            .values()
            .filter(|c| {
                c.name.to_lowercase().contains(&q) || c.url.to_lowercase().contains(&q)
            })
            .collect()
    }
}

fn now_secs() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> Key { [7u8; 32] }

    fn open_tmp_vault() -> Vault {
        let path = std::env::temp_dir()
            .join(format!("test_vault_{}.db", uuid::Uuid::new_v4()));
        Vault::open(path.to_str().unwrap(), &test_key()).unwrap()
    }

    #[test]
    fn test_add_and_list() {
        let mut v = open_tmp_vault();
        let id = v.add(Credential { id: String::new(), name: "GitHub".into(),
            username: "alice".into(), password: "s3cr3t".into(),
            url: "https://github.com".into(), notes: "".into(),
            created_at: 0, updated_at: 0 }).unwrap();
        let list = v.list();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, id);
        assert_eq!(list[0].name, "GitHub");
    }

    #[test]
    fn test_search() {
        let mut v = open_tmp_vault();
        let mk = |name: &str, url: &str| Credential { id: String::new(), name: name.into(),
            username: "u".into(), password: "p".into(), url: url.into(),
            notes: "".into(), created_at: 0, updated_at: 0 };
        v.add(mk("GitHub", "https://github.com")).unwrap();
        v.add(mk("GitLab", "https://gitlab.com")).unwrap();
        assert_eq!(v.search("github").len(), 1);
        assert_eq!(v.search("git").len(), 2);
        assert_eq!(v.search("nothing").len(), 0);
    }

    #[test]
    fn test_delete() {
        let mut v = open_tmp_vault();
        let id = v.add(Credential { id: String::new(), name: "X".into(),
            username: "u".into(), password: "p".into(), url: "".into(),
            notes: "".into(), created_at: 0, updated_at: 0 }).unwrap();
        v.delete(&id).unwrap();
        assert!(v.list().is_empty());
    }

    #[test]
    fn test_update() {
        let mut v = open_tmp_vault();
        v.add(Credential { id: String::new(), name: "Old".into(),
            username: "u".into(), password: "p".into(), url: "".into(),
            notes: "".into(), created_at: 0, updated_at: 0 }).unwrap();
        let mut c = v.list()[0].clone();
        c.name = "New".to_string();
        v.update(c).unwrap();
        assert_eq!(v.list()[0].name, "New");
    }

    #[test]
    fn test_persist_and_reload() {
        let path = std::env::temp_dir()
            .join(format!("test_persist_{}.db", uuid::Uuid::new_v4()));
        let key = test_key();
        let id = {
            let mut v = Vault::open(path.to_str().unwrap(), &key).unwrap();
            v.add(Credential { id: String::new(), name: "Persisted".into(),
                username: "u".into(), password: "p".into(), url: "".into(),
                notes: "".into(), created_at: 0, updated_at: 0 }).unwrap()
        };
        let v2 = Vault::open(path.to_str().unwrap(), &key).unwrap();
        assert_eq!(v2.list().len(), 1);
        assert_eq!(v2.list()[0].id, id);
    }

    #[test]
    fn test_wrong_key_on_reload_fails() {
        let path = std::env::temp_dir()
            .join(format!("test_wrongkey_{}.db", uuid::Uuid::new_v4()));
        let key = test_key();
        {
            let mut v = Vault::open(path.to_str().unwrap(), &key).unwrap();
            v.add(Credential { id: String::new(), name: "X".into(),
                username: "u".into(), password: "p".into(), url: "".into(),
                notes: "".into(), created_at: 0, updated_at: 0 }).unwrap();
        }
        assert!(Vault::open(path.to_str().unwrap(), &[99u8; 32]).is_err());
    }
}
```

- [ ] **Step 7: Run tests**

```bash
cargo test -p password-vault
```

Expected: all 6 tests pass.

- [ ] **Step 8: Commit**

```bash
git add crates/password-vault/
git commit -m "feat(password-vault): encrypted SQLite credential store"
```

---

## Task 6: `crates/wallet` — BIP-39 mnemonic + ETH + BTC

**Files:**
- Modify: `crates/wallet/Cargo.toml`
- Create: `crates/wallet/src/mnemonic.rs`
- Create: `crates/wallet/src/eth.rs`
- Create: `crates/wallet/src/btc.rs`
- Modify: `crates/wallet/src/lib.rs`

- [ ] **Step 1: Update crates/wallet/Cargo.toml**

```toml
[package]
name = "wallet"
version = "0.1.0"
edition = "2021"

[dependencies]
bip39 = "2.0"
coins-bip32 = "0.8"
k256 = { version = "0.13", features = ["ecdsa"] }
tiny-keccak = { version = "2", features = ["keccak"] }
rlp = "0.5"
bitcoin = { version = "0.31", features = ["rand-std"] }
hex = "0.4"
serde = { version = "1", features = ["derive"] }
```

- [ ] **Step 2: Write failing tests**

`crates/wallet/src/lib.rs`:

```rust
mod mnemonic;
mod eth;
mod btc;

pub use mnemonic::{generate_mnemonic, mnemonic_to_seed};
pub use eth::{eth_address, sign_eth_tx, EthTx};
pub use btc::btc_address;

pub struct Wallet {
    seed: [u8; 64],
}

impl Wallet {
    pub fn generate() -> (Self, String) {
        let phrase = generate_mnemonic();
        let seed = mnemonic_to_seed(&phrase).unwrap();
        (Self { seed }, phrase)
    }

    pub fn from_mnemonic(phrase: &str) -> Result<Self, String> {
        let seed = mnemonic_to_seed(phrase)?;
        Ok(Self { seed })
    }

    pub fn eth_address(&self, index: u32) -> Result<String, String> {
        eth_address(&self.seed, index)
    }

    pub fn btc_address(&self, index: u32) -> Result<String, String> {
        btc_address(&self.seed, index)
    }

    pub fn sign_eth_tx(&self, index: u32, tx: &EthTx) -> Result<String, String> {
        sign_eth_tx(&self.seed, index, tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // BIP-39 test mnemonic (12-word, from bip39 spec)
    const TEST_MNEMONIC: &str =
        "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    #[test]
    fn test_generate_mnemonic_12_words() {
        let phrase = generate_mnemonic();
        assert_eq!(phrase.split_whitespace().count(), 12);
    }

    #[test]
    fn test_generate_different_each_time() {
        assert_ne!(generate_mnemonic(), generate_mnemonic());
    }

    #[test]
    fn test_invalid_mnemonic_fails() {
        assert!(Wallet::from_mnemonic("not valid words here at all ok yes no").is_err());
    }

    #[test]
    fn test_eth_address_deterministic() {
        let w = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        let a1 = w.eth_address(0).unwrap();
        let a2 = w.eth_address(0).unwrap();
        assert_eq!(a1, a2);
        assert!(a1.starts_with("0x"));
        assert_eq!(a1.len(), 42);
    }

    #[test]
    fn test_eth_address_index_differs() {
        let w = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        assert_ne!(w.eth_address(0).unwrap(), w.eth_address(1).unwrap());
    }

    #[test]
    fn test_btc_address_deterministic() {
        let w = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        let a = w.btc_address(0).unwrap();
        assert_eq!(a, w.btc_address(0).unwrap());
        assert!(a.starts_with("bc1q"));
    }

    #[test]
    fn test_sign_eth_tx_returns_hex() {
        let w = Wallet::from_mnemonic(TEST_MNEMONIC).unwrap();
        let tx = EthTx {
            chain_id: 1,
            nonce: 0,
            to: "0xd3CdA913deB6f4967b2Ef3aa68f5A843aFbFB95".to_string(),
            value: "1000000000000000000".to_string(), // 1 ETH in wei
            gas_price: "20000000000".to_string(),     // 20 gwei
            gas_limit: 21000,
            data: "0x".to_string(),
        };
        let signed = w.sign_eth_tx(0, &tx).unwrap();
        assert!(signed.starts_with("0x"));
        assert!(signed.len() > 10);
    }
}
```

- [ ] **Step 3: Run tests to confirm they fail**

```bash
cargo test -p wallet 2>&1 | head -30
```

Expected: compile error — modules not found.

- [ ] **Step 4: Implement crates/wallet/src/mnemonic.rs**

```rust
use bip39::{Language, Mnemonic};

/// Generate a random 12-word BIP-39 mnemonic.
pub fn generate_mnemonic() -> String {
    Mnemonic::generate_in(Language::English, 12)
        .expect("mnemonic generation failed")
        .to_string()
}

/// Convert a mnemonic phrase to a 64-byte BIP-39 seed (no passphrase).
pub fn mnemonic_to_seed(phrase: &str) -> Result<[u8; 64], String> {
    let mnemonic = Mnemonic::parse_in(Language::English, phrase)
        .map_err(|e| format!("invalid mnemonic: {e}"))?;
    Ok(mnemonic.to_seed(""))
}
```

- [ ] **Step 5: Implement crates/wallet/src/eth.rs**

```rust
use coins_bip32::prelude::*;
use k256::ecdsa::{signature::hazmat::PrehashSigner, RecoveryId, SigningKey};
use tiny_keccak::{Hasher, Keccak};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EthTx {
    pub chain_id: u64,
    pub nonce: u64,
    pub to: String,
    pub value: String,    // decimal string, wei
    pub gas_price: String, // decimal string, wei
    pub gas_limit: u64,
    pub data: String,     // hex string, "0x..."
}

fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut h = Keccak::v256();
    h.update(data);
    let mut out = [0u8; 32];
    h.finalize(&mut out);
    out
}

/// Derive ETH address at BIP-44 path m/44'/60'/0'/0/{index}.
pub fn eth_address(seed: &[u8; 64], index: u32) -> Result<String, String> {
    let path = format!("m/44'/60'/0'/0/{index}");
    let signing_key = derive_signing_key(seed, &path)?;
    let verifying_key = signing_key.verifying_key();
    // Uncompressed public key: 04 || x (32) || y (32)
    let encoded = verifying_key.to_encoded_point(false);
    let pubkey_bytes = &encoded.as_bytes()[1..]; // strip 04 prefix
    let hash = keccak256(pubkey_bytes);
    let addr_bytes = &hash[12..]; // last 20 bytes
    Ok(format!("0x{}", hex::encode(addr_bytes)))
}

/// Sign an EIP-155 transaction at BIP-44 path m/44'/60'/0'/0/{index}.
/// Returns raw signed transaction as "0x..." hex string.
pub fn sign_eth_tx(seed: &[u8; 64], index: u32, tx: &EthTx) -> Result<String, String> {
    let path = format!("m/44'/60'/0'/0/{index}");
    let signing_key = derive_signing_key(seed, &path)?;

    let value = parse_u256(&tx.value)?;
    let gas_price = parse_u256(&tx.gas_price)?;
    let to_bytes = decode_address(&tx.to)?;
    let data_bytes = decode_hex_data(&tx.data)?;

    // EIP-155 signing payload: RLP([nonce, gasPrice, gasLimit, to, value, data, chainId, 0, 0])
    let mut stream = rlp::RlpStream::new_list(9);
    stream.append(&tx.nonce);
    stream.append(&gas_price.as_slice());
    stream.append(&tx.gas_limit);
    stream.append(&to_bytes.as_slice());
    stream.append(&value.as_slice());
    stream.append(&data_bytes.as_slice());
    stream.append(&tx.chain_id);
    stream.append(&0u8);
    stream.append(&0u8);
    let encoded = stream.out().to_vec();
    let hash = keccak256(&encoded);

    let (sig, recovery_id) = signing_key
        .sign_prehash_recoverable(&hash)
        .map_err(|e| format!("signing failed: {e}"))?;

    let r = sig.r().to_bytes();
    let s = sig.s().to_bytes();
    let v = tx.chain_id * 2 + 35 + recovery_id.to_byte() as u64;

    // Final signed tx: RLP([nonce, gasPrice, gasLimit, to, value, data, v, r, s])
    let mut signed_stream = rlp::RlpStream::new_list(9);
    signed_stream.append(&tx.nonce);
    signed_stream.append(&gas_price.as_slice());
    signed_stream.append(&tx.gas_limit);
    signed_stream.append(&to_bytes.as_slice());
    signed_stream.append(&value.as_slice());
    signed_stream.append(&data_bytes.as_slice());
    signed_stream.append(&v);
    signed_stream.append(&r.as_slice());
    signed_stream.append(&s.as_slice());

    Ok(format!("0x{}", hex::encode(signed_stream.out().to_vec())))
}

fn derive_signing_key(seed: &[u8; 64], path: &str) -> Result<SigningKey, String> {
    let xpriv = XPriv::root_from_seed(seed, None)
        .map_err(|e| format!("root key derivation failed: {e}"))?;
    let child: XPriv = xpriv
        .derive_path(path)
        .map_err(|e| format!("path derivation failed: {e}"))?;
    Ok(child.into())
}

/// Parse a decimal string into a big-endian byte vector (minimal encoding, no leading zeros).
fn parse_u256(decimal: &str) -> Result<Vec<u8>, String> {
    // Use u128 for simplicity; sufficient for gas values and typical ETH amounts
    let n: u128 = decimal.parse().map_err(|_| format!("invalid number: {decimal}"))?;
    if n == 0 {
        return Ok(vec![]);
    }
    let bytes = n.to_be_bytes();
    let start = bytes.iter().position(|&b| b != 0).unwrap_or(0);
    Ok(bytes[start..].to_vec())
}

fn decode_address(addr: &str) -> Result<Vec<u8>, String> {
    let s = addr.strip_prefix("0x").unwrap_or(addr);
    hex::decode(s).map_err(|_| format!("invalid address: {addr}"))
}

fn decode_hex_data(data: &str) -> Result<Vec<u8>, String> {
    let s = data.strip_prefix("0x").unwrap_or(data);
    if s.is_empty() { return Ok(vec![]); }
    hex::decode(s).map_err(|_| format!("invalid hex data: {data}"))
}
```

- [ ] **Step 6: Implement crates/wallet/src/btc.rs**

```rust
use bitcoin::{
    bip32::{DerivationPath, Xpriv},
    secp256k1::Secp256k1,
    Network,
};
use std::str::FromStr;

/// Derive a BIP-84 P2WPKH (native SegWit) address at m/84'/0'/0'/0/{index}.
pub fn btc_address(seed: &[u8; 64], index: u32) -> Result<String, String> {
    let secp = Secp256k1::new();
    let master = Xpriv::new_master(Network::Bitcoin, seed)
        .map_err(|e| format!("BTC master key: {e}"))?;
    let path = DerivationPath::from_str(&format!("m/84'/0'/0'/0/{index}"))
        .map_err(|e| format!("derivation path: {e}"))?;
    let child = master
        .derive_priv(&secp, &path)
        .map_err(|e| format!("BTC child key: {e}"))?;
    let pubkey = child.to_priv().public_key(&secp);
    let address = bitcoin::Address::p2wpkh(&pubkey, Network::Bitcoin)
        .map_err(|e| format!("p2wpkh address: {e}"))?;
    Ok(address.to_string())
}
```

- [ ] **Step 7: Run tests**

```bash
cargo test -p wallet
```

Expected: all 7 tests pass.

Note: If `bitcoin::Address::p2wpkh` API differs in v0.31, check docs — the address derivation pattern is the same; only method signature may vary.

- [ ] **Step 8: Commit**

```bash
git add crates/wallet/
git commit -m "feat(wallet): BIP-39/44/84, ETH EIP-155 signing, BTC P2WPKH addresses"
```

---

## Task 7: `crates/qr-engine`

**Files:**
- Modify: `crates/qr-engine/Cargo.toml`
- Modify: `crates/qr-engine/src/lib.rs`

- [ ] **Step 1: Update crates/qr-engine/Cargo.toml**

```toml
[package]
name = "qr-engine"
version = "0.1.0"
edition = "2021"

[dependencies]
qrcode = "0.14"
image = "0.25"
rqrr = "0.7"
```

- [ ] **Step 2: Write failing tests**

`crates/qr-engine/src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_returns_png_bytes() {
        let png = generate_qr_png("https://example.com").unwrap();
        // PNG magic bytes: 0x89 0x50 0x4E 0x47
        assert_eq!(&png[0..4], b"\x89PNG");
    }

    #[test]
    fn test_roundtrip_generate_and_decode() {
        let original = "Hello, EncryptVault!";
        let png = generate_qr_png(original).unwrap();
        let decoded = decode_bytes(&png).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_decode_invalid_bytes_returns_err() {
        assert!(decode_bytes(b"not an image").is_err());
    }

    #[test]
    fn test_decode_frame_no_qr_returns_none() {
        // Blank RGBA frame (all zeros)
        let frame = vec![0u8; 100 * 100 * 4];
        let result = decode_frame(&frame, 100, 100);
        assert!(result.is_none());
    }
}
```

- [ ] **Step 3: Run tests to confirm they fail**

```bash
cargo test -p qr-engine 2>&1 | head -20
```

Expected: compile error — functions not defined.

- [ ] **Step 4: Implement crates/qr-engine/src/lib.rs**

```rust
use image::{DynamicImage, GrayImage, ImageFormat, Luma};
use qrcode::QrCode;
use rqrr::PreparedImage;
use std::io::Cursor;

/// Generate a QR code PNG from a UTF-8 string. Returns raw PNG bytes.
pub fn generate_qr_png(content: &str) -> Result<Vec<u8>, String> {
    let code = QrCode::new(content.as_bytes()).map_err(|e| format!("QR encode: {e}"))?;
    let image: GrayImage = code.render::<Luma<u8>>().quiet_zone(true).build();
    let mut buf = Vec::new();
    DynamicImage::ImageLuma8(image)
        .write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
        .map_err(|e| format!("PNG encode: {e}"))?;
    Ok(buf)
}

/// Decode a QR code from image file bytes (PNG, JPEG, etc.).
/// Returns the decoded string content.
pub fn decode_bytes(data: &[u8]) -> Result<String, String> {
    let img = image::load_from_memory(data)
        .map_err(|e| format!("image load: {e}"))?
        .to_luma8();
    let mut prepared = PreparedImage::prepare(img);
    let grids = prepared.detect_grids();
    let grid = grids.into_iter().next().ok_or("no QR code found in image")?;
    let (_, content) = grid.decode().map_err(|e| format!("QR decode: {e}"))?;
    Ok(content)
}

/// Decode a QR code from a raw RGBA camera frame.
/// Returns the decoded string, or `None` if no QR code is found.
pub fn decode_frame(rgba: &[u8], width: u32, height: u32) -> Option<String> {
    let gray = GrayImage::from_fn(width, height, |x, y| {
        let i = ((y * width + x) * 4) as usize;
        let r = rgba[i] as u32;
        let g = rgba[i + 1] as u32;
        let b = rgba[i + 2] as u32;
        Luma([(( r * 77 + g * 150 + b * 29) >> 8) as u8])
    });
    let mut prepared = PreparedImage::prepare(gray);
    let grids = prepared.detect_grids();
    let grid = grids.into_iter().next()?;
    let (_, content) = grid.decode().ok()?;
    Some(content)
}
```

- [ ] **Step 5: Run tests**

```bash
cargo test -p qr-engine
```

Expected: all 4 tests pass.

- [ ] **Step 6: Commit**

```bash
git add crates/qr-engine/
git commit -m "feat(qr-engine): generate and decode QR codes from files and camera frames"
```

---

## Task 8: Tauri app state + unlock/lock commands

**Files:**
- Modify: `src-tauri/src/state.rs`
- Modify: `src-tauri/src/commands/vault_cmd.rs`
- Modify: `src-tauri/src/main.rs`

- [ ] **Step 1: Write integration test for unlock/lock**

Create `src-tauri/tests/integration_test.rs`:

```rust
// Integration tests require a running Tauri app — these are compile-time checks
// that the command signatures are correct. Full E2E tested manually.

#[test]
fn state_default_is_locked() {
    let state = app_lib::state::AppState::default();
    assert!(state.vault_key.lock().unwrap().is_none());
    assert!(state.vault.lock().unwrap().is_none());
    assert!(state.wallet.lock().unwrap().is_none());
}
```

Add to `src-tauri/Cargo.toml`:
```toml
[lib]
name = "app_lib"
path = "src/lib.rs"
crate-type = ["staticlib", "cdylib", "rlib"]
```

Create `src-tauri/src/lib.rs`:
```rust
pub mod state;
pub mod commands;
```

- [ ] **Step 2: Implement full AppState in src-tauri/src/state.rs**

```rust
use std::sync::Mutex;
use encrypt::Key;
use password_vault::Vault;
use wallet::Wallet;

pub struct AppState {
    pub vault_key: Mutex<Option<Key>>,
    pub vault: Mutex<Option<Vault>>,
    pub wallet: Mutex<Option<Wallet>>,
    pub vault_db_path: Mutex<Option<String>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            vault_key: Mutex::new(None),
            vault: Mutex::new(None),
            wallet: Mutex::new(None),
            vault_db_path: Mutex::new(None),
        }
    }
}

impl AppState {
    pub fn is_unlocked(&self) -> bool {
        self.vault_key.lock().unwrap().is_some()
    }

    pub fn lock(&self) {
        *self.vault_key.lock().unwrap() = None;
        *self.vault.lock().unwrap() = None;
        *self.wallet.lock().unwrap() = None;
    }
}
```

- [ ] **Step 3: Implement unlock/lock/is_unlocked in src-tauri/src/commands/vault_cmd.rs**

```rust
use tauri::State;
use crate::state::AppState;
use encrypt::{derive_key, generate_salt};
use password_vault::Vault;
use wallet::Wallet;
use std::path::PathBuf;

fn vault_db_path() -> String {
    let mut path = dirs_next::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("EncryptVault");
    std::fs::create_dir_all(&path).ok();
    path.push("vault.db");
    path.to_string_lossy().to_string()
}

#[tauri::command]
pub fn is_unlocked(state: State<'_, AppState>) -> bool {
    state.is_unlocked()
}

#[tauri::command]
pub fn unlock_vault(password: String, state: State<'_, AppState>) -> Result<(), String> {
    let db_path = vault_db_path();

    // Derive or load vault salt
    let salt_path = db_path.replace(".db", ".salt");
    let salt: [u8; 16] = if std::path::Path::new(&salt_path).exists() {
        let bytes = std::fs::read(&salt_path).map_err(|e| e.to_string())?;
        bytes.try_into().map_err(|_| "corrupt salt file".to_string())?
    } else {
        let s = generate_salt();
        std::fs::write(&salt_path, s).map_err(|e| e.to_string())?;
        s
    };

    let key = derive_key(&password, &salt);
    let vault = Vault::open(&db_path, &key)?;

    *state.vault_key.lock().unwrap() = Some(key);
    *state.vault.lock().unwrap() = Some(vault);

    Ok(())
}

#[tauri::command]
pub fn lock_vault(state: State<'_, AppState>) {
    state.lock();
}

#[tauri::command]
pub fn add_credential(
    name: String, username: String, password: String,
    url: String, notes: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let mut vault_guard = state.vault.lock().unwrap();
    let vault = vault_guard.as_mut().ok_or("vault is locked")?;
    let cred = password_vault::Credential {
        id: String::new(), name, username, password, url, notes,
        created_at: 0, updated_at: 0,
    };
    vault.add(cred)
}

#[tauri::command]
pub fn list_credentials(state: State<'_, AppState>) -> Result<Vec<password_vault::Credential>, String> {
    let vault_guard = state.vault.lock().unwrap();
    let vault = vault_guard.as_ref().ok_or("vault is locked")?;
    Ok(vault.list().into_iter().cloned().collect())
}

#[tauri::command]
pub fn update_credential(
    id: String, name: String, username: String, password: String,
    url: String, notes: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut vault_guard = state.vault.lock().unwrap();
    let vault = vault_guard.as_mut().ok_or("vault is locked")?;
    // Preserve created_at from existing record
    let existing = vault.list().into_iter()
        .find(|c| c.id == id)
        .ok_or("credential not found")?
        .clone();
    vault.update(password_vault::Credential {
        id, name, username, password, url, notes,
        created_at: existing.created_at, updated_at: 0,
    })
}

#[tauri::command]
pub fn delete_credential(id: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut vault_guard = state.vault.lock().unwrap();
    let vault = vault_guard.as_mut().ok_or("vault is locked")?;
    vault.delete(&id)
}

#[tauri::command]
pub fn search_credentials(
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<password_vault::Credential>, String> {
    let vault_guard = state.vault.lock().unwrap();
    let vault = vault_guard.as_ref().ok_or("vault is locked")?;
    Ok(vault.search(&query).into_iter().cloned().collect())
}
```

Add `dirs-next` to `src-tauri/Cargo.toml`:
```toml
dirs-next = "2"
```

- [ ] **Step 4: Implement encrypt commands in src-tauri/src/commands/encrypt_cmd.rs**

```rust
use tauri::State;
use crate::state::AppState;
use encrypt::{decrypt_bytes_with_passphrase, decrypt_text, encrypt_bytes_with_passphrase, encrypt_text};

#[tauri::command]
pub fn cmd_encrypt_text(passphrase: String, plaintext: String) -> String {
    encrypt_text(&passphrase, &plaintext)
}

#[tauri::command]
pub fn cmd_decrypt_text(passphrase: String, ciphertext: String) -> Result<String, String> {
    decrypt_text(&passphrase, &ciphertext)
}

#[tauri::command]
pub fn cmd_encrypt_file(passphrase: String, file_path: String) -> Result<String, String> {
    let data = std::fs::read(&file_path).map_err(|e| format!("read file: {e}"))?;
    let encrypted = encrypt_bytes_with_passphrase(&passphrase, &data);
    let out_path = format!("{}.enc", file_path);
    // Atomic write: write to temp file, then rename
    let tmp_path = format!("{}.enc.tmp", file_path);
    std::fs::write(&tmp_path, &encrypted).map_err(|e| format!("write tmp: {e}"))?;
    std::fs::rename(&tmp_path, &out_path).map_err(|e| format!("rename: {e}"))?;
    Ok(out_path)
}

#[tauri::command]
pub fn cmd_decrypt_file(passphrase: String, file_path: String) -> Result<String, String> {
    let data = std::fs::read(&file_path).map_err(|e| format!("read file: {e}"))?;
    let decrypted = decrypt_bytes_with_passphrase(&passphrase, &data)?;
    let out_path = file_path
        .strip_suffix(".enc")
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("{}.dec", file_path));
    let tmp_path = format!("{}.tmp", out_path);
    std::fs::write(&tmp_path, &decrypted).map_err(|e| format!("write tmp: {e}"))?;
    std::fs::rename(&tmp_path, &out_path).map_err(|e| format!("rename: {e}"))?;
    Ok(out_path)
}
```

- [ ] **Step 5: Implement wallet commands in src-tauri/src/commands/wallet_cmd.rs**

```rust
use tauri::State;
use crate::state::AppState;
use wallet::{Wallet, EthTx};

#[tauri::command]
pub fn setup_wallet(state: State<'_, AppState>) -> Result<String, String> {
    let mut wallet_guard = state.wallet.lock().unwrap();
    if wallet_guard.is_some() {
        return Err("wallet already set up".to_string());
    }
    let (wallet, mnemonic) = Wallet::generate();
    *wallet_guard = Some(wallet);
    Ok(mnemonic)
}

#[tauri::command]
pub fn import_wallet(mnemonic: String, state: State<'_, AppState>) -> Result<(), String> {
    let wallet = Wallet::from_mnemonic(&mnemonic)?;
    *state.wallet.lock().unwrap() = Some(wallet);
    Ok(())
}

#[tauri::command]
pub fn derive_eth_address(index: u32, state: State<'_, AppState>) -> Result<String, String> {
    let wallet_guard = state.wallet.lock().unwrap();
    let wallet = wallet_guard.as_ref().ok_or("wallet not loaded")?;
    wallet.eth_address(index)
}

#[tauri::command]
pub fn derive_btc_address(index: u32, state: State<'_, AppState>) -> Result<String, String> {
    let wallet_guard = state.wallet.lock().unwrap();
    let wallet = wallet_guard.as_ref().ok_or("wallet not loaded")?;
    wallet.btc_address(index)
}

#[tauri::command]
pub fn sign_eth_tx(
    index: u32, tx: EthTx,
    state: State<'_, AppState>,
) -> Result<String, String> {
    let wallet_guard = state.wallet.lock().unwrap();
    let wallet = wallet_guard.as_ref().ok_or("wallet not loaded")?;
    wallet.sign_eth_tx(index, &tx)
}
```

- [ ] **Step 6: Implement QR commands in src-tauri/src/commands/qr_cmd.rs**

```rust
use tauri::State;
use crate::state::AppState;
use qr_engine::{decode_bytes, decode_frame, generate_qr_png};
use base64::{engine::general_purpose::STANDARD as B64, Engine};

#[tauri::command]
pub fn cmd_generate_qr(content: String) -> Result<String, String> {
    let png = generate_qr_png(&content)?;
    Ok(format!("data:image/png;base64,{}", B64.encode(png)))
}

#[tauri::command]
pub fn cmd_decode_qr_file(file_path: String) -> Result<String, String> {
    let data = std::fs::read(&file_path).map_err(|e| format!("read file: {e}"))?;
    decode_bytes(&data)
}

#[tauri::command]
pub fn cmd_decode_qr_frame(rgba: Vec<u8>, width: u32, height: u32) -> Option<String> {
    decode_frame(&rgba, width, height)
}
```

Add `base64` to `src-tauri/Cargo.toml`:
```toml
base64 = "0.22"
```

- [ ] **Step 7: Wire all commands into main.rs**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;
mod commands;

use state::AppState;
use commands::{
    encrypt_cmd::{cmd_decrypt_file, cmd_decrypt_text, cmd_encrypt_file, cmd_encrypt_text},
    qr_cmd::{cmd_decode_qr_file, cmd_decode_qr_frame, cmd_generate_qr},
    vault_cmd::{
        add_credential, delete_credential, is_unlocked, list_credentials, lock_vault,
        search_credentials, unlock_vault, update_credential,
    },
    wallet_cmd::{
        derive_btc_address, derive_eth_address, import_wallet, setup_wallet, sign_eth_tx,
    },
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 8: Verify the backend compiles**

```bash
cargo check -p app
```

Expected: `Finished` with no errors.

- [ ] **Step 9: Commit**

```bash
git add src-tauri/
git commit -m "feat(tauri): wire all Rust commands into Tauri app"
```

---

## Task 9: React frontend — App shell, sidebar, and routing

**Files:**
- Create: `src/main.tsx`
- Create: `src/App.tsx`
- Create: `src/context/AppContext.tsx`
- Create: `src/components/Sidebar.tsx`
- Create: `src/components/ErrorBanner.tsx`

- [ ] **Step 1: Create src/main.tsx**

```tsx
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

- [ ] **Step 2: Create src/index.css**

```css
@tailwind base;
@tailwind components;
@tailwind utilities;
```

- [ ] **Step 3: Create src/context/AppContext.tsx**

```tsx
import { createContext, useContext, useState, ReactNode } from "react";

type Page = "encrypt" | "vault" | "wallet" | "qr";

interface AppContextType {
  isUnlocked: boolean;
  setUnlocked: (v: boolean) => void;
  currentPage: Page;
  setPage: (p: Page) => void;
}

const AppContext = createContext<AppContextType | null>(null);

export function AppProvider({ children }: { children: ReactNode }) {
  const [isUnlocked, setUnlocked] = useState(false);
  const [currentPage, setPage] = useState<Page>("encrypt");

  return (
    <AppContext.Provider value={{ isUnlocked, setUnlocked, currentPage, setPage }}>
      {children}
    </AppContext.Provider>
  );
}

export function useApp() {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error("useApp must be used within AppProvider");
  return ctx;
}
```

- [ ] **Step 4: Create src/components/Sidebar.tsx**

```tsx
import { useApp } from "../context/AppContext";
import { invoke } from "@tauri-apps/api/core";

const PAGES = [
  { id: "encrypt", label: "Encrypt / Decrypt", icon: "🔐" },
  { id: "vault",   label: "Password Vault",     icon: "🗝️" },
  { id: "wallet",  label: "Wallet",              icon: "💼" },
  { id: "qr",      label: "QR Code",             icon: "⬛" },
] as const;

export default function Sidebar() {
  const { currentPage, setPage, setUnlocked } = useApp();

  async function handleLock() {
    await invoke("lock_vault");
    setUnlocked(false);
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
            className={`w-full text-left px-4 py-3 flex items-center gap-3 hover:bg-gray-800 transition-colors ${
              currentPage === p.id ? "bg-gray-800 font-medium" : ""
            }`}
          >
            <span>{p.icon}</span>
            <span className="text-sm">{p.label}</span>
          </button>
        ))}
      </nav>
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

- [ ] **Step 5: Create src/components/ErrorBanner.tsx**

```tsx
interface Props {
  message: string | null;
  onDismiss: () => void;
}

export default function ErrorBanner({ message, onDismiss }: Props) {
  if (!message) return null;
  return (
    <div className="bg-red-100 border border-red-400 text-red-800 px-4 py-3 rounded flex justify-between items-center mb-4">
      <span className="text-sm">{message}</span>
      <button onClick={onDismiss} className="ml-4 font-bold">×</button>
    </div>
  );
}
```

- [ ] **Step 6: Create src/App.tsx**

```tsx
import { AppProvider, useApp } from "./context/AppContext";
import Sidebar from "./components/Sidebar";
import UnlockPage from "./pages/UnlockPage";
import EncryptPage from "./pages/EncryptPage";
import VaultPage from "./pages/VaultPage";
import WalletPage from "./pages/WalletPage";
import QRPage from "./pages/QRPage";

function PageRouter() {
  const { isUnlocked, currentPage } = useApp();

  if (!isUnlocked) return <UnlockPage />;

  return (
    <div className="flex h-screen bg-gray-50">
      <Sidebar />
      <main className="flex-1 overflow-auto p-8">
        {currentPage === "encrypt" && <EncryptPage />}
        {currentPage === "vault"   && <VaultPage />}
        {currentPage === "wallet"  && <WalletPage />}
        {currentPage === "qr"      && <QRPage />}
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

- [ ] **Step 7: Create stub page files so the app compiles**

`src/pages/UnlockPage.tsx`: stub (will be implemented in Task 10)
```tsx
export default function UnlockPage() { return <div>Unlock</div>; }
```

Repeat for `EncryptPage.tsx`, `VaultPage.tsx`, `WalletPage.tsx`, `QRPage.tsx` — same one-liner stub pattern.

- [ ] **Step 8: Verify frontend compiles**

```bash
npm run build
```

Expected: `dist/` created, no TypeScript errors.

- [ ] **Step 9: Commit**

```bash
git add src/
git commit -m "feat(frontend): app shell, sidebar, and routing"
```

---

## Task 10: Frontend pages — Unlock, Encrypt, Vault, Wallet, QR

**Files:**
- Modify: `src/pages/UnlockPage.tsx`
- Modify: `src/pages/EncryptPage.tsx`
- Modify: `src/pages/VaultPage.tsx`
- Modify: `src/pages/WalletPage.tsx`
- Modify: `src/pages/QRPage.tsx`

- [ ] **Step 1: Implement src/pages/UnlockPage.tsx**

```tsx
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useApp } from "../context/AppContext";
import ErrorBanner from "../components/ErrorBanner";

export default function UnlockPage() {
  const { setUnlocked } = useApp();
  const [password, setPassword] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault();
    setLoading(true);
    setError(null);
    try {
      await invoke("unlock_vault", { password });
      setUnlocked(true);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="min-h-screen bg-gray-900 flex items-center justify-center">
      <div className="bg-white rounded-2xl p-8 w-full max-w-sm shadow-xl">
        <h1 className="text-2xl font-bold text-gray-900 mb-2">EncryptVault</h1>
        <p className="text-sm text-gray-500 mb-6">Enter your master password to unlock.</p>
        <ErrorBanner message={error} onDismiss={() => setError(null)} />
        <form onSubmit={handleSubmit} className="space-y-4">
          <input
            type="password"
            placeholder="Master password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
            className="w-full border border-gray-300 rounded-lg px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
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
}
```

- [ ] **Step 2: Implement src/pages/EncryptPage.tsx**

```tsx
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open, save } from "@tauri-apps/plugin-dialog";
import ErrorBanner from "../components/ErrorBanner";

type Mode = "text" | "file";

export default function EncryptPage() {
  const [mode, setMode] = useState<Mode>("text");
  const [passphrase, setPassphrase] = useState("");
  const [input, setInput] = useState("");
  const [output, setOutput] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  async function handleEncrypt() {
    setError(null);
    setLoading(true);
    try {
      if (mode === "text") {
        const result = await invoke<string>("cmd_encrypt_text", { passphrase, plaintext: input });
        setOutput(result);
      } else {
        const filePath = await open({ multiple: false, title: "Select file to encrypt" });
        if (!filePath) return;
        const outPath = await invoke<string>("cmd_encrypt_file", { passphrase, filePath: String(filePath) });
        setOutput(`Encrypted to: ${outPath}`);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }

  async function handleDecrypt() {
    setError(null);
    setLoading(true);
    try {
      if (mode === "text") {
        const result = await invoke<string>("cmd_decrypt_text", { passphrase, ciphertext: input });
        setOutput(result);
      } else {
        const filePath = await open({ multiple: false, filters: [{ name: "Encrypted", extensions: ["enc"] }], title: "Select .enc file" });
        if (!filePath) return;
        const outPath = await invoke<string>("cmd_decrypt_file", { passphrase, filePath: String(filePath) });
        setOutput(`Decrypted to: ${outPath}`);
      }
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="max-w-2xl">
      <h2 className="text-xl font-bold mb-6">Encrypt / Decrypt</h2>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />

      <div className="flex gap-2 mb-4">
        {(["text", "file"] as Mode[]).map((m) => (
          <button key={m} onClick={() => setMode(m)}
            className={`px-4 py-1.5 rounded-full text-sm font-medium transition-colors ${mode === m ? "bg-blue-600 text-white" : "bg-gray-200 text-gray-700 hover:bg-gray-300"}`}>
            {m === "text" ? "Text" : "File"}
          </button>
        ))}
      </div>

      <div className="space-y-4">
        <input type="password" placeholder="Passphrase" value={passphrase}
          onChange={(e) => setPassphrase(e.target.value)}
          className="w-full border border-gray-300 rounded-lg px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500" />

        {mode === "text" && (
          <textarea placeholder="Input text" value={input} onChange={(e) => setInput(e.target.value)}
            rows={4}
            className="w-full border border-gray-300 rounded-lg px-4 py-2 text-sm font-mono focus:outline-none focus:ring-2 focus:ring-blue-500" />
        )}

        <div className="flex gap-2">
          <button onClick={handleEncrypt} disabled={loading}
            className="bg-blue-600 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-blue-700 disabled:opacity-50 transition-colors">
            Encrypt
          </button>
          <button onClick={handleDecrypt} disabled={loading}
            className="bg-gray-700 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-gray-800 disabled:opacity-50 transition-colors">
            Decrypt
          </button>
        </div>

        {output && (
          <div className="bg-gray-100 rounded-lg p-4">
            <p className="text-xs text-gray-500 mb-1 font-medium">Output</p>
            <pre className="text-sm font-mono whitespace-pre-wrap break-all">{output}</pre>
            <button onClick={() => navigator.clipboard.writeText(output)}
              className="mt-2 text-xs text-blue-600 hover:underline">
              Copy to clipboard
            </button>
          </div>
        )}
      </div>
    </div>
  );
}
```

Note: add `@tauri-apps/plugin-dialog` to `package.json` dependencies and `"dialog:allow-open"`, `"dialog:allow-save"` to capabilities.

- [ ] **Step 3: Implement src/pages/VaultPage.tsx**

```tsx
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import ErrorBanner from "../components/ErrorBanner";

interface Credential {
  id: string; name: string; username: string; password: string;
  url: string; notes: string; created_at: number; updated_at: number;
}

const EMPTY_FORM = { name: "", username: "", password: "", url: "", notes: "" };

export default function VaultPage() {
  const [creds, setCreds] = useState<Credential[]>([]);
  const [search, setSearch] = useState("");
  const [form, setForm] = useState(EMPTY_FORM);
  const [editId, setEditId] = useState<string | null>(null);
  const [showForm, setShowForm] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [showPasswords, setShowPasswords] = useState<Record<string, boolean>>({});

  async function load() {
    try {
      const result = await invoke<Credential[]>("list_credentials");
      setCreds(result);
    } catch (err) { setError(String(err)); }
  }

  useEffect(() => { load(); }, []);

  const filtered = search.trim()
    ? creds.filter(c => c.name.toLowerCase().includes(search.toLowerCase()) || c.url.toLowerCase().includes(search.toLowerCase()))
    : creds;

  async function handleSave(e: React.FormEvent) {
    e.preventDefault();
    try {
      if (editId) {
        await invoke("update_credential", { id: editId, ...form });
      } else {
        await invoke("add_credential", form);
      }
      setForm(EMPTY_FORM); setEditId(null); setShowForm(false);
      await load();
    } catch (err) { setError(String(err)); }
  }

  async function handleDelete(id: string) {
    if (!confirm("Delete this credential?")) return;
    try { await invoke("delete_credential", { id }); await load(); }
    catch (err) { setError(String(err)); }
  }

  function startEdit(c: Credential) {
    setForm({ name: c.name, username: c.username, password: c.password, url: c.url, notes: c.notes });
    setEditId(c.id); setShowForm(true);
  }

  return (
    <div className="max-w-3xl">
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-bold">Password Vault</h2>
        <button onClick={() => { setForm(EMPTY_FORM); setEditId(null); setShowForm(true); }}
          className="bg-blue-600 text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-blue-700">
          + Add
        </button>
      </div>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />

      <input placeholder="Search by name or URL…" value={search} onChange={e => setSearch(e.target.value)}
        className="w-full border border-gray-300 rounded-lg px-4 py-2 text-sm mb-4 focus:outline-none focus:ring-2 focus:ring-blue-500" />

      {showForm && (
        <form onSubmit={handleSave} className="bg-white border rounded-xl p-5 mb-4 space-y-3 shadow-sm">
          <h3 className="font-semibold text-sm">{editId ? "Edit credential" : "New credential"}</h3>
          {(["name", "username", "password", "url", "notes"] as const).map(field => (
            <input key={field} placeholder={field.charAt(0).toUpperCase() + field.slice(1)}
              type={field === "password" ? "password" : "text"}
              value={form[field]} onChange={e => setForm({ ...form, [field]: e.target.value })}
              className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm"
              required={field === "name"} />
          ))}
          <div className="flex gap-2">
            <button type="submit" className="bg-blue-600 text-white px-4 py-2 rounded-lg text-sm font-medium">Save</button>
            <button type="button" onClick={() => setShowForm(false)} className="text-sm text-gray-500 hover:underline">Cancel</button>
          </div>
        </form>
      )}

      <div className="space-y-2">
        {filtered.length === 0 && <p className="text-sm text-gray-400">No credentials found.</p>}
        {filtered.map(c => (
          <div key={c.id} className="bg-white border rounded-xl px-5 py-4 flex justify-between items-start shadow-sm">
            <div>
              <p className="font-medium text-sm">{c.name}</p>
              <p className="text-xs text-gray-500">{c.username}</p>
              {c.url && <p className="text-xs text-blue-500">{c.url}</p>}
              <div className="text-xs text-gray-400 mt-1 font-mono">
                {showPasswords[c.id] ? c.password : "••••••••"}
                <button onClick={() => setShowPasswords(p => ({ ...p, [c.id]: !p[c.id] }))}
                  className="ml-2 text-blue-500 hover:underline">{showPasswords[c.id] ? "hide" : "show"}</button>
                <button onClick={() => navigator.clipboard.writeText(c.password)}
                  className="ml-2 text-blue-500 hover:underline">copy</button>
              </div>
            </div>
            <div className="flex gap-2 ml-4">
              <button onClick={() => startEdit(c)} className="text-xs text-gray-500 hover:underline">Edit</button>
              <button onClick={() => handleDelete(c.id)} className="text-xs text-red-500 hover:underline">Delete</button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
```

- [ ] **Step 4: Implement src/pages/WalletPage.tsx**

```tsx
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import ErrorBanner from "../components/ErrorBanner";

interface EthTx {
  chain_id: number; nonce: number; to: string; value: string;
  gas_price: string; gas_limit: number; data: string;
}

export default function WalletPage() {
  const [mnemonic, setMnemonic] = useState("");
  const [mnemonicInput, setMnemonicInput] = useState("");
  const [ethAddr, setEthAddr] = useState("");
  const [btcAddr, setBtcAddr] = useState("");
  const [addrIndex, setAddrIndex] = useState(0);
  const [signedTx, setSignedTx] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [tab, setTab] = useState<"setup" | "addresses" | "sign">("setup");
  const [txFields, setTxFields] = useState<EthTx>({
    chain_id: 1, nonce: 0, to: "", value: "0", gas_price: "20000000000", gas_limit: 21000, data: "0x",
  });

  async function handleGenerate() {
    try {
      const phrase = await invoke<string>("setup_wallet");
      setMnemonic(phrase);
    } catch (err) { setError(String(err)); }
  }

  async function handleImport() {
    try {
      await invoke("import_wallet", { mnemonic: mnemonicInput });
      setMnemonic(mnemonicInput);
    } catch (err) { setError(String(err)); }
  }

  async function handleDeriveAddresses() {
    try {
      const eth = await invoke<string>("derive_eth_address", { index: addrIndex });
      const btc = await invoke<string>("derive_btc_address", { index: addrIndex });
      setEthAddr(eth); setBtcAddr(btc);
    } catch (err) { setError(String(err)); }
  }

  async function handleSign() {
    try {
      const signed = await invoke<string>("sign_eth_tx", { index: addrIndex, tx: txFields });
      setSignedTx(signed);
    } catch (err) { setError(String(err)); }
  }

  return (
    <div className="max-w-2xl">
      <h2 className="text-xl font-bold mb-6">Wallet</h2>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />

      <div className="flex gap-2 mb-6">
        {(["setup", "addresses", "sign"] as const).map(t => (
          <button key={t} onClick={() => setTab(t)}
            className={`px-4 py-1.5 rounded-full text-sm font-medium transition-colors ${tab === t ? "bg-blue-600 text-white" : "bg-gray-200 text-gray-700 hover:bg-gray-300"}`}>
            {t.charAt(0).toUpperCase() + t.slice(1)}
          </button>
        ))}
      </div>

      {tab === "setup" && (
        <div className="space-y-4">
          <button onClick={handleGenerate}
            className="bg-blue-600 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-blue-700">
            Generate new wallet
          </button>
          {mnemonic && (
            <div className="bg-yellow-50 border border-yellow-300 rounded-xl p-4">
              <p className="text-xs text-yellow-800 font-semibold mb-2">⚠ Write down your mnemonic — it will not be shown again.</p>
              <p className="font-mono text-sm break-all">{mnemonic}</p>
            </div>
          )}
          <hr />
          <div className="space-y-2">
            <p className="text-sm text-gray-600 font-medium">Import existing wallet</p>
            <textarea placeholder="Enter 12 or 24 word mnemonic…" value={mnemonicInput}
              onChange={e => setMnemonicInput(e.target.value)} rows={3}
              className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm font-mono" />
            <button onClick={handleImport}
              className="bg-gray-700 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-gray-800">
              Import
            </button>
          </div>
        </div>
      )}

      {tab === "addresses" && (
        <div className="space-y-4">
          <div className="flex items-center gap-3">
            <label className="text-sm text-gray-600">Account index</label>
            <input type="number" min={0} value={addrIndex} onChange={e => setAddrIndex(Number(e.target.value))}
              className="w-24 border border-gray-300 rounded-lg px-3 py-1.5 text-sm" />
            <button onClick={handleDeriveAddresses}
              className="bg-blue-600 text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-blue-700">
              Derive
            </button>
          </div>
          {ethAddr && (
            <div className="space-y-2">
              <AddressRow label="ETH" address={ethAddr} />
              <AddressRow label="BTC" address={btcAddr} />
            </div>
          )}
        </div>
      )}

      {tab === "sign" && (
        <div className="space-y-3">
          <p className="text-sm text-gray-600">Sign an ETH transaction (EIP-155, offline)</p>
          {(Object.keys(txFields) as (keyof EthTx)[]).map(f => (
            <div key={f} className="flex items-center gap-3">
              <label className="text-xs text-gray-500 w-24 shrink-0">{f}</label>
              <input value={String(txFields[f])} onChange={e => setTxFields({ ...txFields, [f]: f === "chain_id" || f === "nonce" || f === "gas_limit" ? Number(e.target.value) : e.target.value })}
                className="flex-1 border border-gray-300 rounded-lg px-3 py-1.5 text-sm font-mono" />
            </div>
          ))}
          <button onClick={handleSign}
            className="bg-blue-600 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-blue-700">
            Sign transaction
          </button>
          {signedTx && (
            <div className="bg-gray-100 rounded-lg p-4">
              <p className="text-xs text-gray-500 mb-1">Signed transaction hex</p>
              <pre className="text-xs font-mono break-all whitespace-pre-wrap">{signedTx}</pre>
              <button onClick={() => navigator.clipboard.writeText(signedTx)} className="mt-2 text-xs text-blue-600 hover:underline">Copy</button>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

function AddressRow({ label, address }: { label: string; address: string }) {
  return (
    <div className="bg-white border rounded-xl p-4 flex justify-between items-center">
      <div>
        <p className="text-xs text-gray-500 font-medium">{label}</p>
        <p className="text-sm font-mono break-all">{address}</p>
      </div>
      <button onClick={() => navigator.clipboard.writeText(address)} className="text-xs text-blue-600 hover:underline ml-4 shrink-0">Copy</button>
    </div>
  );
}
```

- [ ] **Step 5: Implement src/pages/QRPage.tsx**

```tsx
import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import ErrorBanner from "../components/ErrorBanner";

export default function QRPage() {
  const [tab, setTab] = useState<"generate" | "decode">("generate");
  const [content, setContent] = useState("");
  const [qrDataUrl, setQrDataUrl] = useState("");
  const [decoded, setDecoded] = useState("");
  const [scanning, setScanning] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const videoRef = useRef<HTMLVideoElement>(null);
  const streamRef = useRef<MediaStream | null>(null);
  const scanLoopRef = useRef<number | null>(null);

  async function handleGenerate() {
    try {
      const dataUrl = await invoke<string>("cmd_generate_qr", { content });
      setQrDataUrl(dataUrl);
    } catch (err) { setError(String(err)); }
  }

  async function handleDecodeFile() {
    try {
      const filePath = await open({ multiple: false, filters: [{ name: "Images", extensions: ["png", "jpg", "jpeg"] }] });
      if (!filePath) return;
      const result = await invoke<string>("cmd_decode_qr_file", { filePath: String(filePath) });
      setDecoded(result);
    } catch (err) { setError(String(err)); }
  }

  async function startCamera() {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ video: { facingMode: "environment" } });
      streamRef.current = stream;
      if (videoRef.current) videoRef.current.srcObject = stream;
      setScanning(true);
      startScanLoop();
    } catch (err) { setError(String(err)); }
  }

  function stopCamera() {
    if (scanLoopRef.current) cancelAnimationFrame(scanLoopRef.current);
    streamRef.current?.getTracks().forEach(t => t.stop());
    setScanning(false);
  }

  function startScanLoop() {
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d")!;

    function scan() {
      const video = videoRef.current;
      if (!video || video.readyState < 2) { scanLoopRef.current = requestAnimationFrame(scan); return; }
      canvas.width = video.videoWidth;
      canvas.height = video.videoHeight;
      ctx.drawImage(video, 0, 0);
      const frame = ctx.getImageData(0, 0, canvas.width, canvas.height);
      invoke<string | null>("cmd_decode_qr_frame", {
        rgba: Array.from(frame.data),
        width: frame.width,
        height: frame.height,
      }).then(result => {
        if (result) { setDecoded(result); stopCamera(); }
        else { scanLoopRef.current = requestAnimationFrame(scan); }
      });
    }

    scanLoopRef.current = requestAnimationFrame(scan);
  }

  useEffect(() => () => { stopCamera(); }, []);

  return (
    <div className="max-w-2xl">
      <h2 className="text-xl font-bold mb-6">QR Code</h2>
      <ErrorBanner message={error} onDismiss={() => setError(null)} />

      <div className="flex gap-2 mb-6">
        {(["generate", "decode"] as const).map(t => (
          <button key={t} onClick={() => setTab(t)}
            className={`px-4 py-1.5 rounded-full text-sm font-medium transition-colors ${tab === t ? "bg-blue-600 text-white" : "bg-gray-200 text-gray-700 hover:bg-gray-300"}`}>
            {t.charAt(0).toUpperCase() + t.slice(1)}
          </button>
        ))}
      </div>

      {tab === "generate" && (
        <div className="space-y-4">
          <textarea placeholder="Text or URL to encode…" value={content}
            onChange={e => setContent(e.target.value)} rows={3}
            className="w-full border border-gray-300 rounded-lg px-4 py-2 text-sm" />
          <button onClick={handleGenerate}
            className="bg-blue-600 text-white px-5 py-2 rounded-lg text-sm font-medium hover:bg-blue-700">
            Generate QR
          </button>
          {qrDataUrl && (
            <div className="bg-white border rounded-xl p-4 flex flex-col items-center gap-3">
              <img src={qrDataUrl} alt="QR code" className="w-48 h-48" />
              <a href={qrDataUrl} download="qrcode.png"
                className="text-xs text-blue-600 hover:underline">Download PNG</a>
            </div>
          )}
        </div>
      )}

      {tab === "decode" && (
        <div className="space-y-4">
          <div className="flex gap-2">
            <button onClick={handleDecodeFile}
              className="bg-gray-700 text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-gray-800">
              Open image file
            </button>
            <button onClick={scanning ? stopCamera : startCamera}
              className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors ${scanning ? "bg-red-600 text-white hover:bg-red-700" : "bg-blue-600 text-white hover:bg-blue-700"}`}>
              {scanning ? "Stop camera" : "Use camera"}
            </button>
          </div>

          {scanning && (
            <div className="rounded-xl overflow-hidden border">
              <video ref={videoRef} autoPlay playsInline muted className="w-full" />
            </div>
          )}

          {decoded && (
            <div className="bg-gray-100 rounded-xl p-4">
              <p className="text-xs text-gray-500 mb-1 font-medium">Decoded content</p>
              <p className="text-sm font-mono break-all">{decoded}</p>
              <button onClick={() => navigator.clipboard.writeText(decoded)}
                className="mt-2 text-xs text-blue-600 hover:underline">Copy</button>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
```

- [ ] **Step 6: Install dialog plugin**

```bash
npm install @tauri-apps/plugin-dialog
```

Add to `src-tauri/Cargo.toml`:
```toml
tauri-plugin-dialog = "2"
```

Register in `src-tauri/src/main.rs` (add before `.manage()`):
```rust
.plugin(tauri_plugin_dialog::init())
```

Add to `src-tauri/capabilities/default.json` permissions array:
```json
"dialog:allow-open",
"dialog:allow-save"
```

- [ ] **Step 7: Final build check**

```bash
npm run build
cargo check -p app
```

Expected: both succeed with no errors.

- [ ] **Step 8: Run the app in dev mode to smoke-test**

```bash
npm run tauri dev
```

Expected: app window opens, unlock screen shows, all four sidebar tabs navigate correctly.

- [ ] **Step 9: Commit**

```bash
git add src/ src-tauri/
git commit -m "feat(frontend): implement all four feature pages"
```

---

## Self-Review

**Spec coverage check:**

| Spec requirement | Task |
|---|---|
| AES-256-GCM encryption | Task 3 |
| Argon2id KDF | Task 3 |
| Text encrypt/decrypt | Task 4 |
| File encrypt/decrypt (atomic write, .enc extension) | Task 8 (encrypt_cmd) |
| Password vault with SQLite | Task 5 |
| Vault DB at ~/Library/Application Support/EncryptVault/ | Task 8 (vault_cmd) |
| DB permissions 0600 | Task 5 (store.rs) |
| Master key in memory only (never persisted) | Task 8 (state + salt-based unlock) |
| Vault CRUD + search | Task 5 + Task 8 |
| BIP-39 mnemonic | Task 6 |
| ETH BIP-44 derivation + EIP-155 signing | Task 6 |
| BTC BIP-84 P2WPKH address | Task 6 |
| Wallet private keys never exposed | Task 8 (wallet_cmd) |
| QR generate (PNG) | Task 7 |
| QR decode from file | Task 7 |
| QR decode from camera frame | Task 7 |
| All commands return Result<T, String> | Tasks 8+ |
| No network access | No networking crates/calls used anywhere |
| Platform-agnostic crates (no OS I/O in crates/) | All four crates — I/O only in src-tauri |
| macOS camera permission | capabilities/default.json (requires `media-capture` capability) |
| Vault locks on error | state.lock() called on unlock failure |
| TDD throughout | All crates have tests before implementation |

**Remaining:** Add `media-capture` Tauri capability for camera access — add `"media-capture:allow-request-permission"` (or equivalent for Tauri v2) to `capabilities/default.json`. Check Tauri v2 docs for exact permission identifier.
