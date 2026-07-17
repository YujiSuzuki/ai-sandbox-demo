# SecureNote iOS アプリ

SwiftUI で実装された iOS アプリケーション。ネイティブなログイン画面と、WebView を使用した Web フロントエンドの統合デモです。

> ⚠️ **注意:** このデモは動作検証が十分ではなく、実際にバグがあります。不具合を見つけたら、HostMCPを使ってAIに調査してもらうことができます。それ自体がサンドボックスの実践的な使い方です。
>
> 📝 **ヒント:** 機能の説明（リクエスト・レスポンス例など）にもバグがあります。[demo-apps/README.ja.md](../demo-apps/README.ja.md) のWeb版との違いを確認してみてください。

## アーキテクチャ

```
┌─────────────────────────────────────────┐
│ SecureNote iOS App (SwiftUI)            │
├─────────────────────────────────────────┤
│                                         │
│  ┌─────────────────────────────────┐   │
│  │ LoginView (ネイティブ)            │   │
│  │ - SwiftUI フォーム                │   │
│  │ - API呼び出し                     │   │
│  │ - トークン取得                    │   │
│  └─────────┬───────────────────────┘   │
│            │ (トークン)                  │
│            ↓                             │
│  ┌─────────────────────────────────┐   │
│  │ MainView (WebView統合)          │   │
│  │ - WebViewContainer              │   │
│  │ - トークン注入                    │   │
│  │ - React Web アプリ表示            │   │
│  └─────────────────────────────────┘   │
│                                         │
│  API: http://api.securenote.test:8000  │
│  Web: http://securenote.test:8000      │
└─────────────────────────────────────────┘
```

## 機能

### 1. ネイティブログイン (LoginView)

- **SwiftUI UI**: ネイティブな iOS デザイン
- **API 連携**: `/api/auth/login` エンドポイント
- **エラーハンドリング**: ユーザーフレンドリーなエラー表示
- **ローディング状態**: ログイン中の UI 反応

**リクエスト例:**
```swift
POST http://api.securenote.test:8000/api/auth/login
Content-Type: application/json

{
  "email": "demo@example.com",
  "password": "demo123"
}
```

**レスポンス:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIs...",
  "user": {
    "id": "user123",
    "email": "demo@example.com",
    "name": "Demo User"
  }
}
```

### 2. WebView 統合 (WebViewContainer)

- **トークン注入**: JavaScript に認証トークンを注入
- **localStorage**: `localStorage.setItem('auth_token', token)`
- **React Web アプリ統合**: `http://securenote.test:8000` をロード
- **自動認証**: Web アプリが localStorage からトークンを読み込み

**トークン注入スクリプト:**
```javascript
window.authToken = 'eyJhbGciOiJIUzI1NiIs...';
localStorage.setItem('auth_token', 'eyJhbGciOiJIUzI1NiIs...');
```

### 3. 認証管理 (AuthManager)

- **ステート管理**: ログイン状態とトークン保持
- **API 通信**: async/await による非同期処理
- **エラー管理**: エラーメッセージの管理と表示

## セットアップ

### 前提条件

- **macOS**: 11 以上
- **Xcode**: 14 以上
- **iOS**: 14 以上をターゲット
- **バックエンド**: `demo-apps` が実行中

### 実行方法

#### 1. バックエンドを起動

```bash
cd demo-apps
docker-compose -f docker-compose.demo.yml up -d
```

**確認:**
- API が `http://api.securenote.test:8000/api` で実行中
- Web が `http://securenote.test:8000` で実行中

> **注意:** シミュレータはホストOSの `/etc/hosts` を参照します。一方、実機の場合はホストOSのhosts設定を参照できないため、Mac上にDNSサーバーを構築して実機のDNS参照先に指定するか、プロキシ（CharlesやProxyman等）を経由させる必要があります。

#### 2. Xcode でプロジェクトを開く

```bash
cd demo-apps-ios
open SecureNote.xcodeproj
```

