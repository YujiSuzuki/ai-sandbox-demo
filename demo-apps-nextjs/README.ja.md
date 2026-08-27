# AI Sandbox Next.js デモ

[AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) 向けのデモアプリです。[demo-apps/](../demo-apps/) の SecureNote デモと同じ「シークレットの隠蔽」というテーマを、web/api を分割せずフロントエンドとバックエンドが1プロセスで動く Next.js アプリで示します。

> ⚠️ **雛形段階:** 現時点ではシークレットファイルを読むルートハンドラ1本だけの最小構成で、`demo-apps/` のような一通りのウォークスルーにはなっていません。

[English version here](README.md)

## このデモが示すもの

`src/app/api/health/route.js` がサーバー側で `secrets/demo-secret.key` を読み込み、シークレットそのものではなくフィンガープリントのみをブラウザに返します。このファイル（および `.env`）は、`demo-apps/securenote-api/secrets` と同じく Docker ボリュームマウントによって AI Sandbox コンテナから隠蔽されます。詳細は [secrets/README.ja.md](secrets/README.ja.md) を参照してください。

## クイックスタート

**必要なもの:** Docker Desktop（または OrbStack）+ HostMCP に接続済みの [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) — [ai-sandbox README の Option B](https://github.com/YujiSuzuki/ai-sandbox#option-b-sandbox--hostmcp) を参照してください。

1. AI Sandbox 内で AI に「Next.js デモをビルドして起動して」と依頼します。これにより、ワークスペースの `.sandbox/host-tools/` にある `docker-compose-build.sh` / `docker-compose-up.sh` が、この Next.js デモの `docker-compose.demo.yml` を対象に HostMCP 経由で実行されます。
2. 続行する前に、AI に HostMCP 経由で `demo-nextjs` コンテナのログやヘルスチェックを確認してもらいます。
3. ブラウザで `http://localhost:3001` を開きます。

### ローカル開発（サンドボックス外）

```bash
npm install
cp .env.example .env
npm run dev
```

> **Note:** AI Sandbox コンテナ自体の中（このデモ用の Docker コンテナではなく）で `npm run build` / `npm run start` を実行する場合は、先に `NODE_ENV` を unset してください（`env -u NODE_ENV npm run build`）。サンドボックスコンテナは自身のツール用に `NODE_ENV=development` をグローバルに export しており、これを Next.js のビルドワーカーが継承すると `Cannot read properties of null (reading 'useContext')` でクラッシュします。Dockerfile 経由の `docker build` はクリーンな環境から始まるためこの影響を受けません。

## ライセンス

MIT
