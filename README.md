# EncryptVault

A fully offline macOS desktop application built with Tauri (Rust + React). Bundles four security tools into one local-only app — no network access, no cloud.

## Features

- **Encrypt / Decrypt** — AES-256-GCM encryption for text and files. Passphrase-based key derivation via Argon2id.
- **Password Vault** — Encrypted local credential store backed by SQLite. Protected by a master password; auto-locks on idle.
- **Wallet** — HD wallet for Ethereum and Bitcoin (BIP-39/44/84). Generates mnemonics, derives addresses, and signs transactions fully offline.
- **QR Code** — Generate QR codes from any text or URL. Decode from image files or live camera scan.

## Architecture

Cargo workspace with four platform-agnostic Rust crates:

| Crate | Responsibility |
|---|---|
| `crates/encrypt` | AES-256-GCM + Argon2id KDF |
| `crates/password-vault` | Encrypted SQLite credential store |
| `crates/wallet` | BIP-39/44/84, ETH EIP-155 signing, BTC P2WPKH |
| `crates/qr-engine` | QR generation and decoding |

`src-tauri` wires the crates into Tauri commands. `src/` is the React/Tailwind frontend. All cryptographic operations stay in Rust — the frontend never handles raw keys or plaintext.

## Security

- All data stored locally; no network requests of any kind
- Master password never stored — only its Argon2id-derived key, held in memory
- Vault auto-locks after 5 minutes of idle (configurable)
- File encryption uses atomic write (temp file + rename) — never overwrites originals
- Wallet private keys never leave the `wallet` crate
- Vault DB file permissions: `0600`

## Platform

- **v1:** macOS
- **Planned:** iOS and Android via Tauri Mobile (Rust crates are platform-agnostic by design)

## Tech Stack

- **Backend:** Rust, Tauri 2, aes-gcm, argon2, rusqlite, bip39, k256, bitcoin, qrcode, rqrr
- **Frontend:** React 18, TypeScript, Vite, Tailwind CSS

## Development

```bash
# Install dependencies
npm install

# Run in dev mode
npm run tauri dev

# Build
npm run tauri build

# Run Rust tests
cargo test
```

## Docs

- [Design Spec](docs/superpowers/specs/2026-04-04-encryptvault-design.md)
- [Implementation Plan](docs/superpowers/plans/2026-04-04-encryptvault.md)
