---
title: How to Set Up Your Development Environment
description: Start safely with ose-primer, then add only the toolchains your chosen example needs
category: how-to
tags:
  - onboarding
  - toolchain
  - setup
  - development
  - docker
  - volta
  - sdkman
  - asdf
  - rustup
---

# How to Set Up Your Development Environment

`ose-primer` is a polyglot Nx starter: it gives you one small CRUD product implemented in several
languages and frameworks, plus the automation that keeps those variants honest. Start with the
smallest useful setup, prove the workspace is healthy, then add a language runtime when you choose
an example to explore. You do not need every runtime—or Docker—to read the repository, run the
repository checks, or make your first change.

## 📋 Overview

The monorepo contains projects in 11 languages (TypeScript, Go, Java, Kotlin, Python, Rust,
Elixir, F#, C#, Clojure, Dart). Each language has its own runtime, but they all share the
same Nx build system and git hooks.

**Two setup paths**:

- **Fresh checkout** — Volta-managed Node.js and npm, then `npm install`. This gets the workspace,
  git hooks, and documentation tooling ready.
- **Choose an example** — Add the runtime named in that app's README. Docker is only needed for a
  containerized service, integration test, or E2E flow.
- **Automated check** — Run `npm run doctor -- --fix` to detect and install supported missing
  tools. Use `npm run doctor -- --fix --dry-run` to see the proposed changes first.

## Prerequisites

- **macOS** or **Ubuntu/Debian Linux**.
- **Windows via WSL2 may work**, but it is not a supported or routinely verified path.
- Admin access is useful only when you install optional system packages or Docker.

## 🚀 Quick Start (Minimal Setup)

For the first checkout, this is all you need:

```bash
# 1. Install Volta (macOS or Linux)
curl https://get.volta.sh | bash
source ~/.zshrc

# 2. Clone and bootstrap
git clone https://github.com/wahidyankf/ose-primer.git
cd ose-primer
npm install

# 3. Verify the workspace and install supported optional tooling only when needed
npm run doctor -- --fix
npm exec nx -- run rhino-cli:test:quick
```

The first command that needs a particular example's language runtime tells you what is missing.
Install that runtime using the relevant section below, then rerun the command. Keep Docker and
Playwright for the integration or browser-test work that actually needs them.

## Full Setup

### Step 1: System Package Manager

**macOS**:

```bash
# Install or update Homebrew
brew --version || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew update
```

**Linux (Debian/Ubuntu)**:

```bash
sudo apt-get update
sudo apt-get install -y build-essential curl git
```

### Step 2: Git and Docker

Git is usually pre-installed on macOS (via Xcode Command Line Tools):

```bash
git --version || xcode-select --install
```

Install Docker Desktop from <https://docs.docker.com/desktop/setup/install/mac-install/>
(macOS) or Docker Engine from <https://docs.docker.com/engine/install/> (Linux).

After installation, verify:

```bash
docker --version
docker compose version
docker info   # Confirms daemon is running
```

Install jq (needed for Claude Code hooks and shell scripts):

```bash
# macOS
brew install jq

# Linux
sudo apt-get install -y jq
```

### Step 3: Node.js via Volta

[Volta](https://volta.sh/) pins Node.js and npm versions per-project. The pinned versions
live in `package.json` under `volta.node` and `volta.npm`.

```bash
curl https://get.volta.sh | bash
source ~/.zshrc   # or source ~/.bashrc
```

After installation, entering the repo directory auto-installs the correct versions:

```bash
cd ose-primer
node --version   # Expected: v24.16.0
npm --version    # Expected: 11.10.1
```

If the versions don't match, force install:

```bash
volta install node@24.16.0
volta install npm@11.10.1
```

### Step 4: Go

Required for `rhino-cli`, `rhino-cli`, `rhino-cli`, `crud-be-golang-gin`,
and `libs/golang-commons`.

```bash
# macOS
brew install go

# Linux — download from https://go.dev/dl/
```

Verify the installed version meets or exceeds the `go` directive in `apps/crud-be-golang-gin/go.mod`:

```bash
go version
```

### Step 5: Java + Maven (via SDKMAN)

Required for `crud-be-java-springboot`, `crud-be-java-vertx`, `crud-be-kotlin-ktor`.

[SDKMAN](https://sdkman.io/) manages JDK and Maven versions:

```bash
curl -s "https://get.sdkman.io" | bash
source "$HOME/.sdkman/bin/sdkman-init.sh"

sdk install java 25-tem
sdk install maven

java -version    # Expected: 25+
mvn --version
```

**Kotlin note**: The Kotlin/Ktor project uses a Gradle wrapper (`./gradlew`), so no separate
Kotlin installation is needed — just the JDK.

### Step 6: Clojure

Required for `crud-be-clojure-pedestal`.

```bash
# macOS
brew install clojure/tools/clojure

# Linux — https://clojure.org/guides/install_clojure

clj --version
```

### Step 7: Python

Required for `crud-be-python-fastapi`.

The minimum version is in `apps/crud-be-python-fastapi/.python-version`.

```bash
# Option A: pyenv (recommended — manages multiple Python versions)
brew install pyenv
pyenv install 3.13.5
pyenv global 3.13.5

# Option B: Homebrew
brew install python@3.13

# Linux
sudo apt-get install -y python3 python3-pip python3-venv

python3 --version
```

### Step 8: Rust

Required for `crud-be-rust-axum`.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

rustc --version

# Install coverage tool
cargo install cargo-llvm-cov
cargo llvm-cov --version
```

### Step 9: Erlang + Elixir (via asdf)

Required for `crud-be-elixir-phoenix`.

[asdf](https://asdf-vm.com/) manages Erlang and Elixir versions. The pinned versions are in
`.tool-versions` at the repo root.

```bash
# Install asdf
brew install asdf   # macOS
# Linux: git clone https://github.com/asdf-vm/asdf.git ~/.asdf --branch v0.15.0

# Install Erlang build dependencies (macOS)
brew install autoconf openssl wxwidgets

# Install Erlang
asdf plugin add erlang
asdf install erlang 27.3
asdf global erlang 27.3

# Install Elixir
asdf plugin add elixir
asdf install elixir 1.19.5-otp-27
asdf global elixir 1.19.5-otp-27

elixir --version
```

**Linux build dependencies** for Erlang:

```bash
sudo apt-get install -y build-essential autoconf libncurses-dev libssl-dev
```

### Step 10: .NET SDK

Required for `crud-be-fsharp-giraffe`, `crud-be-csharp-aspnetcore`, `crud-be-fsharp-giraffe`.

The required major version is in `apps/crud-be-fsharp-giraffe/global.json`.

```bash
# macOS
brew install dotnet

# Linux — https://learn.microsoft.com/en-us/dotnet/core/install/linux

dotnet --version
```

### Step 11: Flutter and Dart

Required for `crud-fe-dart-flutterweb`.

Flutter bundles the Dart SDK. The minimum Dart version is in
`apps/crud-fe-dart-flutterweb/pubspec.yaml` under `environment.sdk`.

```bash
# macOS
brew install flutter

# Or manual: https://docs.flutter.dev/get-started/install

flutter config --enable-web
flutter doctor
dart --version
```

### Step 12: Clone and Bootstrap

```bash
git clone https://github.com/wahidyankf/ose-primer.git
cd ose-primer
npm install
```

`npm install` does three things:

1. Installs all npm dependencies
2. Runs `npm run doctor` automatically (postinstall script) to verify your toolchain
3. Sets up Husky git hooks (pre-commit, commit-msg, pre-push)

### Step 14: Configure an App Only When It Needs Configuration

Do not copy, restore, inspect, or commit a real `.env` file as part of general onboarding. When
an app's README says configuration is needed, consult its tracked `.env.example` and keep any real
values only in your local, untracked environment. A fresh checkout does not require inherited
credentials or someone else's configuration backup.

### Step 15: Install Playwright Browsers

```bash
npx playwright install
```

This downloads Chromium, Firefox, and WebKit (~500 MB total). Required for all `*-e2e`
projects.

On Linux, also install system dependencies:

```bash
npx playwright install-deps
```

## ✅ Verification

### Check all tools

```bash
npm run doctor
```

Expected output: all tools show `ok` status. If any show `missing`, revisit the corresponding
step above.

### Test git hooks

**Pre-commit** (runs on every commit — Prettier, markdownlint, lint-staged):

```bash
# Run the same registered gate without creating a throwaway commit
cargo run --release --quiet --manifest-path apps/rhino-cli/Cargo.toml -- gate run --surface=pre-commit
```

**Pre-push** (runs the repository's affected quality gates):

```bash
# Run the fast affected check before a push
npm exec nx -- affected -t test:quick
```

This also warms the Nx cache, making subsequent pushes fast.

### Test integration tests

```bash
# Run one backend's integration suite (uses Docker + PostgreSQL)
npm exec nx -- run crud-be-golang-gin:test:integration
```

If this passes, Docker and database integration work correctly.

### Test E2E

```bash
# Start a backend, then run E2E tests
npm exec nx -- run crud-be-golang-gin:dev &
sleep 5
npm exec nx -- run crud-be-e2e:test:e2e
kill %1
```

## 🔬 Troubleshooting

### Doctor reports a tool as "missing"

The doctor command shows exactly which tool is missing, its expected version, and where the
version requirement comes from (e.g., `package.json → volta.node`). Reinstall the tool using
the matching step above.

### Pre-push hook times out

The pre-push hook runs the registered affected quality gates. On a cold cache, it can take longer
than usual. Warm the focused check first:

```bash
npm exec nx -- affected -t test:quick
```

Subsequent pushes reuse cached results and complete in seconds.

### Volta not switching Node.js version

Ensure Volta's shims are first in your PATH:

```bash
echo $PATH | tr ':' '\n' | head -5
# ~/.volta/bin should appear before /usr/local/bin
```

If not, add to your shell profile:

```bash
export VOLTA_HOME="$HOME/.volta"
export PATH="$VOLTA_HOME/bin:$PATH"
```

### Docker "permission denied" on Linux

Add your user to the docker group:

```bash
sudo usermod -aG docker $USER
# Log out and back in for changes to take effect
```

### Erlang build fails on macOS

Erlang compilation needs OpenSSL headers. If `asdf install erlang` fails:

```bash
brew install openssl
export KERL_CONFIGURE_OPTIONS="--with-ssl=$(brew --prefix openssl)"
asdf install erlang 27.3
```

### Integration test fails with "port already in use"

Another Docker stack or service is using port 5432. Stop it:

```bash
docker compose -f infra/dev/<other-stack>/docker-compose.yml down
# Or find the process:
lsof -i :5432
```

### Playwright "browser not found"

Re-install browsers:

```bash
npx playwright install
```

On Linux, also run:

```bash
npx playwright install-deps
```

## Version Reference

All version requirements are auto-detected by `npm run doctor` from these config files:

| Tool          | Version Source                                          |
| ------------- | ------------------------------------------------------- |
| Node.js       | `package.json` → `volta.node`                           |
| npm           | `package.json` → `volta.npm`                            |
| Java          | `apps/crud-be-java-springboot/pom.xml` → `java.version` |
| Go            | `apps/crud-be-golang-gin/go.mod` → `go` directive       |
| Python        | `apps/crud-be-python-fastapi/.python-version`           |
| Erlang        | `.tool-versions` → `erlang`                             |
| Elixir        | `.tool-versions` → `elixir`                             |
| .NET          | `apps/crud-be-fsharp-giraffe/global.json` → `sdk`       |
| Dart          | `apps/crud-fe-dart-flutterweb/pubspec.yaml` → `sdk`     |
| Rust, Clojure | Any (no pinned version)                                 |
| Docker, jq    | Any (no pinned version)                                 |

Never hardcode version numbers in scripts — always read from these source-of-truth files.

## 🔗 Related Documentation

- [Development Environment Setup Workflow](../../repo-governance/workflows/infra/infra-development-environment-setup.md) —
  Granular workflow with phases and success criteria
- [Local Development with Docker](./local-dev-docker.md) — Running services via
  Docker Compose
- [Reproducible Environments](../../repo-governance/development/workflow/reproducible-environments.md) —
  Volta, npm, Docker reproducibility practices
- [Running CRUD Tests](./run-crud-tests.md) — Integration and E2E test execution
- [Code Quality Convention](../../repo-governance/development/quality/code.md) — Git hooks and
  automated formatting
- [No Secrets in Committed Files Convention](../../repo-governance/conventions/security/no-secrets-in-committed-files.md) —
  Iron rule: real credential values belong only in gitignored `.env*` files, never in committed
  files including plans, docs, and config
