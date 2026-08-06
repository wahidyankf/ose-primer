---
title: How to Run Nx Commands
description: Common Nx workflows and commands for working with the monorepo
category: how-to
tags:
  - nx
  - monorepo
  - commands
  - workflows
---

# How to Run Nx Commands

This guide covers common Nx workflows in `ose-primer`. It is a starter with real CRUD examples,
not a generic application catalogue: discover the projects available in your checkout before
choosing a command.

Run Nx through the workspace dependency, not a global install. Every command below uses
`npm exec nx --` for that reason.

## 📋 Basic Project Commands

> **Standard target names**: All target names follow [Nx Target Standards](../../repo-governance/development/infra/nx-targets.md). Use `test:quick` for the pre-push gate, `test:unit` for isolated unit tests, `dev` for development servers, `start` for production server mode. Avoid `nx test`, `nx serve`, and other non-standard names.

### Run a Single Project

```bash
# Build a specific project
npm exec nx -- build [project-name]

# Run the fast pre-push quality gate
npm exec nx -- run [project-name]:test:quick

# Run isolated unit tests
npm exec nx -- run [project-name]:test:unit

# Lint a specific project
npm exec nx -- lint [project-name]

# Start development server for an app
npm exec nx -- dev [app-name]

# Start production server for an app
npm exec nx -- start [app-name]
```

**Examples**:

```bash
npm exec nx -- build ts-ui                        # Build a shared TypeScript library
npm exec nx -- run ts-ui:test:quick               # Fast quality gate
npm exec nx -- run crud-be-golang-gin:test:quick  # One backend example
npm exec nx -- dev crud-fe-ts-nextjs              # Start a frontend example
npm exec nx -- build crud-fe-ts-nextjs            # Build that frontend
```

### Run Multiple Projects

```bash
# Build all projects
npm exec nx -- run-many -t build

# Run fast quality gate across all projects
npm exec nx -- run-many -t test:quick

# Lint all projects
npm exec nx -- run-many -t lint

# Run multiple targets
npm exec nx -- run-many -t build lint
```

**Using npm scripts**:

```bash
npm run build    # Same as: nx run-many -t build
npm run lint     # Same as: nx run-many -t lint
```

### Run Specific Projects

```bash
# Build specific projects
npm exec nx -- run-many -t build -p ts-ui crud-fe-ts-nextjs

# Run test:quick for specific projects
npm exec nx -- run-many -t test:quick -p ts-ui crud-fe-ts-nextjs
```

## Affected Commands

Affected commands only run tasks for projects that changed since the last commit (or specified base).

### Build Only What Changed

```bash
# Build affected projects (since main branch)
npm exec nx -- affected -t build

# Run fast quality gate for affected projects (pre-push standard)
npm exec nx -- affected -t test:quick

# Lint affected projects
npm exec nx -- affected -t lint

# Specify a different base
npm exec nx -- affected -t build --base=abc123
npm exec nx -- affected -t test:quick --base=origin/main
```

**Using npm scripts**:

```bash
npm run affected:build         # Same as: nx affected -t build
npm run affected:test:quick    # Same as: nx affected -t test:quick
npm run affected:lint          # Same as: nx affected -t lint
```

### Affected Graph

```bash
# View affected projects graph
npm exec nx -- graph --affected

# View affected projects graph (custom base)
npm exec nx -- graph --affected --base=origin/main
```

### Affected Detection in CI/CD

```bash
# In CI pipeline (GitHub Actions example)
npm exec nx -- affected -t build --base=origin/main --head=HEAD
npm exec nx -- affected -t test:quick --base=origin/main --head=HEAD
npm exec nx -- affected -t lint --base=origin/main --head=HEAD
```

## Dependency Graph

### View Full Dependency Graph

```bash
# Open dependency graph in browser
npm exec nx -- graph

# Using npm script
npm run graph
```

This opens an interactive visualization showing:

- All projects (apps and libs)
- Dependencies between projects
- Direction of dependencies

### View Specific Project Dependencies

```bash
# Show dependencies of a specific project
npm exec nx -- graph --focus=ts-ui

# Show what depends on a project
npm exec nx -- graph --focus=ts-ui --groupByFolder
```

### Export Graph

```bash
# Export graph as HTML
npm exec nx -- graph --file=dependency-graph.html

# Export graph as JSON
npm exec nx -- graph --file=dependency-graph.json
```

## Caching

Nx caches task outputs to speed up subsequent runs.

### Cache Behavior

```bash
# First build (executes task)
npm exec nx -- build ts-ui
# Output: Compiled successfully

# Second build (uses cache)
npm exec nx -- build ts-ui
# Output: [existing outputs match the cache, left as is]
```

### Clear Cache

```bash
# Clear all Nx cache
rm -rf .nx/cache

# Or clear specific project cache
npm exec nx -- reset
```

### Disable Cache (Development)

```bash
# Skip cache for a single run
npm exec nx -- build ts-ui --skip-nx-cache

# Skip cache for affected
npm exec nx -- affected -t build --skip-nx-cache
```

## Workspace Commands

### List All Projects

```bash
# List all projects in workspace
npm exec nx -- show projects

# List only apps
npm exec nx -- show projects --type=app

# List only libs
npm exec nx -- show projects --type=lib
```

### Show Project Details

```bash
# Show project configuration
npm exec nx -- show project ts-ui

# Show project graph
npm exec nx -- graph --focus=ts-ui
```

### Workspace Information

```bash
# Show Nx version
npm exec nx -- --version

# Show workspace information
npm exec nx -- report
```

## 🔄 Common Workflows

### Development Workflow

**Starting a new feature**:

