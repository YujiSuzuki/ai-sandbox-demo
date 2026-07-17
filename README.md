# AI Sandbox Demo

Demo applications for [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) — a Docker-based environment that hides secrets from AI assistants while enabling cross-container access via [HostMCP](https://github.com/YujiSuzuki/hostmcp).

[日本語版はこちら](README.ja.md)

## What's Included

| Directory | Description |
|-----------|-------------|
| [demo-apps/](demo-apps/) | SecureNote web app — Nginx + Node.js API + React frontend |
| [demo-apps-ios/](demo-apps-ios/) | SecureNote iOS app — SwiftUI + WebView integration |

Host tools for HostMCP (`demo-build.sh`, `demo-up.sh`, `demo-down.sh`) are in `.sandbox/host-tools/`.

## Prerequisites

Set up [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) first. These demo apps are Step 5 of the Quick Start guide.

## Getting Started

- **Web demo** → [demo-apps/README.md](demo-apps/README.md)
- **iOS demo** → [demo-apps-ios/README.md](demo-apps-ios/README.md)
- **Hands-on exercises** (using these demo apps to explore AI Sandbox's security features) → [hands-on.md](hands-on.md)

> ⚠️ These demos have not been fully tested. If you find issues, use HostMCP to have AI investigate them — that itself is a practical use case for the sandbox.

## License

MIT
