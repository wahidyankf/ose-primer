# CRUD Backend Dev Stack — EXPH (Elixir/Phoenix)

Local development environment for `crud-be-elixir-phoenix`, the Elixir/Phoenix
alternative backend for the shared CRUD product. Runs on the same port (8201) as the
Go/Gin backend (`crud-be-golang-gin`) — the two stacks are mutually
exclusive and **must not be started simultaneously**.

## Port Assignment

| Service                   | Port |
| ------------------------- | ---- |
| crud-be-elixir-phoenix-db | 5432 |
| crud-be-elixir-phoenix    | 8201 |

## Quick Start

```bash
# From workspace root
cd infra/dev/crud-be-elixir-phoenix

# First run — build image and start services
docker compose up --build

# Subsequent runs (image cached)
docker compose up
```

The `crud-be-elixir-phoenix` container automatically runs `mix ecto.migrate`
before starting Phoenix, so the schema is always up to date.

## Environment Configuration

This stack reads PostgreSQL and JWT configuration from its Compose environment. Start with the
tracked configuration guidance for this stack; keep any real credentials in your local, untracked
environment and never place them in this README or a committed `.env` file.

## Manual Smoke Test

```bash
# Health check
curl http://localhost:8201/health
# Expected: {"status":"UP"}

# Register a user
curl -X POST http://localhost:8201/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com","password":"<test-password>"}'
# Expected: {"id":1,"username":"alice","email":"alice@example.com"}

# Login
curl -X POST http://localhost:8201/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","password":"<test-password>"}'
# Expected: {"access_token":"<jwt>","refresh_token":"<refresh>","token_type":"Bearer"}

# Get profile (replace <jwt> with access_token from login)
curl http://localhost:8201/api/v1/users/me \
  -H "Authorization: Bearer <jwt>"
# Expected: {"id":1,"username":"alice","email":"alice@example.com",...}
```

## Shared Database Note

All crud-be backends use PostgreSQL on port 5432 but cannot run simultaneously since all
bind port 8201. The databases have different
names (`crud_be_elixir_phoenix` for this backend, `crud_be` for golang-gin/java-springboot, etc.) so they could share a PostgreSQL
instance with custom setup, but the default stacks are mutually exclusive.

## E2E Tests

```bash
# Start the stack in CI mode (docker-compose.ci.yml merges on top of docker-compose.yml)
docker compose -f docker-compose.yml -f docker-compose.ci.yml up --build -d

# Run E2E tests from workspace root
BASE_URL=http://localhost:8201 npm exec nx -- run crud-be-e2e:test:e2e

# Stop stack
docker compose -f docker-compose.yml -f docker-compose.ci.yml down
```

## Volume Mounts for Local Dependencies

`crud-be-elixir-phoenix` declares `elixir-gherkin` and `elixir-cabbage` as local
Mix path dependencies. Inside the container, Mix resolves these relative to
`/workspace`:

- `../../libs/elixir-gherkin` → `/libs/elixir-gherkin` (bind-mounted read-only)
- `../../libs/elixir-cabbage` → `/libs/elixir-cabbage` (bind-mounted read-only)

Both must be mounted for `mix deps.get` and compilation to succeed.
