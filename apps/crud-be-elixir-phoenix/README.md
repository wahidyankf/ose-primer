# crud-be-elixir-phoenix

Elixir/Phoenix REST API backend for the Demo Backend platform.
This is an alternative implementation of `crud-be-golang-gin` (Go/Gin), built with
Phoenix 1.7+ on Elixir 1.19 / OTP 27.

## Local Development (Docker)

```bash
# From workspace root — start PostgreSQL + Phoenix server
docker compose -f infra/dev/crud-be-elixir-phoenix/docker-compose.yml up --build
```

The server listens on port **8201** (`http://localhost:8201`).

## Environment Variables

| Variable                            | Required   | Description                          |
| ----------------------------------- | ---------- | ------------------------------------ |
| `DATABASE_URL`                      | Dev / Prod | Ecto connection URL (`ecto://...`)   |
| `CRUD_BE_ELIXIR_PHOENIX_JWT_SECRET` | Dev / Prod | HS256 secret for Guardian JWT tokens |
| `CRUD_BE_ELIXIR_PHOENIX_PORT`       | Optional   | HTTP port (default: `8201`)          |
| `PHX_HOST`                          | Prod only  | Canonical hostname (`example.com`)   |
| `SECRET_KEY_BASE`                   | Prod only  | Phoenix cookie encryption key        |

> `CRUD_BE_ELIXIR_PHOENIX_JWT_SECRET` is **not** required during `mix test` — `config/test.exs` supplies a
> hardcoded test secret so CI can run without injecting secrets.

## Nx Targets

```bash
nx run crud-be-elixir-phoenix:install          # mix deps.get
nx run crud-be-elixir-phoenix:dev              # mix phx.server (development)
nx run crud-be-elixir-phoenix:test:quick       # coveralls.lcov (unit only) + rhino-cli coverage gate (>=90%)
nx run crud-be-elixir-phoenix:test:unit        # mix test --only unit (no coverage measurement)
nx run crud-be-elixir-phoenix:test:integration # docker compose: real PostgreSQL + all BDD scenarios
nx run crud-be-elixir-phoenix:lint             # mix credo --strict
nx run crud-be-elixir-phoenix:typecheck        # mix compile (warnings-as-errors; depends on codegen)
nx run crud-be-elixir-phoenix:build            # mix compile (prod, warnings-as-errors; depends on codegen)
```

`test:quick` and `test:unit` are distinct targets. `test:unit` runs `mix test --only unit` for fast
feedback without coverage overhead. `test:quick` runs `mix coveralls.lcov --only unit` to generate
the LCOV report, then validates coverage with `rhino-cli test-coverage validate` at the ≥90%
threshold — this is the pre-push gate.

`codegen` generates Elixir contract modules from the OpenAPI spec into `generated-contracts/` and
is a dependency of both `typecheck` and `build`.

## API Endpoints

| Method | Path                                           | Auth   | Description                  |
| ------ | ---------------------------------------------- | ------ | ---------------------------- |
| GET    | `/health`                                      | Public | Health check                 |
| GET    | `/.well-known/jwks.json`                       | Public | JWKS public key endpoint     |
| POST   | `/api/v1/auth/register`                        | Public | Register new user            |
| POST   | `/api/v1/auth/login`                           | Public | Login, receive JWT + refresh |
| POST   | `/api/v1/auth/logout`                          | Bearer | Logout current session       |
| POST   | `/api/v1/auth/logout-all`                      | Bearer | Logout all sessions          |
| POST   | `/api/v1/auth/refresh`                         | Public | Refresh access token         |
| GET    | `/api/v1/users/me`                             | Bearer | Get own profile              |
| PATCH  | `/api/v1/users/me`                             | Bearer | Update display name          |
| POST   | `/api/v1/users/me/password`                    | Bearer | Change password              |
| POST   | `/api/v1/users/me/deactivate`                  | Bearer | Self-deactivate account      |
| GET    | `/api/v1/admin/users`                          | Bearer | List users (admin only)      |
| POST   | `/api/v1/admin/users/:id/disable`              | Bearer | Disable user (admin only)    |
| POST   | `/api/v1/admin/users/:id/enable`               | Bearer | Enable user (admin only)     |
| POST   | `/api/v1/admin/users/:id/unlock`               | Bearer | Unlock user (admin only)     |
| POST   | `/api/v1/admin/users/:id/force-password-reset` | Bearer | Force password reset (admin) |
| GET    | `/api/v1/expenses`                             | Bearer | List own entries (paginated) |
| POST   | `/api/v1/expenses`                             | Bearer | Create financial entry       |
| GET    | `/api/v1/expenses/summary`                     | Bearer | Expense totals by currency   |
| GET    | `/api/v1/expenses/:id`                         | Bearer | Get entry by ID              |
| PUT    | `/api/v1/expenses/:id`                         | Bearer | Update entry                 |
| DELETE | `/api/v1/expenses/:id`                         | Bearer | Delete entry                 |
| GET    | `/api/v1/expenses/:id/attachments`             | Bearer | List attachments             |
| POST   | `/api/v1/expenses/:id/attachments`             | Bearer | Upload attachment            |
| GET    | `/api/v1/expenses/:id/attachments/:att_id`     | Bearer | Download attachment metadata |
| DELETE | `/api/v1/expenses/:id/attachments/:att_id`     | Bearer | Delete attachment            |
| GET    | `/api/v1/reports/pl`                           | Bearer | P&L report for date range    |