#### 3. シミュレーターで実行

```
Cmd + R (または Play ボタン)
```

#### 4. ログイン

デモアカウントでログイン:
- **Email**: `demo@example.com`
- **Password**: `demo123`

## 開発ガイド

### ファイル構成

```
SecureNote/
├── App.swift                # メインエントリーポイント
├── AuthManager.swift        # 認証状態管理
├── LoginView.swift          # ログイン画面
├── MainView.swift           # メイン画面
├── WebViewContainer.swift   # WebView統合
└── Info.plist              # アプリ設定
```

### 主要なクラス

#### AuthManager
```swift
@Observable
class AuthManager {
    var isLoggedIn: Bool
    var authToken: String

    func login(email: String, password: String) async
    func logout()
}
```

#### WebViewContainer
```swift
struct WebViewContainer: View {
    let authToken: String
    // トークンを注入して React Web アプリをロード
}
```

### UIフロー

```
App.swift
├─ isLoggedIn = false
│  └─ LoginView
│     └─ submit email/password
│        └─ AuthManager.login()
│           └─ API POST /api/auth/login
│              ├─ success
│              │  └─ isLoggedIn = true
│              │     └─ トークン保存
│              └─ error
│                 └─ エラーメッセージ表示
│
└─ isLoggedIn = true
   └─ MainView
      └─ WebViewContainer(token: authToken)
         └─ トークン注入
            └─ http://securenote.test:8000 ロード
               └─ React Web アプリ
```

## API 連携

### ログインエンドポイント

- **URL**: `http://api.securenote.test:8000/api/auth/login`
- **メソッド**: `POST`
- **Content-Type**: `application/json`

**パラメータ:**
```json
{
  "email": "string",
  "password": "string"
}
```

**レスポンス (200 OK):**
```json
{
  "token": "string",
  "user": {
    "id": "string",
    "email": "string",
    "name": "string"
  }
}
```

**エラーレスポンス (4xx, 5xx):**
```json
{
  "message": "string"
}
```

## トラブルシューティング

### 1. API に接続できない

**問題**: `Connection refused` エラー

**解決:**
```bash
# バックエンドが実行中か確認
curl http://api.securenote.test:8000/api/health

# 実行していなければ起動
cd demo-apps
docker-compose -f docker-compose.demo.yml up -d
```

### 2. カスタムドメインに接続できない

**問題**: iOS シミュレーターがカスタムドメインにアクセスできない

**解決:** ホストOSの `/etc/hosts` にカスタムドメインが設定されているか確認してください。macOS側の `/etc/hosts` やDNS設定が、そのままシミュレータ内にも反映されます。

### 3. WebView が空白

**問題**: React Web アプリが表示されない

**解決:**
- `http://securenote.test:8000` で Web が実行中か確認
- ブラウザで直接アクセス: `open http://securenote.test:8000`
- コンソールを確認: Xcode → Debug → View Debugger

## クロスプラットフォーム開発

このプロジェクトは、複数プロジェクトの連携開発を示しています：

```
ai-sandbox-demo/
├── demo-apps/        # API + React Web (Node.js)
│   ├── securenote-api/
│   ├── securenote-web/
│   └── docker-compose.demo.yml
│
└── demo-apps-ios/    # iOS App (Swift)
    ├── SecureNote/
    └── SecureNote.xcodeproj
```

**AI アシスタント（Claude Code）の利点:**

- ✅ すべてのソースコードを見る
- ✅ API のエラーをデバッグ
- ✅ Web フロントエンドとの連携を確認
- ✅ クロスプラットフォーム問題を調査

## 参考リンク

- [SwiftUI ドキュメント](https://developer.apple.com/documentation/swiftui/)
- [WebKit ドキュメント](https://developer.apple.com/documentation/webkit/)
- [URLSession ガイド](https://developer.apple.com/documentation/foundation/urlsession)

## ライセンス

MIT License
