# Demo Application Specs

Platform-agnostic specifications for a crud-scale full-stack application covering authentication,
user management, and a multi-currency expense domain. The application consists of a backend REST API
and a frontend single-page application (SPA).

## Structure

```
specs/apps/crud/
├── README.md
├── product/                       # PM-first product documentation (placeholder)
├── system-context/
│   ├── README.md
│   └── context.md                 # L1 — system context (4 actors)
├── containers/
│   ├── README.md
│   ├── container.md               # L2 — containers (SPA, Static Server, API, DB, FS)
│   └── contracts/                 # OpenAPI 3.1 API contract
├── components/
│   ├── README.md
│   ├── be/
│   │   └── component-be.md        # L3 — REST API internals
│   └── web/
│       └── component-web.md       # L3 — SPA internals
└── behavior/
    ├── README.md
    ├── be/                        # Backend specs (HTTP-semantic)
    │   └── gherkin/               # Backend Gherkin scenarios (see be/gherkin/README)
    └── web/                       # Web frontend specs (UI-semantic)
        └── gherkin/               # Web Gherkin scenarios (see web/gherkin/README)
```

## Backend vs Web Frontend

| Aspect      | Backend (behavior/be/)                                  | Web Frontend (behavior/web/)                              |
| ----------- | ------------------------------------------------------- | --------------------------------------------------------- |
| Perspective | HTTP-semantic (GET, POST, status codes)                 | UI-semantic (clicks, types, sees)                         |
| Background  | `Given the API is running`                              | `Given the app is running`                                |
| Scenarios   | See [be/gherkin/](./behavior/crud-be/gherkin/README.md) | See [web/gherkin/](./behavior/crud-web/gherkin/README.md) |
| Domains     | 7 domains                                               | 8 domains (7 shared + layout)                             |
| Consumed by | `apps/crud-be-{lang}-{framework}/` (11 backends)        | `apps/crud-fe-{lang}-{framework}/` (5 frontends)          |

Both spec sets cover the same functional surface from different perspectives. The frontend app
consumes the backend API.

Fullstack apps (`apps/crud-fs-{lang}-{framework}/`) consume **both** BE and web Gherkin specs
since they combine backend and frontend in a single deployable unit.

## Shared Domains

| Domain           | BE Features | Web Features | Description                         |
| ---------------- | ----------- | ------------ | ----------------------------------- |
| health           | 1           | 1            | Service liveness/health status      |
| authentication   | 2           | 2            | Login, token refresh, logout        |
| user-lifecycle   | 2           | 2            | Registration, profile, deactivation |
| security         | 1           | 1            | Password policy, lockout, unlock    |
| token-management | 1           | 1            | JWT claims, JWKS, revocation        |
| admin            | 1           | 1            | User management panel               |
| expenses         | 5           | 5            | CRUD, currency, units, P&L, uploads |
| layout           | —           | 2            | Responsive design, accessibility    |

## Spec Artifacts

- **[system-context/](./system-context/README.md)** — C4 Level 1 system context diagram
- **[containers/](./containers/README.md)** — C4 Level 2 container diagram + OpenAPI contracts
- **[components/](./components/README.md)** — C4 Level 3 component diagrams (be, web)
- **[behavior/be/](./behavior/crud-be/README.md)** — Backend API specs ([Gherkin features](./behavior/crud-be/gherkin/README.md))
- **[behavior/web/](./behavior/crud-web/README.md)** — Web frontend app specs ([Gherkin features](./behavior/crud-web/gherkin/README.md))

## Spec Consumption

All crud backends consume the backend Gherkin specs at **all three test levels**:

- **`test:unit`** — steps call service functions with mocked dependencies; Gherkin spec paths
  are included in Nx cache inputs so cache invalidates when specs change
- **`test:quick`** — unit + coverage check; Gherkin spec paths included in Nx cache inputs
- **`test:integration`** — steps call service functions with real PostgreSQL; cache disabled

Spec-coverage validation (`rhino-cli spec-coverage validate`) is enforced for all crud apps via the
`spec-coverage` Nx target, the pre-push hook, and scheduled Test CI workflows.

## Related

- [Three-Level Testing Standard](../../../repo-governance/development/quality/three-level-testing-standard.md)
- [BDD Spec-Test Mapping](../../../repo-governance/development/infra/bdd-spec-test-mapping.md)
- [BDD Standards](../../../docs/explanation/software-engineering/development/behavior-driven-development-bdd/README.md)
