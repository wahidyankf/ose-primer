# Applications

`apps/` is the hands-on reference library in `ose-primer`. Each project is a deliberately small,
working example of a delivery surface or language stack—not a product that a new repository must
keep. Use these apps to see how the workspace, contracts, tests, and quality gates fit together;
then keep, adapt, or remove them when you make the primer your own. 🧭

## Start with one example

From the repository root, run a project through Nx with the workspace-pinned CLI:

```bash
npm exec nx -- dev crud-fe-ts-nextjs
```

The Next.js frontend starts on its configured local port. For a backend example, choose one
implementation and read its README before starting it—the backend examples share a behavioral
contract but use their language-native tooling and, for some flows, a local database.

## What is here

| Group                | Projects                                                                                                                                                          | Why you might open it                                                                                                          |
| -------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| Backend references   | `crud-be-{clojure-pedestal,csharp-aspnetcore,elixir-phoenix,fsharp-giraffe,golang-gin,java-springboot,java-vertx,kotlin-ktor,python-fastapi,rust-axum,ts-effect}` | Compare one API contract across practical language and framework choices.                                                      |
| Frontend references  | `crud-fe-{dart-flutterweb,ts-nextjs,ts-tanstack-start}`                                                                                                           | Explore separate web-client approaches for the same kind of product work.                                                      |
| Full-stack reference | `crud-fs-ts-nextjs`                                                                                                                                               | See a compact Next.js application that owns both browser and server concerns.                                                  |
| End-to-end harnesses | `crud-be-e2e`, `crud-fe-e2e`                                                                                                                                      | Learn how Gherkin behavior is exercised against a running backend or browser UI.                                               |
| Repository tooling   | `rhino-cli`                                                                                                                                                       | The workspace’s Rust CLI for repository checks and automation; it is maintained in lockstep with its sibling OSE repositories. |

The `crud-*` names identify examples, not a required product domain. Their shared contract and
behavioral specifications live in [specs/](../specs/README.md).

## Conventions that matter

- Apps are deployable workspace projects. They can import reusable packages from
  [libs/](../libs/README.md), but must not import another app.
- Names describe the surface and implementation: `be` for backend, `fe` for frontend, `fs` for
  full-stack, and `*-e2e` for an end-to-end test harness.
- Read an app’s own README for prerequisites and first-run commands. The examples intentionally
  demonstrate different runtimes, so there is no single runtime prerequisite beyond the workspace
  setup described in the root README.
- Never copy real credentials into documentation or committed files. When an app supports local
  configuration, begin with its tracked example configuration and keep real values local.

## Useful commands

```bash
# See the targets a project exposes
npm exec nx -- show project crud-fe-ts-nextjs

# Run the quick quality gate for one project
npm exec nx -- run crud-fe-ts-nextjs:test:quick

# See workspace relationships
npm exec nx -- graph
```

For a clean starting point, the root [README](../README.md) explains how to keep only the examples
that serve your new repository. The rest of the docs explain the reusable workflow rather than
treating these references as a production service catalogue.
