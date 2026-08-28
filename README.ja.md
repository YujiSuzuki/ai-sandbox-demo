# AI Sandbox を試すためのデモアプリ

[AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) のデモアプリケーション集です。
AI アシスタントから機密情報を隠しつつ、[HostMCP](https://github.com/YujiSuzuki/hostmcp) で
クロスコンテナアクセスの体験とiOSアプリのビルドを試せます。

[English README is here](README.md)

## 含まれるもの

| ディレクトリ | 内容 |
|-------------|------|
| [demo-apps/](demo-apps/) | SecureNote Web アプリ — Nginx + Node.js API + React フロントエンド |
| [demo-apps-ios/](demo-apps-ios/) | SecureNote iOS アプリ — SwiftUI + WebView 統合 |
| [demo-apps-nextjs/](demo-apps-nextjs/) | Next.js デモ — フロントエンド+バックエンドが単一プロセス（⚠️ 雛形段階、README参照） |
| [demo-apps-rust/](demo-apps-rust/) | Rust デモ — 単一バイナリの axum サーバー（⚠️ 雛形段階、README参照） |

このデモ自体はHostMCP用ホストツールを持ちません。このリポジトリはAI Sandboxのワークスペースの**内側に配置して使う**想定で（[前提条件](#前提条件)参照）、Docker操作はそのワークスペース側の`.sandbox/host-tools/`に既にある汎用の`docker-compose-up.sh`・`docker-compose-down.sh`・`docker-compose-build.sh`を、このデモの`demo-apps/docker-compose.demo.yml`に向けて実行します。

## 前提条件

先に [AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) をセットアップし、そのワークスペースの中にこのリポジトリを配置してください（例: `ai-sandbox-workspace/demo-project/` のような配置になります）。初めての方は [macOSセットアップガイド](macos-setup.ja.md) を最初から読み進めると、この配置手順も含めて一通り説明されています（具体的な配置例は[手順3](macos-setup.ja.md#3-デモアプリをダウンロードする)を参照）。このデモは Quick Start ガイドのステップ 4 に該当します。

## はじめ方

- **Web デモ** → [demo-apps/README.ja.md](demo-apps/README.ja.md)
- **iOS デモ** → [demo-apps-ios/README.ja.md](demo-apps-ios/README.ja.md)
- **ハンズオン演習**（これらのデモアプリを使って AI Sandbox のセキュリティ機能を体験） → [hands-on.ja.md](hands-on.ja.md)
- **丁寧な手順で進めたい方向け（macOS）** — Homebrew・Docker・VS Code のインストールから順に説明 → [macos-setup.ja.md](macos-setup.ja.md)（[English](macos-setup.md)）

> ⚠️ これらのデモは十分にテストされていません。問題が見つかった場合は HostMCP を使って AI に調査させてみてください — それ自体がサンドボックスの実践的なユースケースです。

## ライセンス

MIT
