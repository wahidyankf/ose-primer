# Component Diagram: Single Page Application

Level 3 of the C4 model. Shows the logical components inside the SPA container and how they relate.
Organised into five layers: pages, shared components, state management, API client, and
infrastructure.

**Public pages** (login, registration) bypass Auth Guard.
**Protected pages** pass through Auth Guard before rendering.
**Admin pages** additionally pass through Admin Guard after Auth Guard.

## Component Diagrams

### Auth and Core Pages Routing

```mermaid
%% Color Palette: Blue #0173B2 | Orange #DE8F05 | Teal #029E73 | Purple #CC78BC | Brown #CA9161 | Gray #808080
graph TB
    EU("End User"):::actor
    ADM("Administrator"):::actor_admin

    EU -->|"public/protected routes"| ROUTER
    ADM -->|"admin routes"| ROUTER

    ROUTER["Router<br/>────────────────<br/>Client-side routing<br/>Route guards"]:::infra
    ROUTER -->|"public routes"| LP
    ROUTER -->|"public routes"| RP
    ROUTER -->|"public routes"| HSP
    ROUTER -->|"protected routes"| AUTH_GUARD

    LP["Login Page<br/>────────────────<br/>Username + password<br/>Public"]:::page
    RP["Registration Page<br/>────────────────<br/>Form validation<br/>Public"]:::page
    HSP["Health Status<br/>────────────────<br/>Backend health<br/>indicator"]:::page
    AUTH_GUARD["Auth Guard<br/>────────────────<br/>Check token<br/>Redirect to login"]:::guard

    AUTH_GUARD --> DP
    AUTH_GUARD --> PP
    AUTH_GUARD --> ADMIN_GUARD

    DP["Dashboard Page<br/>────────────────<br/>Overview stats<br/>Quick actions"]:::page
    PP["Profile Page<br/>────────────────<br/>Display name edit<br/>Password change"]:::page
    ADMIN_GUARD["Admin Guard<br/>────────────────<br/>Check admin role<br/>Redirect to 403"]:::guard

    ADMIN_GUARD --> AP

    AP["Admin Panel<br/>────────────────<br/>User list, search<br/>Disable, enable"]:::page_admin

    LP --> AUTH_STORE
    AUTH_GUARD --> AUTH_STORE
    PP --> USER_API

    AUTH_STORE["Auth Store<br/>────────────────<br/>Access token<br/>Refresh token"]:::state
    USER_API["User API<br/>────────────────<br/>profile, password<br/>deactivate"]:::api

    AUTH_STORE --> AUTH_API
    AP --> ADMIN_API
    AUTH_API --> HTTP
    USER_API --> HTTP
    ADMIN_API --> HTTP

    AUTH_API["Auth API<br/>────────────────<br/>login, register<br/>refresh, logout"]:::api
    ADMIN_API["Admin API<br/>────────────────<br/>users, disable<br/>enable, unlock"]:::api
    HTTP["HTTP Client<br/>────────────────<br/>Auth header inject<br/>Token refresh"]:::api
    HTTP -->|"REST calls"| API

    API["Demo Backend<br/>REST API"]:::external

    classDef actor fill:#DE8F05,stroke:#000000,color:#000000,stroke-width:2px
    classDef actor_admin fill:#CA9161,stroke:#000000,color:#000000,stroke-width:2px
    classDef page fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef page_admin fill:#CA9161,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef state fill:#029E73,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef api fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef infra fill:#808080,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef guard fill:#CC78BC,stroke:#000000,color:#000000,stroke-width:2px
    classDef external fill:#808080,stroke:#000000,color:#FFFFFF,stroke-width:2px,stroke-dasharray:5 5
```

### Expense Pages and Data Flow