```bash
# 1. Pull latest changes
git pull origin main

# 2. Start development server
npm exec nx -- dev crud-fe-ts-nextjs

# 3. Make changes to app or libs

# 4. Test changes
npm exec nx -- run ts-ui:test:quick
npm exec nx -- build crud-fe-ts-nextjs

# 5. View affected projects
npm exec nx -- graph --affected
```

### Testing Workflow

```bash
# 1. Run fast quality gate for changed projects (pre-push standard)
npm exec nx -- affected -t test:quick

# 2. Run test:quick for a specific project
npm exec nx -- run ts-ui:test:quick

# 3. Run isolated unit tests for a specific project
npm exec nx -- run ts-ui:test:unit

# 4. Run all test:quick targets
npm exec nx -- run-many -t test:quick
```

### Build Workflow

```bash
# 1. Build affected projects
npm exec nx -- affected -t build

# 2. Build specific project and its dependencies
npm exec nx -- build crud-fe-ts-nextjs
# (Automatically builds ts-ui first)

# 3. Build all projects
npm exec nx -- run-many -t build

# 4. Verify build outputs
ls libs/ts-ui/dist
ls apps/crud-fe-ts-nextjs/.next
```

### Pre-Commit Workflow

```bash
# 1. Check affected projects
npm exec nx -- graph --affected

# 2. Build affected
npm exec nx -- affected -t build

# 3. Run fast quality gate for affected (same as pre-push hook)
npm exec nx -- affected -t test:quick

# 4. Lint affected
npm exec nx -- affected -t lint

# 5. If all pass, commit changes
git add .
git commit -m "feat: add new feature"
```

## CI/CD Workflows

### GitHub Actions Example

```yaml
name: CI

on: [push, pull_request]

jobs:
 build:
  runs-on: ubuntu-latest
  steps:
   - uses: actions/checkout@v3
    with:
     fetch-depth: 0  # Fetch all history for affected detection

   - name: Setup Node.js
    uses: actions/setup-node@v3
    with:
     node-version: '24.16.0'

   - name: Install dependencies
    run: npm ci

   - name: Build affected
    run: nx affected -t build --base=origin/main --head=HEAD

   - name: Quick Tests (required status check before PR merge)
    run: nx affected -t test:quick --base=origin/main --head=HEAD

   - name: Lint affected
    run: nx affected -t lint --base=origin/main --head=HEAD
```

> **Note**: `test:quick` is the required GitHub Actions status check before PR merge. E2E tests (`test:e2e`) run separately on a scheduled cron job, not on every PR. See [Nx Target Standards](../../repo-governance/development/infra/nx-targets.md) for the full execution model.

### Optimize CI with Caching

```yaml
- name: Cache Nx
 uses: actions/cache@v3
 with:
  path: .nx/cache
  key: nx-${{ runner.os }}-${{ hashFiles('package-lock.json') }}
  restore-keys: |
   nx-${{ runner.os }}-
```

## Performance Tips

### Use Affected Commands in CI

Instead of rebuilding everything:

```bash
# ❌ Slow: Build everything
npm exec nx -- run-many -t build

# ✅ Fast: Build only affected
npm exec nx -- affected -t build

# ✅ Fast quality gate (pre-push and CI)
npm exec nx -- affected -t test:quick
```

### Use Parallel Execution

Nx automatically runs tasks in parallel when possible:

```bash
# Runs builds in parallel (respects dependency order)
npm exec nx -- run-many -t build --parallel=3
```

### Use Watch Mode for Development

```bash
# Watch mode for builds (if configured)
npm exec nx -- build ts-ui --watch
```

## 🔬 Troubleshooting

### Cache Issues

**Problem**: Cached results are stale or incorrect

**Solution**:

```bash
# Clear Nx cache
npm exec nx -- reset

# Rebuild from scratch
npm exec nx -- build ts-ui --skip-nx-cache
```

### Dependency Issues

**Problem**: Changes to library don't trigger app rebuild

**Solution**:

```bash
# Check if dependency exists in graph
npm exec nx -- graph --focus=crud-fe-ts-nextjs

# Ensure library is built first
npm exec nx -- build ts-ui
npm exec nx -- build crud-fe-ts-nextjs
```

### Affected Detection Issues

**Problem**: Affected detection doesn't identify changed projects

**Solution**:

```bash
# Check git status
git status

# Ensure changes are committed or staged
git add .

# Use specific base
npm exec nx -- affected -t build --base=origin/main

# View affected graph to debug
npm exec nx -- graph --affected
```

## Advanced Commands

### Run Commands with Environment Variables

```bash
# Set environment variable for command
NODE_ENV=production npm exec nx -- build crud-fe-ts-nextjs

# Multiple environment variables
NODE_ENV=production DEBUG=true npm exec nx -- build crud-fe-ts-nextjs
```

### Run Custom Commands

```bash
# Run arbitrary command for all projects
npm exec nx -- run-many -t custom-script

# Run command for specific projects
npm exec nx -- run custom-target -p crud-fe-ts-nextjs
```

### Generate Dependency Report

```bash
# Export dependency graph as JSON
npm exec nx -- graph --file=graph.json

# Use jq to analyze dependencies
npm exec nx -- graph --file=graph.json | jq '.dependencies'
```

## 🔗 Related Documentation

- [Nx Target Standards](../../repo-governance/development/infra/nx-targets.md) - Canonical target names, mandatory targets per project type, caching rules, and execution model
- [Add New App](./add-new-app.md)
- [Add New Library](./add-new-lib.md)
- [Monorepo Structure Reference](../reference/monorepo-structure.md)
- [Nx Configuration Reference](../reference/nx-configuration.md)
