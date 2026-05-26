# BRD — Adopt Hexagonal Architecture + DDD

## Business Goal

Establish hexagonal architecture and Domain-Driven Design (DDD) as the canonical structural
pattern for all non-E2E applications in ose-primer, making the monorepo a reliable, navigable
reference template for polyglot production repos.

## Business Rationale

ose-primer serves as a template that teams fork when starting new OSE-style monorepos. A template
has value only when it demonstrates clear, correct patterns. Today each app uses an
idiosyncratic structure: some have partial hexagonal layers, others are completely flat, and
none enforce bounded-context separation. A new contributor opening any two apps sees two
completely different shapes.

The problems this causes:

- **Onboarding friction**: contributors must re-learn the layout for each language — the
  template teaches inconsistency rather than a transferable pattern.
- **Architectural drift risk in downstream repos**: teams forking ose-primer copy whatever
  structure is present, propagating flat or inconsistent layouts into production codebases.
- **Testing gaps**: without an explicit application layer there is nowhere to put use-case
  tests that are isolated from HTTP and database concerns.
- **OpenAPI contract coverage is uneven**: some BE apps have wired codegen targets, others do
  not — FE apps must write their own client types, which quickly diverge from the server.

## Business Impact

- Contributors working across apps see one canonical pattern regardless of language.
- Teams forking ose-primer get a correct architectural template for each of the 11 supported
  languages.
- The five governance convention documents become the normative reference for architectural
  decisions in downstream OSE repos.

## Affected Roles

This is a solo-maintainer repository. The maintainer wears all relevant hats:

- **Template author** — concerned with correctness and breadth of patterns across all 11 languages
- **Governance author** — concerned with convention documents being accurate and navigable
- **Downstream consumer** — teams or agents forking the repo and following the template

## Business-Level Success Metrics

- **Observable fact**: All 17 non-E2E apps have the canonical hexagonal layer directories
  established as verifiable paths on disk. `bash test -d` checks in CI pass for every expected
  directory. [Repo-grounded after delivery]
- **Observable fact**: All 5 governance convention documents exist under
  `repo-governance/development/pattern/` and pass markdownlint. [Repo-grounded after delivery]
- **Observable fact**: All existing tests continue to pass after the structural changes — zero
  regressions. [Repo-grounded after delivery]
- **Observable fact**: All 11 BE apps have Nx codegen targets for OpenAPI contracts wired to
  the three primary FE consumers. [Repo-grounded after delivery]
- _Judgment call_: A new contributor able to read `tech-docs.md` can navigate any app's
  hexagonal structure without reading app-specific documentation. This is not measurable
  automatically but is the north-star intent behind the layering convention.

## Business-Scope Non-Goals

- Rewriting any application's business logic — only directory/module structure changes.
- Adding new application features or API endpoints.
- Adding new language apps beyond the 17 currently in scope.
- Changing database schemas or running database migrations.
- Adopting hexagonal architecture in the two E2E apps (`crud-fe-e2e`, `crud-be-e2e`).

## Business Risks and Mitigations

| Risk                                                     | Likelihood | Impact | Mitigation                                                                    |
| -------------------------------------------------------- | ---------- | ------ | ----------------------------------------------------------------------------- |
| Structural changes break existing test imports           | Medium     | High   | Phase 0 baseline; test after each app; fix before proceeding                  |
| Governance docs introduce language-specific inaccuracies | Medium     | Medium | Research findings from resolved design decisions; inline confidence labels    |
| OpenAPI codegen wiring introduces build-time regressions | Low        | High   | Verify codegen targets compile; include in affected CI checks                 |
| Partial migration left in `main` if plan is interrupted  | Low        | Medium | Each phase ends with a CI-green push; no half-migrated state left uncommitted |
