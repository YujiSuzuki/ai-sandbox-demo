# Secrets ディレクトリ

このディレクトリには、AI アシスタントに見せてはいけない機密情報が含まれています。

## ファイル

- `demo-secret.key` - `/api/health` ハンドラが読み込むダミーのシークレット

## セキュリティデモ

AI Sandbox 環境（DevContainer または cli_claude）で実行している場合：
- このディレクトリは空の tmpfs ボリュームとしてマウントされます
- AI アシスタントはこれらのファイルを読み取れません
- しかし Rust コンテナはアクセス可能です

これは、この axum デモのような単一バイナリのサーバーであっても、web/api を
分割した構成と同じくサーバー側のシークレットを AI コンテナから隠す必要が
あることを示しています — 同じデモの別バージョンとして
[demo-apps/securenote-api/secrets](../../demo-apps/securenote-api/secrets)、
[demo-apps-nextjs/secrets](../../demo-apps-nextjs/secrets) も参照してください。

## 本番環境での利用

本番環境では：
- 環境変数やシークレット管理サービスを使用してください
- シークレットを git にコミットしないでください
- キーは定期的にローテーションしてください
