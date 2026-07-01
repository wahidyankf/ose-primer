# Product Overview: Demo CRUD Application

Polyglot demo application that proves a single OpenAPI 3.1 contract and shared Gherkin
specifications can drive fully interchangeable backend and frontend implementations across
11 languages and frameworks.

## Goals

- Provide a working, spec-first reference for each polyglot backend/frontend tier
- Demonstrate that contract-first development with `crud-contracts` eliminates per-language
  integration drift
- Serve as the primary test bed for `rhino-cli` validators (`specs:behavior:coverage`,
  `specs:domain:coverage`, `specs:structure:validation`)

## Scope

| Dimension      | In scope                                                                                    |
| -------------- | ------------------------------------------------------------------------------------------- |
| Backends (BE)  | Go, Java/Spring Boot, Java/Vert.x, Kotlin, Python, Rust, TS/Effect, F#, C#, Clojure, Elixir |
| Frontends (FE) | TypeScript/Next.js, TypeScript/TanStack Start, Dart/Flutter Web                             |
| Features       | Auth, token management, user lifecycle, expenses, admin, health                             |

## Actors

- **End User** — registers, logs in, manages profile and expense entries, views P&L
- **Administrator** — manages users (list, status, password reset)
- **Operations Engineer** — monitors service health via liveness endpoint
- **Service Integrator** — verifies JWTs via the JWKS public-key endpoint

## Related

- **System context**: [context.md](../system-context/context.md)
- **Container diagram**: [container.md](../containers/container.md)
- **Backend component diagram**: [component-be.md](../components/be/component-be.md)
- **Frontend component diagram**: [component-web.md](../components/web/component-web.md)
- **DDD registry**: [bounded-contexts.yaml](../ddd/bounded-contexts.yaml)
- **Parent**: [crud specs](../README.md)
