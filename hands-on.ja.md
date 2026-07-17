# ハンズオンガイド

[English README is here](hands-on.md)

このリポジトリに含まれる SecureNote デモアプリを使って、[AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) のセキュリティ機能を実際に体験する演習です。

[← README に戻る](README.ja.md)

---

## 前提条件

- [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) が起動しており、HostMCP が接続済みであること — [はじめにガイド](https://github.com/YujiSuzuki/ai-sandbox/blob/main/docs/getting-started.ja.md) のステップ1〜6を参照
- このリポジトリの SecureNote デモアプリが起動していること — セットアップ方法は [demo-apps/README.ja.md](demo-apps/README.ja.md) を参照

---

## AIに話しかけてみる

AI Sandbox 内で、Claude Code（または Gemini）に次のようなプロンプトを試してみてください:

### デモアプリ起動後

```
「コンテナの一覧を見せて」
→ HostMCP経由で利用可能なコンテナが一覧表示される

「securenote-api のログを見せて」
→ HostMCP経由でコンテナログが表示される

「securenote-api で npm test を実行して」
→ テスト結果が返される
```

### HostMCPの機能

```
「securenote-api コンテナの詳細情報を見せて」
→ コンテナのinspect結果が表示される

「securenote-api のメモリ使用量は？」
→ コンテナのリソース統計が表示される
```

---

## これが示していること

- **秘匿情報は隠されたまま** — AI に `demo-apps/securenote-api/.env` や `demo-apps/securenote-api/secrets/` の中身を読ませてみてください。`securenote-api` コンテナ自身はこれらに完全にアクセスできますが、AI Sandbox 内では（ボリュームマウントにより）空に見えます。
- **それでもコンテナ間アクセスは機能する** — 秘匿情報が隠された状態でも、AI は Docker ソケットへの直接アクセスなしに、HostMCP 経由で別コンテナである `securenote-api` のログ確認やテスト実行ができます。

---

## トラブルシューティング

### デモアプリのコンテナが見つからない

- ホストOSで `docker ps` を実行し、コンテナが起動しているか確認
- （`demo-apps/` から）`docker compose -f docker-compose.demo.yml up -d --build` を再実行
- `hostmcp.yaml` の `allowed_containers` にコンテナ名のパターン（例: `securenote-*`）が含まれているか確認

AI Sandbox / HostMCP 全般の接続トラブルについては、[AI Sandbox のトラブルシューティングガイド](https://github.com/YujiSuzuki/ai-sandbox/blob/main/docs/reference.ja.md#トラブルシューティング) を参照してください。

---

[← README に戻る](README.ja.md)
