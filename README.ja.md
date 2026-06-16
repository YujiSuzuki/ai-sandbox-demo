# AI Sandbox デモ

[AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) のデモアプリケーション集です。AI アシスタントから機密情報を隠しつつ、[DockMCP](https://github.com/YujiSuzuki/dkmcp) でクロスコンテナアクセスを実現する Docker ベースの環境です。

[English README is here](README.md)

## 含まれるもの

| ディレクトリ | 内容 |
|-------------|------|
| [demo-apps/](demo-apps/) | SecureNote Web アプリ — Nginx + Node.js API + React フロントエンド |
| [demo-apps-ios/](demo-apps-ios/) | SecureNote iOS アプリ — SwiftUI + WebView 統合 |

DockMCP 用ホストツール（`demo-build.sh`、`demo-up.sh`、`demo-down.sh`）は `.sandbox/host-tools/` にあります。

## 前提条件

先に [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) をセットアップしてください。このデモは Quick Start ガイドのステップ 5 に該当します。

## はじめ方

- **Web デモ** → [demo-apps/README.ja.md](demo-apps/README.ja.md)
- **iOS デモ** → [demo-apps-ios/README.ja.md](demo-apps-ios/README.ja.md)

> ⚠️ これらのデモは十分にテストされていません。問題が見つかった場合は DockMCP を使って AI に調査させてみてください — それ自体がサンドボックスの実践的なユースケースです。

## ライセンス

MIT
