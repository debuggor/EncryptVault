# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## IMPORTANT: Superpowers Required

All development in this repository MUST be performed using superpowers skills. Before doing any work — writing code, planning, debugging, reviewing, or shipping — you MUST invoke the relevant superpowers skill via the `Skill` tool. Direct implementation without going through the appropriate superpowers workflow is not permitted.

## Commands

```bash
# Development (hot reload — Rust + React)
npm run tauri dev

# Run all Rust tests
cargo test

# Run tests for a specific crate
cargo test -p encrypt
cargo test -p password-vault
cargo test -p wallet
cargo test -p qr-engine
cargo test -p settings

# Run a single test by name
cargo test -p encrypt test_name

# Production build → src-tauri/target/release/bundle/macos/
npm run tauri build

# Frontend-only build (no Tauri)
npm run build
```

## Architecture

This is a **Tauri 2 desktop app** — Rust backend + React/TypeScript frontend. All crypto operations happen exclusively in Rust; the frontend never touches keys or plaintext.

### Rust workspace layout

```
Cargo.toml               # workspace root
crates/
  encrypt/               # AES-256-GCM + Argon2id KDF
  password-vault/        # Encrypted SQLite credential store
  wallet/                # BIP-39/44/84, ETH EIP-155, BTC P2WPKH
  qr-engine/             # QR generation (qrcode) + decoding (rqrr)
  settings/              # Master password reset / app settings
src-tauri/src/
  main.rs                # Tauri entry point
  lib.rs                 # re-exports state, paths, commands modules
  state.rs               # AppState (vault_key, vault, wallet — all Mutex-guarded)
  paths.rs               # Platform data dir paths for vault.db and vault.salt
  commands/              # One file per feature domain, registered in main.rs
    encrypt_cmd.rs
    vault_cmd.rs
    wallet_cmd.rs
    qr_cmd.rs
    settings_cmd.rs
```

### Frontend layout

```
src/
  main.tsx               # React root
  App.tsx                # PageRouter: UnlockPage gate → Sidebar + page switch
  context/AppContext.tsx # isUnlocked + currentPage state (no external state lib)
  pages/                 # One file per page: Unlock, Encrypt, Vault, Wallet, QR, Settings
  components/            # Sidebar, ErrorBanner
```

### Key design decisions

- **Auth gate**: `AppContext.isUnlocked` drives routing. `UnlockPage` calls `unlock_vault` Tauri command; on success sets `isUnlocked = true`. All other pages are only rendered after unlock.
- **AppState locking**: `AppState::lock()` zeroes all three Mutex-guarded fields simultaneously. The vault key is a raw `[u8; 32]` held in memory — never persisted.
- **Vault persistence**: `vault.db` and `vault.salt` live in the platform data directory (`~/Library/Application Support/EncryptVault/` on macOS), resolved via `paths.rs`.
- **No networking**: CSP in `tauri.conf.json` is `default-src 'self'`. No network requests anywhere in the codebase.
- **Adding a new Tauri command**: implement in `src-tauri/src/commands/<domain>_cmd.rs`, register via `tauri::Builder::invoke_handler` in `main.rs`, and invoke from the frontend with `invoke("command_name", { args })` from `@tauri-apps/api/core`.
