# Secrets Directory

This directory contains sensitive information that should NOT be visible to AI assistants.

## Files

- `demo-secret.key` - Dummy secret read by the `/api/health` handler

## Security Demo

When running in the AI Sandbox environment (DevContainer or cli_claude):
- This directory is mounted as an empty tmpfs volume
- AI assistants cannot read these files
- But the Rust container CAN access them

This demonstrates that even a single-binary server like this axum demo still
needs its server-side secrets hidden from the AI container the same way a
split web/api setup does — see the
[demo-apps/securenote-api/secrets](../../demo-apps/securenote-api/secrets)
and [demo-apps-nextjs/secrets](../../demo-apps-nextjs/secrets) versions of
this same demo.

## Production Use

In production:
- Use environment variables or secret management services
- Never commit secrets to git
- Rotate keys regularly
