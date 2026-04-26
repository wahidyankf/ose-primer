# Apps Folder

## Purpose

The `apps/` directory contains **deployable application projects** (executables). These are the final artifacts that can be run, deployed, and served to end users.

## Naming Convention

Apps follow the naming pattern: **`{domain}-{part}`**

Where `{part}` describes the role and technology stack:

| Part pattern            | Examples                                              | Description                              |
| ----------------------- | ----------------------------------------------------- | ---------------------------------------- |
| `be-{lang}-{framework}` | `be-golang-gin`, `be-java-springboot`, `be-ts-effect` | Backend service                          |
| `fe-{lang}-{framework}` | `fe-ts-nextjs`, `fe-dart-flutterweb`                  | Frontend application                     |
| `fs-{lang}-{framework}` | `fs-ts-nextjs`                                        | Fullstack application (FE + BE combined) |
| `cli`                   | `rhino-cli`, `rhino-cli`, `rhino-cli`                 | CLI tool                                 |
| `web`                   | `crud-fs-ts-nextjs`, `crud-fs-ts-nextjs`              | Web platform (content site)              |
| `{role}-e2e`            | `be-e2e`, `fe-e2e`, `crud-fe-e2e`                     | E2E test project for the named role      |
| `be` / `fe`             | `crud-be-fsharp-giraffe`, `crud-fe-ts-nextjs`         | Simple single-technology projects        |

