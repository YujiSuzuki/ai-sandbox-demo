# AI Sandbox Rust Demo

A demonstration application for [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox), showing the same secret-hiding idea as the [demo-apps/](../demo-apps/) SecureNote demo and the [demo-apps-nextjs/](../demo-apps-nextjs/) demo, but as a single-binary [axum](https://github.com/tokio-rs/axum) server instead of separate web/api containers or a Node.js process.

> ⚠️ **Scaffold status:** this demo is a minimal skeleton (one route handler reading a secret file) — not yet a full walkthrough like `demo-apps/`. Contributions welcome.

[日本語版はこちら](README.ja.md)

## What This Demonstrates

`src/main.rs`'s `/api/health` handler reads `secrets/demo-secret.key` server-side and returns only a fingerprint of it — never the secret itself — to the browser. That file (and `.env`) are hidden from the AI Sandbox container the same way `demo-apps/securenote-api/secrets` is, via Docker volume mounts. See [secrets/README.md](secrets/README.md) for details.

## Quick Start

**Requirements:** Docker Desktop (or OrbStack) + [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) with HostMCP connected — see the [ai-sandbox README, Option B](https://github.com/YujiSuzuki/ai-sandbox#option-b-sandbox--hostmcp).

1. In the AI Sandbox, ask the AI to "build and start the Rust demo" — this runs your workspace's `docker-compose-build.sh` / `docker-compose-up.sh` from `.sandbox/host-tools/`, pointed at this demo's `docker-compose.demo.yml`, via HostMCP.
2. Ask the AI to check the `demo-rust` container logs or health via HostMCP before continuing.
3. Open `http://localhost:3002` in your browser.

### Local development (without AI Sandbox)

Requires a local Rust toolchain ([rustup.rs](https://rustup.rs/)):

```bash
cp .env.example .env
cargo run
```

> **Note:** `Cargo.lock` is gitignored — the Docker build runs entirely inside the container image with no local Rust toolchain available to generate it there. If you run `cargo run`/`cargo build` locally, please commit the resulting `Cargo.lock`.

## License

MIT