## Three-Level Test Architecture

This application follows the standard three-level testing strategy:

```
unit        → fast, in-memory, no external services, fully cached
integration → Docker Compose + real PostgreSQL, not cached
e2e         → Playwright against a live running stack (apps/crud-be-e2e)
```

### Level 1: Unit Tests (`test:unit` / `test:quick`)

Unit tests run with `MIX_ENV=test` using in-memory context implementations. No database
or external services are required. These tests are **fully cached** by Nx.

```bash
# Fast feedback — run unit tests without coverage overhead
nx run crud-be-elixir-phoenix:test:unit

# Pre-push quality gate — run unit tests with coverage + enforce >=90%
nx run crud-be-elixir-phoenix:test:quick
```

`test:unit` runs `mix test --only unit`. `test:quick` runs `mix coveralls.lcov --only unit` to
produce the LCOV report, then validates coverage with `rhino-cli test-coverage validate`.

**What runs:**

- All shared Gherkin BDD scenarios re-implemented in `test/unit/steps/` with `@moduletag :unit`
- Controller error-path tests in `test/crud_be_exph_web/controllers/coverage_test.exs`
- (`test:quick` only) ExCoveralls LCOV report generated to `cover/lcov.info`
- (`test:quick` only) `rhino-cli test-coverage validate` enforces ≥90% line coverage

**Mock architecture:**

All context modules are replaced at test time via `config/test.exs`:

- `CrudBeExph.Accounts` → `CrudBeExph.InMemoryAccounts`
- `CrudBeExph.Token.TokenContext` → `CrudBeExph.InMemoryTokenContext`
- `CrudBeExph.Expense.ExpenseContext` → `CrudBeExph.InMemoryExpenseContext`
- `CrudBeExph.Attachment.AttachmentContext` → `CrudBeExph.InMemoryAttachmentContext`

The `Repo` GenServer is not started in the `:test` environment. Tests are fully
deterministic with no external service dependencies, making them safe for Nx caching.

### Level 2: Integration Tests (`test:integration`)

Integration tests run the same shared Gherkin BDD scenarios (`test/integration/steps/`) against
a real PostgreSQL 17 database via Docker Compose. These tests are **never cached**.

```bash
nx run crud-be-elixir-phoenix:test:integration
```

**What runs:**

- `docker-compose.integration.yml` spins up `postgres:17-alpine` + an `elixir:1.17-otp-27-alpine` test runner
- Migrations run with `MIX_ENV=integration mix ecto.create && mix ecto.migrate`
- All integration step files tagged `@moduletag :integration` execute against the real Ecto repo
- Ecto SQL Sandbox (`:manual` mode) provides test isolation

**Prerequisites:** Docker with Compose plugin must be installed.

### Level 3: E2E Tests

End-to-end tests live in `apps/crud-be-e2e` and run Playwright scenarios against a fully
deployed stack. See that project's README for details.

## BDD Feature Specifications

Feature specifications live in `specs/apps/crud/behavior/crud-be/gherkin/` (workspace root) and are shared
across all demo backend implementations. The scenarios cover:

- Health check
- User registration
- Password login
- Token lifecycle
- Token management
- User account management
- Security (lockout, brute-force)
- Admin operations
- Expense management
- Currency handling
- Unit handling
- Financial reporting (P&L)
- Attachments

Both `test/unit/steps/` and `test/integration/steps/` contain step definitions for all
All scenarios are shared — the unit steps use in-memory stores, the integration steps use the real Ecto repo.

## Related Documentation

- [Three-Level Testing Standard](../../repo-governance/development/quality/three-level-testing-standard.md) — Unit, integration, and E2E testing boundaries
- [Code Coverage Reference](../../docs/reference/code-coverage.md) — Coverage tools and thresholds
- [Project Dependency Graph](../../docs/reference/project-dependency-graph.md) — Nx dependency visualization
- [Backend Gherkin Specs](../../specs/apps/crud/behavior/crud-be/gherkin/README.md) — Shared feature files (source of truth)
- [OpenAPI Contract](../../specs/apps/crud/containers/contracts/README.md) — API contract and codegen
