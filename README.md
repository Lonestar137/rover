# Requirements

- node(or nix-shell)
- Rust stable toolchain
- Rust WASM build target enabled

# Serving parts of this application

## Frontend
```bash
cd frontend
nix-shell ./shell.nix
trunk serve
```

## Backend

```bash
cargo run -p backend
```
