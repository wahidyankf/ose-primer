# CRUD Backend Dev Stack — CSAS (C#/ASP.NET Core)

Local development environment for `crud-be-csharp-aspnetcore`, the C#/ASP.NET Core
alternative backend for the shared CRUD product. Runs on the same port (8201) as the
Go/Gin backend (`crud-be-golang-gin`) and other alternative implementations — the
stacks are mutually exclusive and **must not be started simultaneously**.

## Port Assignment

| Service                      | Port |
| ---------------------------- | ---- |
| crud-be-csharp-aspnetcore-db | 5432 |
| crud-be-csharp-aspnetcore    | 8201 |

## Quick Start

```bash
# From workspace root
cd infra/dev/crud-be-csharp-aspnetcore

# First run — build image and start services
docker compose up --build

# Subsequent runs (image cached)
docker compose up
```

EF Core auto-migrates the database on startup via `EnsureCreatedAsync`, so the schema
is always up to date.

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
# Expected: {"id":"<uuid>","username":"alice","email":"alice@example.com"}

# Login
curl -X POST http://localhost:8201/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","password":"<test-password>"}'
# Expected: {"access_token":"<jwt>","refresh_token":"<refresh>","token_type":"Bearer"}

# Get profile (replace <jwt> with access_token from login)
curl http://localhost:8201/api/v1/users/me \
  -H "Authorization: Bearer <jwt>"
# Expected: {"id":"<uuid>","username":"alice","email":"alice@example.com",...}
```

## E2E Tests

```bash
# Start the stack in CI mode (docker-compose.ci.yml merges on top of docker-compose.yml)
docker compose -f docker-compose.yml -f docker-compose.ci.yml up --build -d

# Run E2E tests from workspace root
BASE_URL=http://localhost:8201 npm exec nx -- run crud-be-e2e:test:e2e

# Stop stack
docker compose -f docker-compose.yml -f docker-compose.ci.yml down
```
