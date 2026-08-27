# Secrets Directory

This directory contains sensitive information that should NOT be visible to AI assistants.

## Files

- `demo-secret.key` - Dummy secret read by the `/api/health` route handler

## Security Demo

When running in the AI Sandbox environment (DevContainer or cli_claude):
- This directory is mounted as an empty tmpfs volume
- AI assistants cannot read these files
- But the Next.js container CAN access them

This demonstrates that even a single-process, frontend+backend-merged
framework like Next.js still needs its server-side secrets hidden from the
AI container the same way a split web/api setup does — see the
[demo-apps/securenote-api/secrets](../../demo-apps/securenote-api/secrets)
version of this same demo for the split-container equivalent.

## Production Use

In production:
- Use environment variables or secret management services
- Never commit secrets to git
- Rotate keys regularly
