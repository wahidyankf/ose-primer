---
title: "Repository Ecosystem Convention"
description: Canonical description of the three sibling repositories in the OSE family, their roles, propagation rules, and how to verify that propagation has occurred.
category: explanation
subcategory: conventions
tags:
  - conventions
  - repository-structure
  - ecosystem
  - propagation
  - sibling-repos
---

# Repository Ecosystem Convention

The OSE (Open Sharia Enterprise) family consists of three independent git repositories. There is no parent monorepo, no submodule relationship, and no shared workspace. The three repos stay aligned through explicit textual cross-references and deliberate propagation of governance artifacts.

## Principles Implemented/Respected

- **[Explicit Over Implicit](../../principles/software-engineering/explicit-over-implicit.md)**: Sibling relationships and propagation directions are stated in writing rather than inferred from directory structure or tooling conventions.
- **[Documentation First](../../principles/content/documentation-first.md)**: The ecosystem relationship is codified here before it can be assumed from context, giving contributors a stable reference.
- **[Simplicity Over Complexity](../../principles/general/simplicity-over-complexity.md)**: A flat family of independent repos with documented propagation rules is simpler to operate than a nested monorepo or submodule graph.

## Purpose

Before 2026-05-23, the three OSE repositories lived under a shared parent directory managed by an umbrella repository called `ose-projects`. That umbrella was deleted on 2026-05-23. The three repositories are now fully independent top-level git repositories.

This change means there is no structural mechanism — no shared `package.json`, no `.gitmodules`, no workspace link — to remind contributors that the three repos form a family. This convention provides that reminder. It records:

- which repositories belong to the family,
- the role and visibility of each,
- which artifacts propagate between them and in which direction, and
- how to verify that a propagation has reached its destination.

Each repository in the family carries its own copy of this convention so that the rules are self-documenting regardless of which repo a contributor enters first.

## Sibling Repositories

| Repository   | URL                                      | Role                                                                                                                 | Visibility | License           |
| ------------ | ---------------------------------------- | -------------------------------------------------------------------------------------------------------------------- | ---------- | ----------------- |
| `ose-public` | https://github.com/wahidyankf/ose-public | Main OSE platform monorepo; upstream source of governance, conventions, agents, and skills                           | Public     | Open source (MIT) |
| `ose-primer` | https://github.com/wahidyankf/ose-primer | Repository template extracted from `ose-public`; MIT-licensed starting point for new OSE-style polyglot Nx monorepos | Public     | MIT               |
| `ose-infra`  | https://github.com/wahidyankf/ose-infra  | Private infrastructure repo (Terraform, deploy pipelines, cloud configuration) backing `ose-public`                  | Private    | Proprietary       |

Each repository is independently clonable. A contributor working in `ose-primer` does not need access to `ose-public` or `ose-infra` to do useful work.

## Propagation Rules

### Governance, conventions, agents, and skills

Governance artifacts — conventions, principles, agent definitions, agent skills, workflow definitions — originate in `ose-public` and propagate outward:

```
ose-public  →  ose-primer  →  downstream forks
```

When a convention is updated in `ose-public`, the maintainer of `ose-primer` applies the equivalent change to `ose-primer`. Downstream forks decide independently whether to pull the update.

`ose-primer` never propagates governance changes back to `ose-public`. Changes that the `ose-primer` maintainer discovers as improvements should be raised as contributions to `ose-public` first, then pulled forward.

The multi-harness binding scaffolding propagates along this same path: the [Multi-Harness Binding](./multi-harness-binding.md) convention, the generated bridge files for non-`AGENTS.md`-reading harnesses, the deterministic binding-parity guard (`rhino-cli agents validate-bindings`), and the harness-compatibility audit workflow plus its checker/fixer agents all flow `ose-public → ose-primer → downstream forks`. A downstream fork that adopts `ose-primer` therefore inherits compatibility with the supported coding-agent harnesses out of the box. Product-specific binding content (which harnesses a given fork actually wires up) is a per-fork decision and does not propagate.

