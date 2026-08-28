# AI Sandbox Demo

Demo applications for [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) — a Docker-based environment that hides secrets from AI assistants while enabling cross-container access via [HostMCP](https://github.com/YujiSuzuki/hostmcp). Try out cross-container access and building the iOS app hands-on.

[日本語版はこちら](README.ja.md)

## What's Included

| Directory | Description |
|-----------|-------------|
| [demo-apps/](demo-apps/) | SecureNote web app — Nginx + Node.js API + React frontend |
| [demo-apps-ios/](demo-apps-ios/) | SecureNote iOS app — SwiftUI + WebView integration |
| [demo-apps-nextjs/](demo-apps-nextjs/) | Next.js demo — single process frontend+backend (⚠️ scaffold, see its README) |
| [demo-apps-rust/](demo-apps-rust/) | Rust demo — single-binary axum server (⚠️ scaffold, see its README) |

These demos don't ship their own HostMCP host tools — this repo is meant to be placed *inside* an AI Sandbox workspace (see [Prerequisites](#prerequisites)), and Docker operations go through the generic `docker-compose-up.sh` / `docker-compose-down.sh` / `docker-compose-build.sh` already provided in that workspace's `.sandbox/host-tools/`, pointed at this demo's `demo-apps/docker-compose.demo.yml`.

## Prerequisites

Set up [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) first, and place this repo inside that workspace (this demo ends up as e.g. `ai-sandbox-workspace/demo-project/`). New to this? Start from the top of the [macOS Setup Guide](macos-setup.md) — it walks through both steps (see [Step 3](macos-setup.md#3-download-this-demo-app) specifically for the exact layout). These demo apps are Step 4 of the Quick Start guide.

## Getting Started

- **Web demo** → [demo-apps/README.md](demo-apps/README.md)
- **iOS demo** → [demo-apps-ios/README.md](demo-apps-ios/README.md)
- **Hands-on exercises** (using these demo apps to explore AI Sandbox's security features) → [hands-on.md](hands-on.md)
- **Detailed step-by-step setup (macOS)** — from installing Homebrew, Docker, and VS Code → [macos-setup.md](macos-setup.md) ([日本語](macos-setup.ja.md))

> ⚠️ These demos have not been fully tested. If you find issues, use HostMCP to have AI investigate them — that itself is a practical use case for the sandbox.

## License

MIT
