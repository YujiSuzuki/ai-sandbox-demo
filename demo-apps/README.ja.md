# SecureNote デモアプリケーション

[AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) のデモンストレーションアプリケーション - AIコーディングアシスタントを使いながら秘密情報を保護する方法を体験できます。コンテナ間アクセスの制御は [HostMCP](https://github.com/YujiSuzuki/hostmcp) が担います。

[English README is here](README.md)

> ⚠️ **注意:** このデモは動作検証が十分ではありません。不具合を見つけたら、HostMCPを使ってAIに調査してもらうことができます。それ自体がサンドボックスの実践的な使い方です。

## このデモが示すもの

### 問題
通常、DevContainer内でAIアシスタント（Claude Code、Gemini Code Assist）を使用すると:
- AIはコンテナにマウントされたすべてのファイルを見ることができる
- 機密ファイル（APIキー、暗号化キー、`.env`）が露出する
- AIの学習データに誤って漏洩するリスク

### 解決策

AI Sandbox がボリュームマウントで秘匿情報を隠し、HostMCP がAIに他コンテナへの制御されたアクセスを提供します。一般的な仕組みは [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox#readme) と [HostMCP](https://github.com/YujiSuzuki/hostmcp#readme) を参照してください — このデモはそれらを具体的なアプリに適用した例です。

## アーキテクチャ

```
┌──────────────────────────────────────────┐
│ DevContainer (AI環境)                    │
│                                          │
│ secrets/ → 空（tmpfs）     🔐 隠蔽       │
│ .env → /dev/null           🔐 隠蔽       │
│                                          │
│ Claude Codeができること:                 │
│ ✅ アプリケーションコードを読む           │
│ ✅ HostMCPでAPIログを確認              │
│ ✅ HostMCPでテスト実行                 │
│ 🔐 秘密情報は読めない                    │
└──────────────────────────────────────────┘

┌──────────────────────────────────────────┐
│ API Container (プロジェクト実行環境)      │
│                                          │
│ secrets/ → 実ファイル      ✅ 見える     │
│ .env → 実際の設定          ✅ 見える     │
│                                          │
│ APIは完全アクセスで正常動作              │
└──────────────────────────────────────────┘
```

## クイックスタート

**所要時間:** 約15分
**必要なもの:** Docker Desktop（または OrbStack） + HostMCP接続済みの [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox)

1. AI Sandbox + HostMCP をセットアップ — [ai-sandbox README のオプションB](https://github.com/YujiSuzuki/ai-sandbox#オプションb-sandbox--hostmcp)、またはより詳しい [はじめにガイド](https://github.com/YujiSuzuki/ai-sandbox/blob/main/docs/getting-started.ja.md) を参照
2. AI Sandbox 内で、AI に「デモアプリをビルドして起動して」と頼む — AI Sandboxワークスペース側の`.sandbox/host-tools/`にある `docker-compose-build.sh`・`docker-compose-up.sh` が、このデモの`demo-apps/docker-compose.demo.yml`（例: `demo-project/demo-apps/docker-compose.demo.yml` — 配置は[macOSセットアップガイド](../macos-setup.ja.md#3-デモアプリをダウンロードする)参照）に対して HostMCP 経由で実行されます（初回は `hostmcp tools sync` での承認が必要です）
3. AI に「securenote-apiのログを見せて」「コンテナの一覧を見せて」などと頼んで、起動が完了しているか確認する — HostMCP経由でAIが直接確認できるので、自分でログを監視する必要はありません。次に進む前に、起動完了の確認を待ってください。
4. `/etc/hosts` にカスタムドメインを追加（初回のみ、ホストOS上で）:
   ```bash
   echo "127.0.0.1 securenote.test api.securenote.test" | sudo tee -a /etc/hosts
   ```
   > **注意:** nginx設定によりドメイン名でのアクセスが必要です。`localhost:8000` では404になります。
5. ブラウザで `http://securenote.test:8000` を開いてログイン:
   - ユーザー名: `demo` パスワード: `demo123`
   - ユーザー名: `alice` パスワード: `alice123`
6. 暗号化されたメモを作成してみる — AIが見えない秘密情報を使って暗号化されます！
7. [ハンズオンガイド](../hands-on.ja.md) のプロンプトを試す — 例:「securenote-apiのログを表示して」

## プロジェクト構造

```
demo-apps/
├── securenote-api/          # バックエンドAPI (Node.js)
│   ├── src/
│   │   ├── server.js
│   │   ├── routes/
│   │   │   ├── auth.js      # JWT認証
│   │   │   ├── notes.js     # 暗号化付きCRUD
│   │   │   └── demo.js      # 秘密情報ステータスエンドポイント
│   │   ├── services/
│   │   │   └── encryption.js
│   │   └── middleware/
│   ├── secrets/             # 🔒 AIから隠蔽
│   │   ├── jwt-secret.key
│   │   └── encryption.key
│   ├── .env                 # 🔒 AIから隠蔽
│   └── tests/
│
├── securenote-web/          # Webフロントエンド (React + Vite)
│   ├── src/
│   │   ├── App.jsx
│   │   ├── pages/
│   │   ├── components/
│   │   └── services/
│   └── Dockerfile
│
└── docker-compose.demo.yml  # デモオーケストレーション
```

## APIエンドポイント

### 認証
- `POST /api/auth/login` - ユーザー名/パスワードでログイン

### ノート（認証必要）
- `GET /api/notes` - すべてのノートを一覧表示（復号化済み）
- `GET /api/notes/:id` - 特定のノートを取得
- `POST /api/notes` - 新しいノートを作成（暗号化）
- `PUT /api/notes/:id` - ノートを更新
- `DELETE /api/notes/:id` - ノートを削除

### デモ
- `GET /api/health` - ヘルスチェック
- `GET /api/demo/secrets-status` - 秘密情報が読み込まれているか確認

## 秘密情報隔離のテスト

### DevContainer（AI Sandbox環境）から:

```bash
# 秘密情報を読もうとする
cat demo-apps/securenote-api/secrets/jwt-secret.key
# 出力: (空またはエラー)

cat demo-apps/securenote-api/.env
# 出力: (空)

# しかしHostMCPは使える！
```

試せるプロンプトは [ハンズオンガイド](../hands-on.ja.md) を参照してください。

### APIが秘密情報を持っていることを確認:

```bash
# デモエンドポイントを呼び出す
curl http://api.securenote.test:8000/api/demo/secrets-status

# レスポンス:
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

## デモを停止

AI Sandbox内で、AIに「デモアプリを停止して」と頼めば、AI Sandboxワークスペース側の`.sandbox/host-tools/docker-compose-down.sh`が、このデモの`demo-apps/docker-compose.demo.yml`（例: `demo-project/demo-apps/docker-compose.demo.yml`）に対してHostMCP経由で実行されます。

## アプリケーションへのアクセス

| アプリケーション | URL |
|---|---|
| **Web版** | http://securenote.test:8000 |
| **API** | http://api.securenote.test:8000 |

> `/etc/hosts` への追加が必要です（クイックスタート参照）

## 詳細情報

- [HostMCP ドキュメント](https://github.com/YujiSuzuki/hostmcp#readme)
- [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox)
- [Model Context Protocol (MCP)](https://modelcontextprotocol.io/)