**Language abbreviations** (`{lang}`): `ts` (TypeScript), `golang` (Go), `java` (Java), `kt` (Kotlin),
`py` (Python), `rs` (Rust), `cs` (C#), `fs` (F#), `clj` (Clojure), `dart` (Dart), `ex` (Elixir).

**Framework abbreviations** (`{framework}`): `nextjs`, `gin`, `springboot`, `ktor`, `fastapi`, `axum`,
`aspnetcore`, `giraffe`, `pedestal`, `phoenix`, `vertx`, `effect`, `tanstack-start`, `flutterweb`.

### Current Apps

- `crud-fs-ts-nextjs` - demo website ([example.com](https://example.com)) - Hugo static site
- `crud-fs-ts-nextjs` - demo educational platform ([example.com](https://example.com)) - Next.js 16 fullstack content platform (TypeScript, tRPC)
- `crud-be-e2e` - Playwright BE E2E tests for crud-fs-ts-nextjs tRPC API
- `crud-fe-e2e` - Playwright FE E2E tests for crud-fs-ts-nextjs UI
- `rhino-cli` - demo CLI tool for link validation - Go application
- `rhino-cli` - Repository management CLI tools (includes `java validate-annotations`) - Go application
- `rhino-cli` - demo CLI tool for link validation - Go application
- `crud-fe-ts-nextjs` - demo landing website (www.example.com) - Next.js app (port 3200)
- `crud-be-fsharp-giraffe` - demo backend API (F#/Giraffe) - F# application (port 8202)
- `crud-fe-e2e` - FE E2E tests for crud-fe-ts-nextjs - Playwright (browser testing)
- `crud-be-e2e` - BE E2E tests for crud-be-fsharp-giraffe - Playwright (API testing)
- `crud-be-golang-gin` - demo backend API (Go/Gin) - Go application (port 8201)
- `crud-be-e2e` - E2E tests for demo-be REST API - Playwright (API testing)

## Application Characteristics

- **Consumers** - Apps import and use libs, but don't export anything for reuse
- **Isolated** - Apps should NOT import from other apps
- **Deployable** - Each app is independently deployable
- **Specific** - Contains app-specific logic and configuration
- **Entry Points** - Has clear entry points (index.ts, main.ts, etc.)

## App Structure Examples

### Hugo Static Site (crud-fs-ts-nextjs)

```
├── content/                 # Markdown content files
├── layouts/                 # Hugo templates
├── static/                  # Static assets (images, CSS, JS)
├── themes/                  # Hugo themes
├── public/                  # Build output (gitignored)
├── hugo.yaml                # Hugo configuration
├── project.json             # Nx project configuration
├── build.sh                 # Build script
├── vercel.json              # Deployment configuration
└── README.md                # App documentation
```

### Go CLI Application (Current)

```
├── cmd/                     # CLI commands
├── internal/                # Internal packages
├── dist/                    # Build output (gitignored)
├── main.go                  # Entry point
├── go.mod                   # Go module definition
├── project.json             # Nx project configuration
└── README.md                # App documentation
```

```
apps/rhino-cli/
├── cmd/                     # CLI commands
├── internal/                # Internal packages
├── dist/                    # Build output (gitignored)
├── main.go                  # Entry point
├── go.mod                   # Go module definition
├── project.json             # Nx project configuration
└── README.md                # App documentation
```

```
├── internal/                # Internal packages (links/)
├── cmd/                     # CLI commands
├── dist/                    # Build output (gitignored)
├── main.go                  # Entry point
├── go.mod                   # Go module definition
├── project.json             # Nx project configuration
└── README.md                # App documentation
```

### Go/Gin Application (Current Default)

```
apps/crud-be-golang-gin/
├── cmd/server/              # Main entry point
│   └── main.go
├── internal/                # Internal packages
│   ├── config/              # Configuration (env vars)
│   ├── handler/             # HTTP handlers
│   ├── router/              # Gin router setup
│   ├── server/              # Server startup
│   └── store/               # Data access layer
├── go.mod                   # Go module definition
├── go.sum                   # Dependency checksums
├── Dockerfile               # Production Docker image
├── project.json             # Nx configuration
└── README.md                # App documentation
```

### Playwright E2E Test App (Current)

```
apps/crud-be-e2e/
├── playwright.config.ts         # Playwright configuration (baseURL, reporters)
├── package.json                 # Pinned @playwright/test dependency
├── tsconfig.json                # TypeScript config (extends workspace base)
├── project.json                 # Nx configuration
├── tests/
│   ├── e2e/
│   │   ├── hello/
│   │   │   └── hello.spec.ts    # Tests for GET /api/v1/hello
│   │   └── actuator/
│   │       └── health.spec.ts   # Tests for GET /actuator/health
│   └── utils/
│       └── api-helpers.ts       # Shared request utilities
└── README.md                    # App documentation
```

### Next.js Application (Current)

```
├── src/
│   ├── app/                    # Next.js App Router pages
│   │   ├── dashboard/          # Dashboard route
│   │   ├── login/              # Login route
│   │   ├── api/                # API route handlers
│   │   ├── layout.tsx          # Root layout
│   │   └── page.tsx            # Root page
│   ├── components/             # Reusable React components
│   │   └── ui/                 # shadcn-ui component library
│   ├── contexts/               # Shared React contexts
│   ├── data/                   # JSON data files
│   └── lib/                    # Utility functions and helpers
├── public/                     # Static assets
├── components.json             # shadcn-ui configuration
├── next.config.mjs             # Next.js configuration
├── tailwind.config.ts          # TailwindCSS configuration
├── tsconfig.json               # TypeScript configuration
├── vercel.json                 # Vercel deployment configuration
├── project.json                # Nx project configuration
└── README.md                   # App documentation
```

### Future App Types

Kotlin, Python apps will have language-specific structures and tooling.

## Nx Configuration (project.json)

Each app must have a `project.json` file with Nx configuration.

**Hugo App Example** (`crud-fs-ts-nextjs`):

```json
{
  "name": "crud-fs-ts-nextjs",
  "projectType": "application",
  "targets": {
    "dev": {
      "executor": "nx:run-commands",
      "options": {
        "command": "hugo server --buildDrafts --buildFuture"
      }
    },
    "build": {
      "executor": "nx:run-commands",
      "options": {
        "command": "bash build.sh"
      },
      "outputs": ["{projectRoot}/public"]
    },
    "clean": {
      "executor": "nx:run-commands",
      "options": {
        "command": "rm -rf public resources"
      }
    }
  },
  "tags": ["type:app", "platform:nextjs", "lang:ts", "domain:demo"]
}
```

**Note**: This repository uses vanilla Nx (no plugins), so all executors use `nx:run-commands` to run standard build tools directly (Hugo, Go, etc.).

## How to Add a New App

See the how-to guide: `docs/how-to/add-new-app.md` (to be created)

## Importing from Libraries

Apps can import from any library in `libs/` using path mappings:

```typescript
// Future TypeScript apps will use path mappings like:
import { utils } from "@open-sharia-enterprise/ts-utils";
import { Button } from "@open-sharia-enterprise/ts-components";
```

Path mappings are configured in the workspace `tsconfig.base.json` file.

**Note**: Currently there are no libraries in `libs/`. Libraries will be created as shared functionality is identified.

## Running Apps

Use Nx commands to run apps:

```bash
# Development mode (Hugo site)
nx dev crud-fs-ts-nextjs

# Development mode (Next.js)
nx dev crud-fe-ts-nextjs
nx dev crud-fs-ts-nextjs

# Build for production
nx build crud-fs-ts-nextjs
nx build crud-fs-ts-nextjs
nx build rhino-cli
nx build rhino-cli
nx build crud-fe-ts-nextjs

# Run CLI applications
nx run rhino-cli

# Clean build artifacts
nx clean crud-fs-ts-nextjs

# Run E2E tests for crud-fe-ts-nextjs (crud-fe-ts-nextjs must be running first)
nx run crud-fe-e2e:test:e2e

# Run API E2E tests (backend must be running first)
nx run crud-be-e2e:test:e2e
```

## Deployment Branches

Vercel-deployed apps use dedicated production branches (deployment-only — never commit directly):

| Branch                   | Production URL                              | App               |
| ------------------------ | ------------------------------------------- | ----------------- |
| `prod-crud-fs-ts-nextjs` | [example.com](https://example.com)          | crud-fs-ts-nextjs |
| `prod-crud-fs-ts-nextjs` | [example.com](https://example.com)          | crud-fs-ts-nextjs |
| `prod-demo-web`          | [www.example.com](https://www.example.com/) | crud-fe-ts-nextjs |

**crud-fs-ts-nextjs**: Deploy by force-pushing `main` to the production branch:

```bash
git push origin main:prod-crud-fs-ts-nextjs --force
```

**crud-fs-ts-nextjs**: Deployed automatically by scheduled GitHub Actions
workflow (`test-and-deploy-crud-fs-ts-nextjs.yml`) running at 6 AM and 6 PM
WIB. The workflow detects changes scoped to the app directory before building and deploying.
Trigger on-demand from the GitHub Actions UI (set `force_deploy=true` to skip change detection).

**crud-fe-ts-nextjs**: Deploy by force-pushing `main` to the production branch:

```bash
git push origin main:prod-demo-web --force
```

Use the corresponding deployer agent (e.g. `apps-crud-fe-ts-nextjs-deployer`) for guided deployment.

## Language Support

Currently:

- **Hugo** (static sites) - crud-fs-ts-nextjs
- **Go** (CLI tools) - rhino-cli, rhino-cli
- **TypeScript/Next.js** (web applications) - crud-fe-ts-nextjs, crud-fs-ts-nextjs
- **F#/Giraffe** (backend API) - crud-be-fsharp-giraffe
- **Go/Gin** (backend API) - crud-be-golang-gin
- **TypeScript/Playwright** (E2E testing) - crud-be-e2e, crud-fe-e2e, crud-be-e2e

Future: Kotlin, Python apps (each language will have language-specific structure and tooling)
