# Hands-on Guide

[日本語版はこちら](hands-on.ja.md)

Hands-on exercises for [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox)'s security features, using the SecureNote demo apps in this repository.

[← Back to README](README.md)

---

## Prerequisites

- [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) is running, with HostMCP connected — see the [Getting Started Guide](https://github.com/YujiSuzuki/ai-sandbox/blob/main/docs/getting-started.md), Steps 1–6
- The SecureNote demo apps from this repo are running — see [demo-apps/README.md](demo-apps/README.md) for setup

---

## Talk to the AI

In the AI Sandbox, try these prompts with Claude Code (or Gemini):

### With the demo apps running

```
"Show me the list of containers"
→ Available containers listed via HostMCP

"Show me the logs from securenote-api"
→ Container logs displayed via HostMCP

"Run npm test in securenote-api"
→ Test results returned
```

### HostMCP Features

```
"Show me detailed info about the securenote-api container"
→ Container inspect results displayed

"How much memory is securenote-api using?"
→ Container resource stats displayed
```

---

## What This Demonstrates

- **Secrets stay hidden** — Ask the AI to read `demo-apps/securenote-api/.env` or the contents of `demo-apps/securenote-api/secrets/`. They appear empty inside the AI Sandbox (hidden by volume mount), even though the `securenote-api` container itself has full access to them.
- **Cross-container access still works** — Even with secrets hidden, the AI can read logs and run tests inside the separate `securenote-api` container via HostMCP, without any direct Docker socket access.

---

## Troubleshooting

### Demo app containers not found

- Run `docker ps` on the host OS to verify the containers are running
- Re-run `docker compose -f docker-compose.demo.yml up -d --build` (from `demo-apps/`)
- Check that `allowed_containers` in your `hostmcp.yaml` includes the container name patterns (e.g. `securenote-*`)

For general AI Sandbox / HostMCP connection issues, see [AI Sandbox's Troubleshooting guide](https://github.com/YujiSuzuki/ai-sandbox/blob/main/docs/reference.md#troubleshooting).

---

[← Back to README](README.md)
