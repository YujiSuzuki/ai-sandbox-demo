# AI Sandbox Rust デモ

[AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) 向けのデモアプリです。[demo-apps/](../demo-apps/) の SecureNote デモや [demo-apps-nextjs/](../demo-apps-nextjs/) デモと同じ「シークレットの隠蔽」というテーマを、web/api を分割したコンテナや Node.js プロセスではなく、単一バイナリの [axum](https://github.com/tokio-rs/axum) サーバーで示します。

> ⚠️ **雛形段階:** 現時点ではシークレットファイルを読むルートハンドラ1本だけの最小構成で、`demo-apps/` のような一通りのウォークスルーにはなっていません。

[English version here](README.md)

## このデモが示すもの

`src/main.rs` の `/api/health` ハンドラがサーバー側で `secrets/demo-secret.key` を読み込み、シークレットそのものではなくフィンガープリントのみをブラウザに返します。このファイル（および `.env`）は、`demo-apps/securenote-api/secrets` と同じく Docker ボリュームマウントによって AI Sandbox コンテナから隠蔽されます。詳細は [secrets/README.ja.md](secrets/README.ja.md) を参照してください。

## クイックスタート

**必要なもの:** Docker Desktop（または OrbStack）+ HostMCP に接続済みの [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) — [ai-sandbox README の Option B](https://github.com/YujiSuzuki/ai-sandbox#option-b-sandbox--hostmcp) を参照してください。

1. AI Sandbox 内で AI に「Rust デモをビルドして起動して」と依頼します。これにより、ワークスペースの `.sandbox/host-tools/` にある `docker-compose-build.sh` / `docker-compose-up.sh` が、この Rust デモの `docker-compose.demo.yml` を対象に HostMCP 経由で実行されます。
2. 続行する前に、AI に HostMCP 経由で `demo-rust` コンテナのログやヘルスチェックを確認してもらいます。
3. ブラウザで `http://localhost:3002` を開きます。

### ローカル開発（AI Sandbox を使わずに直接実行を試す場合）

ローカルの Rust ツールチェイン（[rustup.rs](https://rustup.rs/)）が必要です：

```bash
cp .env.example .env
cargo run
```

> **Note:** `Cargo.lock` は gitignore 対象です — Docker ビルドはコンテナイメージ内で完結しており、生成に使えるローカルの Rust ツールチェインがありません。ローカルで `cargo run` / `cargo build` を実行した場合は、生成された `Cargo.lock` をコミットしてください。

## ライセンス

MIT