```mermaid
%% Color Palette: Blue #0173B2 | Orange #DE8F05 | Teal #029E73 | Purple #CC78BC | Gray #808080
graph TB
    EU("End User"):::actor

    EU -->|"protected routes"| AUTH_GUARD
    AUTH_GUARD["Auth Guard<br/>────────────────<br/>Check token<br/>Redirect to login"]:::guard

    AUTH_GUARD --> ELP
    AUTH_GUARD --> EDP
    AUTH_GUARD --> NEP
    AUTH_GUARD --> RPP

    ELP["Entry List Page<br/>────────────────<br/>Paginated table<br/>Filter and sort"]:::page
    EDP["Entry Detail Page<br/>────────────────<br/>Full entry view<br/>Attachment list"]:::page
    NEP["New Entry Page<br/>────────────────<br/>Entry form<br/>Currency select"]:::page
    RPP["Reporting Page<br/>────────────────<br/>Date range picker<br/>P&L chart"]:::page

    ELP --> ENTRY_STORE
    EDP --> ATTACH_API

    ENTRY_STORE["Entry Store<br/>────────────────<br/>Entry list cache<br/>Filter state"]:::state

    ENTRY_STORE --> EXPENSE_API
    ATTACH_API --> HTTP
    EXPENSE_API --> HTTP

    EXPENSE_API["Expense API<br/>────────────────<br/>CRUD, summary<br/>P&L reports"]:::api
    ATTACH_API["Attachment API<br/>────────────────<br/>upload, list<br/>delete"]:::api
    HTTP["HTTP Client<br/>────────────────<br/>Auth header inject<br/>Token refresh"]:::api
    HTTP -->|"REST calls"| API

    API["Demo Backend<br/>REST API"]:::external

    classDef actor fill:#DE8F05,stroke:#000000,color:#000000,stroke-width:2px
    classDef page fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef state fill:#029E73,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef api fill:#0173B2,stroke:#000000,color:#FFFFFF,stroke-width:2px
    classDef guard fill:#CC78BC,stroke:#000000,color:#000000,stroke-width:2px
    classDef external fill:#808080,stroke:#000000,color:#FFFFFF,stroke-width:2px,stroke-dasharray:5 5
```

## Gherkin Coverage by Component

Each component above is exercised by Gherkin features from
[`specs/apps/crud/behavior/crud-web/gherkin/`](../../behavior/crud-web/gherkin/README.md):

| Component                             | Gherkin Domain(s) | Features                                                         |
| ------------------------------------- | ----------------- | ---------------------------------------------------------------- |
| Health Status                         | health            | health-status (2)                                                |
| Login Page + Auth Store               | authentication    | login (5), session (7)                                           |
| Registration Page                     | user-lifecycle    | registration (6)                                                 |
| Profile Page                          | user-lifecycle    | user-profile (6)                                                 |
| Admin Panel                           | admin             | admin-panel (6)                                                  |
| Entry List + Entry Detail + New Entry | expenses          | expense-management (7), currency-handling (6), unit-handling (4) |
| Reporting Page                        | expenses          | reporting (6)                                                    |
| Entry Detail (attachments)            | expenses          | attachments (10)                                                 |
| Auth Store + Auth Guard               | token-management  | tokens (6)                                                       |
| Login Page (lockout)                  | security          | security (5)                                                     |
| Navigation + Data Display + all pages | layout            | responsive (10)                                                  |
| Form Kit + Modal + all pages          | layout            | accessibility (6)                                                |

## API Contract

All 3 frontend implementations generate types from the same OpenAPI 3.1 spec:

- **Source**: [`specs/apps/crud/containers/contracts/openapi.yaml`](../../containers/contracts/openapi.yaml)
- **Codegen target**: `nx run <frontend>:codegen` (depends on `crud-contracts:bundle`)
- **Output**: `<frontend>/generated-contracts/` or `<frontend>/src/generated-contracts/`

## Testing

| Level       | What                            | Gherkin             | Coverage |
| ----------- | ------------------------------- | ------------------- | -------- |
| `test:unit` | Service-layer calls, mocked API | Yes (all scenarios) | >= 70%   |
| `test:e2e`  | Full browser via Playwright     | Yes (all scenarios) | N/A      |

Frontends do not have `test:integration` — the unit/E2E split covers the
same ground. Unit tests use in-memory service clients (Flutter) or mocked
API modules (Next.js, TanStack Start).

## Related

- **Container diagram**: [container.md](../../containers/container.md)
- **Backend component diagram**: [component-be.md](../be/component-be.md)
- **API contract**: [../../containers/contracts/openapi.yaml](../../containers/contracts/openapi.yaml)
- **Frontend gherkin specs**: [web/gherkin/](../../behavior/crud-web/gherkin/README.md)
