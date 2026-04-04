# EncryptVault — Design Spec

**Date:** 2026-04-04
**Status:** Approved

---

## Overview

EncryptVault is a fully offline desktop application built with Tauri (Rust backend + React/Tailwind frontend). It bundles four security-focused tools into a single local-only app: file/text encryption, a password manager, a crypto wallet (ETH + BTC), and a QR code generator/reader. All cryptographic operations run in Rust; the frontend never handles raw keys or plaintext. The app makes no network requests of any kind.

**Platform roadmap:** v1 targets macOS only. Future versions will expand to iOS and Android via Tauri Mobile. The Rust workspace crates (`encrypt`, `password-vault`, `wallet`, `qr-engine`) are written as platform-agnostic libraries from the start to enable this expansion with minimal rework.

---

## Architecture

### Cargo Workspace

```
EncryptVault/
├── Cargo.toml              # workspace root
├── crates/
│   ├── encrypt/            # AES-256-GCM encryption for text and files
│   ├── password-vault/     # encrypted local credential store
│   ├── wallet/             # HD wallet — ETH + BTC
│   └── qr-engine/          # QR code generation and decoding
├── src-tauri/              # Tauri app crate — commands, app state, lifecycle
└── src/                    # React + Tailwind SPA
```

### Frontend

Single-page React app with a sidebar offering four feature tabs:
- Encrypt / Decrypt
- Password Vault
- Wallet
- QR Code

State management via React context (no external store needed at this scope).

---

## Crate Responsibilities

### `crates/encrypt`
- AES-256-GCM encryption and decryption
- Supports: arbitrary text strings and binary files
- Key derivation: Argon2id from a user-supplied passphrase
- File output: writes `.enc` files alongside originals — never overwrites in place
- No I/O logic — accepts bytes in, returns bytes out; file I/O handled by `src-tauri`

### `crates/password-vault`
- Encrypted SQLite database at `~/Library/Application Support/EncryptVault/vault.db`
- Schema: credentials (id, name, username, password, url, notes, created_at, updated_at)
- Master password → Argon2id → AES-256-GCM key used to encrypt/decrypt the DB
- Vault is loaded into memory on unlock; re-encrypted and flushed on lock or app close
- Operations: create, read, update, delete credentials; search by name/url

### `crates/wallet`
- BIP-39 mnemonic generation and import
- BIP-44 HD derivation for ETH (`m/44'/60'/0'/0/x`) and BTC (`m/44'/0'/0'/0/x`)
- Mnemonic stored encrypted in `vault.db` (same master password key)
- Exposed operations: generate mnemonic, import mnemonic, derive accounts (public address), sign transaction offline
- No network access — no balance queries, no RPC calls, no broadcast
- Private keys never leave this crate — Tauri commands receive only public addresses and signed transaction bytes
- ETH: `ethers` crate (offline signing only); BTC: `bitcoin` crate

### `crates/qr-engine`
- **Generate:** accepts a UTF-8 string, returns a PNG (bytes) encoding a QR code (`qrcode` crate)
- **Decode from file:** accepts file bytes (PNG/JPG/SVG), returns decoded string (`rqrr` or `zbar` binding)
- **Decode from camera frame:** accepts raw RGBA frame bytes + dimensions, returns decoded string or `None`
- No camera I/O — frame capture is handled by the frontend via `getUserMedia`

---

## Data Flow

### Master Password & Unlock
1. First launch: user sets master password → Argon2id derives 256-bit key → empty vault DB created.
2. Subsequent launches: user enters master password → key derived → vault DB decrypted into memory.
3. Derived key lives in memory only (held in Tauri app state behind a `Mutex`). Never persisted to disk.
4. Auto-lock after configurable idle timeout (default: 5 minutes). Manual lock available at any time.

### Encrypt / Decrypt
- Text: frontend sends plaintext + passphrase → `encrypt` crate → ciphertext returned as base64 string.
- File: frontend sends file path + passphrase → `src-tauri` reads file → `encrypt` crate → writes `.enc` file to same directory.
- Decrypt is the reverse; file decrypt writes original filename (stripping `.enc`).

### Password Vault
- All CRUD operations go through Tauri commands → `password-vault` crate → in-memory vault → periodic flush to encrypted DB.
- Search is in-memory (no plaintext ever written to SQLite; rows are individually encrypted).

### Wallet
- On unlock, HD wallet reconstructed in memory from decrypted mnemonic.
- Derive address: frontend requests account index → Tauri → `wallet` crate derives public address → returned to frontend.
- Sign transaction: frontend sends unsigned tx (raw bytes or structured fields) → Tauri → `wallet` crate signs with in-memory key → signed tx bytes returned as hex. User copies and broadcasts manually via an external tool.
- No network calls of any kind — balance and broadcast are out of scope.

### QR Code
- Generate: frontend sends string → Tauri → `qr-engine` → PNG bytes → displayed in frontend as data URL.
- File decode: frontend sends file path → Tauri reads file → `qr-engine` decodes → string returned.
- Camera decode: frontend captures video frames via `getUserMedia`, sends RGBA frame bytes to Tauri on interval → `qr-engine` attempts decode → result returned; frontend shows live viewfinder overlay.

---

## Error Handling

- All Tauri commands return `Result<T, String>`. Errors are user-readable messages (e.g., "Wrong password", "File not found").
- Raw panic messages and stack traces are never surfaced to the frontend.
- Vault operations fail closed — on any unexpected error, the vault locks immediately.
- File encryption never partially writes; uses a temp file + atomic rename pattern.

---

## Security Constraints

- No network access of any kind — fully air-gapped operation.
- Private keys and master-derived key never serialized to disk or sent over any Tauri command.
- macOS `camera` capability declared in `tauri.conf.json`; permission prompt shown on first camera use.
- Vault DB file permissions set to `0600` on creation.

---

## Testing Strategy

- Each crate has unit tests covering its core logic with real cryptographic operations (no mocking of crypto primitives).
- `src-tauri` integration tests spin up a temp vault and exercise the full Tauri command layer end-to-end.
- QR encode→decode round-trip tested in `qr-engine`.
- Wallet derivation vectors tested against BIP-44 reference test vectors.

---

## Out of Scope (v1)

- Cloud sync or backup
- Browser extension integration
- Multi-device support / sync
- iOS and Android (planned for future versions)
- Networks beyond ETH and BTC
- NFT or DeFi features
- Balance queries or transaction broadcast (user broadcasts signed tx via external tools)