### Infrastructure concerns

Infrastructure configuration — Terraform modules, deploy pipelines, cloud resource definitions, secrets management — is managed between `ose-public` and `ose-infra` only:

```
ose-public  ↔  ose-infra
```

`ose-primer` has no infrastructure. It does not receive infrastructure artifacts and does not propagate them.

### What does not propagate to `ose-primer`

The following never flow from `ose-public` to `ose-primer`:

- Infrastructure configuration or deploy pipeline definitions.
- Application-specific business logic or product data models.
- Internal service credentials, environment-specific configuration, or secrets.

### What does not propagate from `ose-primer`

`ose-primer` is a downstream template, not an upstream source. Changes made in `ose-primer` do not automatically flow back to `ose-public` or to `ose-infra`. A human decision is required to contribute any `ose-primer` change upstream.

## Anti-Patterns

The following are explicitly prohibited. Do not introduce them, and remove them if found.

**No submodules.** Do not add any of the three sibling repositories as a git submodule of another. Submodules create hidden coupling and complicate cloning for contributors who need only one repo.

**No workspace links.** Do not reference sibling repositories as npm workspaces, Go module replacements, or any other dependency-manager-level link. Each repo resolves its own dependencies from public registries.

**No resurrection of `ose-projects`.** The `ose-projects` umbrella repository was deleted on 2026-05-23. Do not recreate it under the same name or a similar name. If a new umbrella structure is ever required, it must be established through a deliberate governance decision documented in a plan, not through ad hoc recreation.

**No implicit propagation.** Do not assume that a change in one repo has reached another. Use the Verification steps below to confirm propagation explicitly.

## Verification

To confirm that a governance artifact has propagated from `ose-public` to `ose-primer` (or to a downstream fork), check the following three signals in the destination repository:

1. **Sibling URL presence in `README.md`**: The README must reference all three sibling URLs. Grep for any of the three repository URLs:

   ```bash
   grep -n "wahidyankf/ose-public\|wahidyankf/ose-primer\|wahidyankf/ose-infra" README.md
   ```

   All three URLs should appear.

2. **Sibling URL presence in `AGENTS.md`**: The canonical agent instruction surface must also reference the sibling relationship:

   ```bash
   grep -n "wahidyankf/ose-public\|wahidyankf/ose-primer\|wahidyankf/ose-infra" AGENTS.md
   ```

3. **Presence of this convention file**: The convention file itself is the strongest signal. If `repo-governance/conventions/structure/repository-ecosystem.md` is present and non-empty, the propagation is confirmed for the ecosystem convention specifically:

   ```bash
   test -s repo-governance/conventions/structure/repository-ecosystem.md && echo "present"
   ```

4. **Convention index entry**: The `repo-governance/conventions/structure/README.md` must list this file. Grep for the entry:

   ```bash
   grep -n "repository-ecosystem" repo-governance/conventions/structure/README.md
   ```

A destination repo that passes all four checks has received this propagation.

## Related

- [Governance Vendor Independence](./governance-vendor-independence.md) — Propagating this convention to sibling repositories does not override the vendor-neutrality requirement. Each copy of this file must remain free of forbidden vendor terms in load-bearing prose, exactly as any other governance file must.
- [File Naming Convention](./file-naming.md) — Kebab-case file naming applies to this convention file and to any future ecosystem-related governance documents.
- [Repository Governance Architecture](../../repository-governance-architecture.md) — Six-layer governance hierarchy that this convention sits within (Layer 2: Conventions).

## Conventions Implemented/Respected

- **[File Naming Convention](./file-naming.md)**: This file uses kebab-case.
- **[Governance Vendor Independence](./governance-vendor-independence.md)**: This file contains no forbidden vendor terms in load-bearing prose.
- **[Content Quality Principles](../writing/quality.md)**: Active voice, proper heading hierarchy, single H1.
