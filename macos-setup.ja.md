# macOS 向け 詳細セットアップ手順

[English README is here](macos-setup.md)

[← README に戻る](README.ja.md)

Homebrew・Docker・VS Code のインストールから、AI Sandbox とこのデモアプリが起動するところまでを、ひとつずつ丁寧に説明します。

GitHub アカウントは不要です。すべて Zip ファイルのダウンロードで進められます。

---

## 0. ターミナルを開く

このガイドのコマンドはすべて **ターミナル** アプリに入力します。開き方: `Cmd + Space` を押して `ターミナル`（または `Terminal`）と入力し、Enter を押します（または アプリケーション → ユーティリティ → ターミナル から起動できます）。このガイドを通してずっと使うので、開いたままにしておいてください。

---

## 1. 必要なソフトをインストールする

### 1-1. Homebrew（ホームブルー）をインストール

[Homebrew](https://brew.sh) は、ターミナルからコマンド一つでソフトをインストールできる「パッケージマネージャー」です。

https://brew.sh を開き、案内に従ってインストールしてください。

> [!NOTE]
> Homebrew を使いたくない場合は、このあと出てくる各ソフトをブラウザから個別にダウンロードしてもかまいません。その場合は末尾の [ブラウザからインストールしたい場合](#ブラウザからインストールしたい場合) を参照してください。

### 1-2. wget をインストール（あると便利）

ブラウザを開かずにファイルをダウンロードできるコマンドです。必須ではありませんが、あると後の手順が楽になります。

```bash
brew install wget
```

> [!NOTE]
> インストールしない場合、以降の手順2・3では `wget` を使ってファイルをダウンロードします — 代わりの方法は末尾の [ブラウザからインストールしたい場合](#ブラウザからインストールしたい場合) を参照してください。

### 1-3. Docker 環境をインストール

コンテナを動かすためのソフトです。どちらか一方で構いません。

**OrbStack（おすすめ・軽量）**

```bash
brew install orbstack
```

**Docker Desktop**

```bash
brew install --cask docker
```

どちらかをインストールしたら、一度アプリケーションフォルダ（または Launchpad）から起動し、初回セットアップを完了させてください — `brew install` はアプリをインストールするだけで、起動はしません。メニューバーにアイコンが表示されれば準備完了です。

### 1-4. Visual Studio Code をインストール

```bash
brew install --cask visual-studio-code
```

### 1-5. ターミナルから `code` コマンドで VS Code を開けるようにする

1. VS Code を起動する
2. `Cmd + Shift + P` でコマンドパレットを開く
3. `Shell Command: Install 'code' command in PATH` と入力して選択・実行する
4. 一度 VS Code を閉じる（あとでターミナルから開き直します）

### 1-6. Dev Containers 拡張機能をインストール

```bash
code --install-extension ms-vscode-remote.remote-containers
```

---

## 2. AI Sandbox 本体をダウンロードする

[AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) は、このデモアプリの土台になるテンプレートです。作業用にしたいフォルダに移動してから、以下を実行してください。

```bash
wget https://github.com/YujiSuzuki/ai-sandbox/archive/refs/heads/main.zip
unzip main.zip
rm main.zip
```

`ai-sandbox-main` というフォルダが展開されます。わかりやすい名前に変更しておきましょう。

```bash
mv ai-sandbox-main ai-sandbox-workspace
cd ai-sandbox-workspace
```

中身を確認してみます。

```
$ ls -1aF
./
../
.aiexclude
.cursorrules
.devcontainer/
.env.sandbox.example
.github/
.gitignore
.mcp.json.example
.sandbox/
CLAUDE.md
cli_sandbox/
docs/
GEMINI.md
LICENSE
README.ja.md
README.md
```

上記のようなファイル・フォルダが見えていれば OK です（ドットファイルの細かい内訳はバージョンによって多少前後することがあります）。

---

## 3. デモアプリをダウンロードする

続けて、このデモアプリ（[ai-sandbox-demo](https://github.com/YujiSuzuki/ai-sandbox-demo)）を、さきほど展開した `ai-sandbox-workspace` フォルダの中に配置します。**`ai-sandbox-workspace` フォルダの中にいる状態で**実行してください。

```bash
wget https://github.com/YujiSuzuki/ai-sandbox-demo/archive/refs/heads/main.zip
unzip main.zip
rm main.zip
mv ai-sandbox-demo-main demo-project
```

最終的に、次のような配置になります。

```
ai-sandbox-workspace/
├── .devcontainer/
├── .sandbox/
├── demo-project/          ← 今回配置したデモアプリ
│   ├── demo-apps/
│   └── demo-apps-ios/
├── CLAUDE.md
├── cli_sandbox/
├── docs/
├── README.ja.md
└── README.md
```

---

## 4. （必要な場合）日本語環境やタイムゾーンを設定する

コンテナを開く前に、ホスト OS（お使いの Mac）上で以下を実行してください。これは対話式のプロンプトです — `2` を入力して Enter を押すと、コンテナ内のターミナル出力を日本語にできます。日本語を選んだ場合のみ、続けてタイムゾーンを Asia/Tokyo に設定するかどうかも聞かれます。Enter（または `1` の入力）のままだと英語設定のままとなり、タイムゾーンの質問は表示されません。

```bash
.sandbox/host-setup/init-host-env.sh
```

デフォルトの英語/UTC設定のままで問題なければ、このステップはスキップしてかまいません。

> [!NOTE]
> このスクリプトでは「Install and configure HostMCP now? [y/N]:」と聞かれることもあります。ここは Enter（No）のまま進めてください — HostMCP のインストールは手順5で改めて行います。

---

## 5. HostMCP をインストールして起動する

HostMCP は、AI がこのデモアプリを起動・停止・ビルドしたり、コンテナのログを見たり、テストを実行したりするために使う仕組みです — 必須の手順です。

手順4と同様に、ホストOS（お使いの Mac）上で以下を実行してください:

```bash
.sandbox/host-setup/install-hostmcp.sh
```

インストールが終わると、Finder から起動できる `.command` ファイルが作られます。

```
$ ls -1 hostmcp-*
hostmcp-serve.command
hostmcp-sync.command
```

Finder でこのフォルダを開きます:

```bash
open .
```

開いたら `hostmcp-serve.command` をダブルクリックして起動します。

> [!NOTE]
> AIにホストツール（ホストOS上で実行するスクリプト。このデモのコンテナ起動・停止など）を実行してもらうには、スクリプトの承認が必要です。`hostmcp-sync.command` をダブルクリックすると承認できます — これは他のドキュメントで「ターミナルで `hostmcp tools sync` を実行する」と説明されているのと同じ承認手順です。この `.command` ファイルはそれをターミナルなしでできるようにしたものです。今すぐ実行する必要はありません — AIがホストツールの承認が必要になったタイミング（多くの場合、デモアプリの起動を最初に頼んだとき）で知らせてくれるので、そのときにこのフォルダに戻ってダブルクリックしてください。

---

## 6. VS Code でコンテナを開く

ターミナルに戻り、VS Code を開きます。

```bash
code .
```

VS Code が起動したら、`Cmd+Shift+P`（または `F1`）から「Dev Containers: Reopen in Container」を実行してください。

初回起動時のみ、コンテナ内に SandboxMCP と、ホスト側の HostMCP に接続するための hostmcp CLI をインストールするため時間がかかります。（SandboxMCPは、コンテナ内で動作し、このワークスペースの`.sandbox/scripts`・`.sandbox/tools`以下のスクリプトをAIが見つけて実行できるようにする仕組みです。上記のホストツール（`.sandbox/host-tools/`）は HostMCP が担当します。）

準備ができたら、コンテナ *内* のターミナル（コンテナ内で動く VS Code のターミナル）を開き、Claude Code（`claude` と入力）や Gemini Code Assist などの AI アシスタントを起動してください。以降のドキュメントで「AIに〜と頼む」と書かれているのは、このチャットに直接話しかけることを指します。

その後、[demo-apps/README.ja.md](demo-apps/README.ja.md#クイックスタート) に進んで、実際にデモアプリを起動してみましょう。

---

## トラブルシューティング

**`.command` ファイルをダブルクリックすると「開発元が未確認のため開けません」と表示される**
macOS は未確認の開発元のスクリプトをデフォルトでブロックします。該当ファイルを右クリック（または Control+クリック）して「開く」を選び、表示されるダイアログで確認してください。ファイルごとに最初の1回だけで大丈夫です。

**ターミナルで `code` コマンドが見つからない**
Homebrew と VS Code のインストール後、ターミナルを一度閉じて開き直してください（または `source ~/.zprofile` を実行）。`code` コマンドは、シェルが PATH を再読み込みするまで反映されません。

**`code .` を実行しても VS Code が開かない**
手順 1-5 の「Install 'code' command in PATH」が正しく完了しているか確認し、新しいターミナルウィンドウで再度試してください。

---

## ブラウザからインストールしたい場合

Homebrew や `wget` を使わず、ブラウザだけで進めたい場合は、以下の公式サイトからそれぞれダウンロードしてください。

| ソフト | ダウンロード先 |
|--------|---------------|
| OrbStack | [orbstack.dev/download](https://orbstack.dev/download) |
| Docker Desktop | [docker.com/ja-jp/products/docker-desktop](https://www.docker.com/ja-jp/products/docker-desktop/) |
| Visual Studio Code | [code.visualstudio.com](https://code.visualstudio.com) |
| Dev Containers 拡張機能 | [マーケットプレイスページ](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) を開いて Install を押す |
| AI Sandbox 本体 | [GitHubページ](https://github.com/YujiSuzuki/ai-sandbox) を開き、緑色の `<> Code` ボタン → `Download ZIP` |
| このデモアプリ | [GitHubページ](https://github.com/YujiSuzuki/ai-sandbox-demo) を開き、緑色の `<> Code` ボタン → `Download ZIP` |
