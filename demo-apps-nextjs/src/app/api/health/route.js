import { readFile } from "node:fs/promises";
import { createHash } from "node:crypto";

// Runs server-side only (Route Handler) — the secret file is read here,
// but only a hash of it ever leaves this function. This is the point of
// the demo: even though Next.js merges "frontend" and "backend" into one
// process, the secret still needs to be hidden from the AI container the
// same way securenote-api/secrets is (see ../../../../secrets/README.md).
export async function GET() {
  const secretPath = process.env.DEMO_SECRET_PATH || "./secrets/demo-secret.key";

  try {
    // turbopackIgnore: the path comes from env config, not project files —
    // no need to trace it into the build output.
    const secret = (await readFile(/* turbopackIgnore: true */ secretPath, "utf-8")).trim();
    return Response.json({
      status: "ok",
      secretLoaded: true,
      secretFingerprint: createHash("sha256").update(secret).digest("hex").slice(0, 12),
    });
  } catch {
    return Response.json({ status: "ok", secretLoaded: false });
  }
}
