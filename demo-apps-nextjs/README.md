# AI Sandbox Next.js Demo

A demonstration application for [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox), showing the same secret-hiding idea as the [demo-apps/](../demo-apps/) SecureNote demo, but for a Next.js app where frontend and backend run in a single process instead of separate web/api containers.

> ⚠️ **Scaffold status:** this demo is a minimal skeleton (one route handler reading a secret file) — not yet a full walkthrough like `demo-apps/`. Contributions welcome. The skeleton itself has been verified end-to-end (Docker build, HostMCP logs/healthcheck, browser) — see the Quick Start steps below.

[日本語版はこちら](README.ja.md)

## What This Demonstrates

`src/app/api/health/route.js` reads `secrets/demo-secret.key` server-side and returns only a fingerprint of it — never the secret itself — to the browser. That file (and `.env`) are hidden from the AI Sandbox container the same way `demo-apps/securenote-api/secrets` is, via Docker volume mounts. See [secrets/README.md](secrets/README.md) for details.

## Quick Start

**Requirements:** Docker Desktop (or OrbStack) + [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) with HostMCP connected — see the [ai-sandbox README, Option B](https://github.com/YujiSuzuki/ai-sandbox#option-b-sandbox--hostmcp).

1. In the AI Sandbox, ask the AI to "build and start the Next.js demo" — this runs your workspace's `docker-compose-build.sh` / `docker-compose-up.sh` from `.sandbox/host-tools/`, pointed at this demo's `docker-compose.demo.yml`, via HostMCP.
2. Ask the AI to check the `demo-nextjs` container logs or health via HostMCP before continuing.
3. Open `http://localhost:3001` in your browser.

### Local development (without AI Sandbox)

```bash
npm install
cp .env.example .env
npm run dev
```

> **Note:** if you run `npm run build` / `npm run start` *inside* the AI Sandbox container itself (not the demo's own Docker container), unset `NODE_ENV` first (`env -u NODE_ENV npm run build`). The sandbox container exports `NODE_ENV=development` globally for its own tooling, which the Next.js build workers inherit and crash on (`Cannot read properties of null (reading 'useContext')`) — this doesn't affect `docker build` via the Dockerfile, which starts from a clean environment.

## License

MIT
