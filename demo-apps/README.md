# SecureNote Demo Application

A demonstration application for [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) — showing how to safely use AI coding assistants while protecting secrets, with [HostMCP](https://github.com/YujiSuzuki/hostmcp) providing controlled cross-container access.

> ⚠️ **Note:** This demo has not been fully tested. If you find any issues, you can use HostMCP to have AI investigate them. That itself is a practical use case for the sandbox.

## What This Demonstrates

### The Problem
When using AI assistants (Claude Code, Gemini Code Assist) in DevContainers:
- AI can see all files mounted in the container
- Sensitive files (API keys, encryption keys, `.env`) are exposed
- Risk of accidental leakage to AI training data

### The Solution

AI Sandbox hides secrets via volume mounts, and HostMCP gives AI controlled access to other containers instead. See [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox#readme) and [HostMCP](https://github.com/YujiSuzuki/hostmcp#readme) for how these work in general — this demo shows them applied to a concrete app.

## Architecture

```
┌──────────────────────────────────────────┐
│ DevContainer (AI Environment)            │
│                                          │
│ secrets/ → empty (tmpfs)      🔐 Hidden │
│ .env → /dev/null              🔐 Hidden │
│                                          │
│ Claude Code can:                         │
│ ✅ Read application code                 │
│ ✅ Use HostMCP to check API logs        │
│ ✅ Use HostMCP to run tests             │
│ 🔐 Cannot read secrets                   │
└──────────────────────────────────────────┘

┌──────────────────────────────────────────┐
│ API Container (Project Runtime)          │
│                                          │
│ secrets/ → real files         ✅ Visible │
│ .env → real config            ✅ Visible │
│                                          │
│ API works normally with full access      │
└──────────────────────────────────────────┘
```

## Quick Start

**Time:** ~15 minutes
**Requirements:** Docker Desktop (or OrbStack) + [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) with HostMCP connected

1. Set up AI Sandbox + HostMCP — see the [ai-sandbox README, Option B](https://github.com/YujiSuzuki/ai-sandbox#option-b-sandbox--hostmcp) (or the more detailed [Getting Started Guide](https://github.com/YujiSuzuki/ai-sandbox/blob/main/docs/getting-started.md))
2. In the AI Sandbox, ask the AI to "build and start the demo app" — this runs your AI Sandbox workspace's `docker-compose-build.sh` / `docker-compose-up.sh` from `.sandbox/host-tools/`, pointed at this demo's `demo-apps/docker-compose.demo.yml` (e.g. `demo-project/demo-apps/docker-compose.demo.yml` — see the [macOS Setup Guide](../macos-setup.md#3-download-this-demo-app) for the layout), via HostMCP (first run requires approval via `hostmcp tools sync`)
3. Ask the AI to check that the containers are ready — e.g. "check the securenote-api logs" or "show me the container list" — it can check via HostMCP directly, so you don't need to watch logs yourself. Wait for this confirmation before continuing.
4. Add the custom domain to `/etc/hosts` (first time only, on the host OS):
   ```bash
   echo "127.0.0.1 securenote.test api.securenote.test" | sudo tee -a /etc/hosts
   ```
   > **Note:** Domain-based access is required due to nginx configuration. `localhost:8000` returns 404.
5. Open `http://securenote.test:8000` in your browser and log in:
   - Username: `demo` Password: `demo123`
   - Username: `alice` Password: `alice123`
6. Create some encrypted notes — they're encrypted using secrets that AI cannot see!
7. Try the prompts in the [Hands-on Guide](../hands-on.md) — e.g. "Show me logs from securenote-api"

## Project Structure

```
demo-apps/
├── securenote-api/          # Backend API (Node.js)
│   ├── src/
│   │   ├── server.js
│   │   ├── routes/
│   │   │   ├── auth.js      # JWT authentication
│   │   │   ├── notes.js     # CRUD with encryption
│   │   │   └── demo.js      # Secrets status endpoint
│   │   ├── services/
│   │   │   └── encryption.js
│   │   └── middleware/
│   ├── secrets/             # 🔒 Hidden from AI
│   │   ├── jwt-secret.key
│   │   └── encryption.key
│   ├── .env                 # 🔒 Hidden from AI
│   └── tests/
│
├── securenote-web/          # Web Frontend (React + Vite)
│   ├── src/
│   │   ├── App.jsx
│   │   ├── pages/
│   │   ├── components/
│   │   └── services/
│   └── Dockerfile
│
└── docker-compose.demo.yml  # Demo orchestration
```

## API Endpoints

### Authentication
- `POST /api/auth/login` - Login with username/password

### Notes (requires auth)
- `GET /api/notes` - List all notes (decrypted)
- `GET /api/notes/:id` - Get specific note
- `POST /api/notes` - Create new note (encrypted)
- `PUT /api/notes/:id` - Update note
- `DELETE /api/notes/:id` - Delete note

### Demo
- `GET /api/health` - Health check
- `GET /api/demo/secrets-status` - Verify secrets are loaded

## Testing Secret Isolation

### From DevContainer (AI environment):

```bash
# Try to read secrets
cat demo-apps/securenote-api/secrets/jwt-secret.key
# Output: (empty or error)

cat demo-apps/securenote-api/.env
# Output: (empty)

# But you can use HostMCP!
```

See the [Hands-on Guide](../hands-on.md) for prompts to try.

### Verify API has secrets:

```bash
# Call the demo endpoint
curl http://api.securenote.test:8000/api/demo/secrets-status

# Response:
{
  "message": "This API has access to secrets",
  "secretsLoaded": true,
  "proof": {
    "jwtSecretLoaded": true,
    "jwtSecretPreview": "super-sec***",
    "encryptionKeyLoaded": true
  }
}
```

## Stop Demo

In the AI Sandbox, ask the AI to "stop the demo app" — this runs your AI Sandbox workspace's `.sandbox/host-tools/docker-compose-down.sh`, pointed at this demo's `demo-apps/docker-compose.demo.yml` (e.g. `demo-project/demo-apps/docker-compose.demo.yml`), via HostMCP.

## Access the Application

| Application | URL |
|---|---|
| **Web** | http://securenote.test:8000 |
| **API** | http://api.securenote.test:8000 |

> Requires the `/etc/hosts` entry from Quick Start.

## Learn More

- [HostMCP Documentation](https://github.com/YujiSuzuki/hostmcp#readme)
- [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox)
- [Model Context Protocol (MCP)](https://modelcontextprotocol.io/)
