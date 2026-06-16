# SecureNote Demo Application

A demonstration application for **DockMCP** - showing how to safely use AI coding assistants while protecting secrets.

> ⚠️ **Note:** This demo has not been fully tested. If you find any issues, you can use DockMCP to have AI investigate them. That itself is a practical use case for DockMCP.

## What This Demonstrates

### The Problem
When using AI assistants (Claude Code, Gemini Code Assist) in DevContainers:
- AI can see all files mounted in the container
- Sensitive files (API keys, encryption keys, `.env`) are exposed
- Risk of accidental leakage to AI training data

### The Solution
**DockMCP** provides:
1. **Secret isolation** - Hide sensitive files from AI using volume mounts
2. **Cross-container access** - AI can still check logs, run tests via MCP
3. **Normal development** - No workflow disruption

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
│ ✅ Use DockMCP to check API logs        │
│ ✅ Use DockMCP to run tests             │
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

### Option 1: Web Demo (Recommended First Try)

**Time:** 5 minutes
**Requirements:** Docker Desktop only

```bash
# 1. Start the demo application
cd demo-apps
docker-compose -f docker-compose.demo.yml up -d

# 2. Watch logs until services are ready (~30 seconds)
#    Press Ctrl+C once you see "Server running on port 8080"
docker-compose -f docker-compose.demo.yml logs -f

# 3. Add custom domain to /etc/hosts (first time only)
echo "127.0.0.1 securenote.test api.securenote.test" | sudo tee -a /etc/hosts

# 4. Open in browser
open http://securenote.test:8000
```

> **Note:** Domain-based access is required due to nginx configuration. `localhost:8000` returns 404.

**Login:**
- Username: `demo` Password: `demo123`
- Username: `alice` Password: `alice123`

**Try it out:**
1. Login with demo credentials
2. Create some encrypted notes
3. Notes are encrypted using secrets that AI cannot see!

### Option 2: With DockMCP (Full Experience)

**Time:** 15 minutes
**Requirements:** Docker Desktop + DockMCP

```bash
# 1. Install and start DockMCP on host OS (see https://github.com/YujiSuzuki/dkmcp)
go install github.com/YujiSuzuki/dkmcp@latest
curl -L https://raw.githubusercontent.com/YujiSuzuki/dkmcp/main/configs/dkmcp.example.yaml -o dkmcp.yaml
dkmcp serve --config dkmcp.yaml
# DockMCP running on http://localhost:8080

# 2. Start demo application
cd demo-apps
docker-compose -f docker-compose.demo.yml up -d

# 3. Open DevContainer in VS Code
code ..
# Claude Code will connect to DockMCP automatically

# 4. Try asking Claude Code:
"Show me logs from securenote-api"
"Run tests in securenote-api container"
"Check if secrets are loaded in the API"
```

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

# But you can use DockMCP!
# Ask Claude Code: "Check the API logs"
# Ask Claude Code: "Run npm test in securenote-api"
```

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

```bash
cd demo-apps
docker-compose -f docker-compose.demo.yml down
```

## Learn More

- [DockMCP Documentation](https://github.com/YujiSuzuki/dkmcp#readme)
- [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox)
- [Model Context Protocol (MCP)](https://modelcontextprotocol.io/)
