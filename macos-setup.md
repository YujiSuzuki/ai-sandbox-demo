# Detailed Setup Guide for macOS

[日本語版はこちら](macos-setup.ja.md)

[← Back to README](README.md)

This guide walks through everything step by step, from installing Homebrew, Docker, and VS Code, to getting AI Sandbox and this demo app running.

No GitHub account is needed. Everything is done via Zip file downloads.

---

## 0. Open Terminal

Every command in this guide is typed into the **Terminal** app. To open it: press `Cmd + Space`, type `Terminal`, and press Enter (or find it under Applications → Utilities → Terminal). Keep it open — you'll come back to it throughout this guide.

---

## 1. Install the required software

### 1-1. Install Homebrew

[Homebrew](https://brew.sh) is a "package manager" that lets you install software with a single terminal command.

Open https://brew.sh and follow the instructions there.

> [!NOTE]
> If you'd rather not use Homebrew, you can download each piece of software from its website instead — see [If you'd rather install via browser](#if-youd-rather-install-via-browser) at the end of this guide.

### 1-2. Install wget (optional, but handy)

Lets you download files without opening a browser. Not required, but it makes the later steps smoother.

```bash
brew install wget
```

> [!NOTE]
> Skipping this? Steps 2 and 3 below use `wget` to download files — see [If you'd rather install via browser](#if-youd-rather-install-via-browser) at the end of this guide for the alternative.

### 1-3. Install a Docker runtime

Needed to run containers. Pick either one.

**OrbStack (recommended — lightweight)**

```bash
brew install orbstack
```

**Docker Desktop**

```bash
brew install --cask docker
```

After installing either one, launch it once from Applications and complete its first-time setup — `brew install` only installs the app, it doesn't start it. You'll know it's ready when its icon appears in the menu bar.

### 1-4. Install Visual Studio Code

```bash
brew install --cask visual-studio-code
```

### 1-5. Enable the `code` command in your terminal

1. Launch VS Code
2. Open the command palette with `Cmd + Shift + P`
3. Type and select `Shell Command: Install 'code' command in PATH`
4. Close VS Code for now (you'll reopen it from the terminal later)

### 1-6. Install the Dev Containers extension

```bash
code --install-extension ms-vscode-remote.remote-containers
```

---

## 2. Download AI Sandbox

[AI Sandbox](https://github.com/YujiSuzuki/ai-sandbox) is the template this demo app runs on top of. Move into whichever folder you want to work from, then run:

```bash
wget https://github.com/YujiSuzuki/ai-sandbox/archive/refs/heads/main.zip
unzip main.zip
rm main.zip
```

This extracts a folder named `ai-sandbox-main`. Let's rename it to something clearer.

```bash
mv ai-sandbox-main ai-sandbox-workspace
cd ai-sandbox-workspace
```

Check the contents:

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

If you see files and folders like the above (a few dotfiles' exact names may differ slightly between versions), you're good.

---

## 3. Download this demo app

Next, place this demo app ([ai-sandbox-demo](https://github.com/YujiSuzuki/ai-sandbox-demo)) inside the `ai-sandbox-workspace` folder you just extracted. Make sure you're **inside the `ai-sandbox-workspace` folder** when you run this.

```bash
wget https://github.com/YujiSuzuki/ai-sandbox-demo/archive/refs/heads/main.zip
unzip main.zip
rm main.zip
mv ai-sandbox-demo-main demo-project
```

You should end up with this layout:

```
ai-sandbox-workspace/
├── .devcontainer/
├── .sandbox/
├── demo-project/          ← the demo app you just placed here
│   ├── demo-apps/
│   └── demo-apps-ios/
├── CLAUDE.md
├── cli_sandbox/
├── docs/
├── README.ja.md
└── README.md
```

---

## 4. (Optional) Set the language/timezone for the container

Before opening the container, run this on the host OS (your Mac). It's an interactive prompt: type `2` and press Enter to make the container's terminal output Japanese instead of English — only then will it also ask whether to set the timezone to Asia/Tokyo. Pressing Enter (or typing `1`) keeps everything in English and skips the timezone question entirely.

```bash
.sandbox/host-setup/init-host-env.sh
```

If you're comfortable with the default English/UTC setup, you can skip this step.

> [!NOTE]
> The script may also ask "Install and configure HostMCP now? [y/N]:". Press Enter (No) here — step 5 below walks through installing HostMCP properly.

---

## 5. Install and start HostMCP

HostMCP is what lets the AI start, stop, and build this demo app, and check container logs and run tests for it — it's required, not optional.

Run this on the host OS (your Mac), same as step 4:

```bash
.sandbox/host-setup/install-hostmcp.sh
```

Once installed, you'll get `.command` files you can launch from Finder.

```
$ ls -1 hostmcp-*
hostmcp-serve.command
hostmcp-sync.command
```

Open a Finder window in this folder:

```bash
open .
```

Then double-click `hostmcp-serve.command` to start it.

> [!NOTE]
> Before the AI can run host tools (scripts that execute on the host OS, like starting/stopping this demo's containers), those scripts need to be approved. Double-click `hostmcp-sync.command` to approve them — this is the same approval step other docs in this repo refer to as running `hostmcp tools sync` in a terminal; the `.command` file just does it for you without needing the terminal. You don't need to do this now — the AI will tell you when a host tool needs approval (usually the first time you ask it to start the demo app), and you can come back to this folder and double-click it again at that point.

---

## 6. Open the container in VS Code

Back in the terminal, open VS Code:

```bash
code .
```

Once VS Code launches, run "Dev Containers: Reopen in Container" via `Cmd+Shift+P` (or `F1`).

The first launch takes a few minutes, since it installs SandboxMCP and the `hostmcp` CLI client (which connects to the HostMCP server you started on the host OS in Step 5) inside the container. (SandboxMCP is a companion tool that runs inside the container and lets the AI discover and run the scripts under this workspace's `.sandbox/scripts` and `.sandbox/tools` folders. The host tools mentioned above, under `.sandbox/host-tools/`, are handled by HostMCP instead.)

Once it's ready, open a terminal *inside* the container (VS Code's own terminal, now running inside the Dev Container) and launch an AI coding assistant — for example Claude Code (run `claude`) or Gemini Code Assist. This is "the AI" the rest of these docs mean whenever they say "ask the AI to...": type your request directly into that chat.

Then head to [demo-apps/README.md](demo-apps/README.md#quick-start) to actually start and try the demo app.

---

## Troubleshooting

**"Apple could not verify..." warning when double-clicking a `.command` file**
macOS blocks scripts from unidentified developers by default. Right-click (or Control-click) the file and choose **Open**, then confirm in the dialog that appears. You only need to do this once per file.

**`code` command not found in the terminal**
Close and reopen your terminal window (or run `source ~/.zprofile`) after installing Homebrew and VS Code — the `code` command isn't picked up until the shell reloads its PATH.

**`code .` doesn't open VS Code**
Re-run step 1-5 to make sure the "Install 'code' command in PATH" step actually completed, then try again from a new terminal window.

---

## If you'd rather install via browser

If you'd rather not use Homebrew or `wget`, you can download everything from these official sites instead:

| Software | Download |
|----------|----------|
| OrbStack | [orbstack.dev/download](https://orbstack.dev/download) |
| Docker Desktop | [docker.com/products/docker-desktop](https://www.docker.com/products/docker-desktop/) |
| Visual Studio Code | [code.visualstudio.com](https://code.visualstudio.com) |
| Dev Containers extension | Open the [marketplace page](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers) and click Install |
| AI Sandbox | Open the [GitHub page](https://github.com/YujiSuzuki/ai-sandbox), click the green `<> Code` button → `Download ZIP` |
| This demo app | Open the [GitHub page](https://github.com/YujiSuzuki/ai-sandbox-demo), click the green `<> Code` button → `Download ZIP` |
