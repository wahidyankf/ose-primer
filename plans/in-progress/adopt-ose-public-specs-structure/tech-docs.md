---
title: "Tech Docs: Adopt ose-public Specs Structure"
description: Gap inventory, migration recipes, and file-impact map
category: plan
---

# Tech Docs — Adopt ose-public Specs Structure

## Gap Inventory

| ID    | Location                                                             | Current state                                                                                               | Target state                                                                           | Severity |
| ----- | -------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- | -------- |
| GAP-1 | `specs/README.md` "Standard Folder Pattern" section                  | Documents flat `be/fe/c4/contracts/` as canonical                                                           | Five-folder tree + `behavior/<surface>/gherkin/<domain>/`                              | HIGH     |
| GAP-2 | `specs/apps/crud/`                                                   | Flat-root: `be/`, `fe/`, `c4/`, `contracts/` at app root                                                    | Five-folder C4-aware tree; `fe/` renamed to `behavior/web/`                            | HIGH     |
| GAP-3 | `specs/apps/rhino/`                                                  | Only `behavior/cli/gherkin/` populated; missing `product/`, `system-context/`, `containers/`, `components/` | Full CLI-only five-folder profile                                                      | MEDIUM   |
| GAP-4 | `specs/apps/rhino/behavior/cli/gherkin/`                             | 19 flat `.feature` files directly under `gherkin/`                                                          | All features inside domain subdirs per D5 grouping table                               | HIGH     |
| GAP-5 | `repo-governance/conventions/structure/specs-directory-structure.md` | Old convention: flat-root layout, CLI flat exception, no migration map                                      | New C4-aware five-folder convention matching ose-public (updated via repo-rules-maker) | HIGH     |
| GAP-6 | 17 `apps/crud-*/project.json`                                        | `specs/apps/crud/be/gherkin` and `fe/gherkin` in Nx inputs + spec-coverage commands                         | `behavior/be/gherkin` and `behavior/web/gherkin`                                       | HIGH     |
| GAP-7 | 9 governance docs + `specs/README.md` + `README.md` (root)           | Path examples reference `crud/be/`, `crud/fe/`, `crud/c4/`, flat rhino CLI features                         | All examples updated to canonical paths                                                | HIGH     |
| GAP-8 | `plans/in-progress/add-investment-oracle-app/` (3 files)             | References `crud/c4/`, `crud/be/gherkin/`, `crud/fe/gherkin/`, old `contracts/` location                    | Updated to new five-folder paths                                                       | MEDIUM   |

## Decision — D1: Domain Groupings for Rhino CLI Gherkin

**Default groupings** (apply at execution time; adjust if filenames have drifted):

| Domain             | Files                                                                                         |
| ------------------ | --------------------------------------------------------------------------------------------- |
| `agents/`          | `agents-sync.feature`, `agents-validate-claude.feature`, `agents-validate-naming.feature`     |
| `contracts/`       | `contracts-dart-scaffold.feature`, `contracts-java-clean-imports.feature`                     |
| `docs/`            | `docs-validate-links.feature`, `docs-validate-mermaid.feature`                                |
| `env/`             | `env-backup.feature`, `env-init.feature`, `env-restore.feature`                               |
| `git/`             | `git-pre-commit.feature`                                                                      |
| `java/`            | `java-validate-annotations.feature`                                                           |
| `repo-governance/` | `repo-governance-vendor-audit.feature`                                                        |
| `spec-coverage/`   | `spec-coverage-validate.feature`                                                              |
| `system/`          | `doctor.feature` (lone singleton with no domain prefix)                                       |
| `test-coverage/`   | `test-coverage-diff.feature`, `test-coverage-merge.feature`, `test-coverage-validate.feature` |
| `workflows/`       | `workflows-validate-naming.feature`                                                           |

**Pre-flight**: verify with
`ls specs/apps/rhino/behavior/cli/gherkin/*.feature | sort` before executing. The
table above was verified on 2026-05-24. If filenames have drifted, assign new files
to the closest domain or create a new domain matching the feature's prefix.

## Migration Recipes

### R1 — crud: Flat-root → C4-aware five-folder tree (atomic commit)

```bash
# Step 1 — create destination directories
mkdir -p specs/apps/crud/behavior
mkdir -p specs/apps/crud/system-context
mkdir -p specs/apps/crud/containers
mkdir -p specs/apps/crud/components/be
mkdir -p specs/apps/crud/components/web

# Step 2 — git mv: be/ and fe/ into behavior/
git mv specs/apps/crud/be   specs/apps/crud/behavior/be
git mv specs/apps/crud/fe   specs/apps/crud/behavior/web   # fe RENAMED to web

# Step 3 — git mv: c4/ → split into canonical C4 folders
git mv specs/apps/crud/c4/context.md      specs/apps/crud/system-context/context.md
git mv specs/apps/crud/c4/container.md    specs/apps/crud/containers/container.md
git mv specs/apps/crud/c4/component-be.md specs/apps/crud/components/be/component-be.md
git mv specs/apps/crud/c4/component-fe.md specs/apps/crud/components/web/component-web.md  # RENAMED

# Step 4 — git mv: contracts/ into containers/
git mv specs/apps/crud/contracts          specs/apps/crud/containers/contracts

# Step 5 — remove now-empty c4/ directory
git rm specs/apps/crud/c4/README.md

# Step 6 — create new README files for each new folder
# (see R3 skeleton template; use docs-maker for content)

# Step 7 — sweep ALL path references in the same commit
# 17 project.json files + governance docs + specs READMEs:
grep -rln 'specs/apps/crud/be/gherkin' apps \
  | xargs -I {} sed -i.bak 's|specs/apps/crud/be/gherkin|specs/apps/crud/behavior/be/gherkin|g' {}
grep -rln 'specs/apps/crud/fe/gherkin' apps \
  | xargs -I {} sed -i.bak 's|specs/apps/crud/fe/gherkin|specs/apps/crud/behavior/web/gherkin|g' {}
grep -rln 'specs/apps/crud/contracts' apps \
  | xargs -I {} sed -i.bak 's|specs/apps/crud/contracts|specs/apps/crud/containers/contracts|g' {}
find . -name '*.bak' -delete

# Step 8 — verify
find specs/apps/crud -maxdepth 1 -type d | sort
# Must not include: be/, fe/, c4/, contracts/
# Must include: product/(optional), system-context/, containers/, components/, behavior/
```

### R2 — rhino: Fill out CLI-only five-folder tree + regroup features (atomic commit)

```bash
# Step 1 — create missing C4 folders
mkdir -p specs/apps/rhino/product
mkdir -p specs/apps/rhino/system-context
mkdir -p specs/apps/rhino/containers
mkdir -p specs/apps/rhino/components/cli

# Step 2 — create CLI-gherkin domain subdirs per D1 grouping table
mkdir -p specs/apps/rhino/behavior/cli/gherkin/{agents,contracts,docs,env,git,java,repo-governance,spec-coverage,system,test-coverage,workflows}

# Step 3 — git mv features by prefix (loop pattern)
for f in specs/apps/rhino/behavior/cli/gherkin/agents-*.feature; do
  git mv "$f" specs/apps/rhino/behavior/cli/gherkin/agents/"$(basename "$f")"
done
for f in specs/apps/rhino/behavior/cli/gherkin/contracts-*.feature; do
  git mv "$f" specs/apps/rhino/behavior/cli/gherkin/contracts/"$(basename "$f")"
done
for f in specs/apps/rhino/behavior/cli/gherkin/docs-*.feature; do
  git mv "$f" specs/apps/rhino/behavior/cli/gherkin/docs/"$(basename "$f")"
done
for f in specs/apps/rhino/behavior/cli/gherkin/env-*.feature; do
  git mv "$f" specs/apps/rhino/behavior/cli/gherkin/env/"$(basename "$f")"
done
for f in specs/apps/rhino/behavior/cli/gherkin/git-*.feature; do
  git mv "$f" specs/apps/rhino/behavior/cli/gherkin/git/"$(basename "$f")"
done
for f in specs/apps/rhino/behavior/cli/gherkin/java-*.feature; do
  git mv "$f" specs/apps/rhino/behavior/cli/gherkin/java/"$(basename "$f")"
done
for f in specs/apps/rhino/behavior/cli/gherkin/repo-governance-*.feature; do
  git mv "$f" specs/apps/rhino/behavior/cli/gherkin/repo-governance/"$(basename "$f")"
done
for f in specs/apps/rhino/behavior/cli/gherkin/spec-coverage-*.feature; do
  git mv "$f" specs/apps/rhino/behavior/cli/gherkin/spec-coverage/"$(basename "$f")"
done
for f in specs/apps/rhino/behavior/cli/gherkin/test-coverage-*.feature; do
  git mv "$f" specs/apps/rhino/behavior/cli/gherkin/test-coverage/"$(basename "$f")"
done
for f in specs/apps/rhino/behavior/cli/gherkin/workflows-*.feature; do
  git mv "$f" specs/apps/rhino/behavior/cli/gherkin/workflows/"$(basename "$f")"
done
# Singleton with no domain prefix:
git mv specs/apps/rhino/behavior/cli/gherkin/doctor.feature \
       specs/apps/rhino/behavior/cli/gherkin/system/doctor.feature

# Step 4 — verify no flat features remain
find specs/apps/rhino/behavior/cli/gherkin -maxdepth 1 -name '*.feature'
# Must return empty

# Step 5 — sweep path references (per-file refs in bdd-spec-test-mapping.md)
grep -rln 'specs/apps/rhino/behavior/cli/gherkin/' \
  repo-governance docs .github .husky \
  | xargs -I {} sed -i.bak \
    's|cli/gherkin/agents-sync|cli/gherkin/agents/agents-sync|g;
     s|cli/gherkin/agents-validate-claude|cli/gherkin/agents/agents-validate-claude|g;
     s|cli/gherkin/agents-validate-naming|cli/gherkin/agents/agents-validate-naming|g;
     s|cli/gherkin/doctor|cli/gherkin/system/doctor|g' {}
find . -name '*.bak' -delete
# Hand-verify any remaining per-file references in governance docs.
```

### R3 — Skeleton README template

```markdown
# <APP> — <FOLDER>

<one-line description of this C4 level for <APP>>

> _Skeleton placeholder. Substantive content to be authored in a follow-up plan._

See [Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical purpose of this folder.
```

Adjust relative-link depth per nesting level (e.g., `components/cli/` needs `../../../../../`).

### R4 — Convention doc replacement (repo-rules-maker)

`specs-directory-structure.md` must be rewritten to match the ose-public version
(2026-05-24 state). Key changes from the current ose-primer version:

1. Replace "Gherkin Feature Files" section with five-folder C4-aware canonical tree
2. Drop CLI-flat exception; replace with "domain subdirs for every surface" rule
3. Add "Flat-Root to C4-Aware" migration table (maps `be/` → `behavior/be/`, etc.)
4. Update `be/fe/c4/contracts/` examples throughout to new canonical paths
5. Add §Migration Path dated note for ose-primer adoption (2026-05-24)

Use `repo-rules-maker` agent. Pass the ose-public convention doc as reference.

### R5 — Governance doc propagation (repo-rules-maker)

Files requiring path-example updates (delegate to `repo-rules-maker`):

| File                                                                   | Stale references                                                              |
| ---------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| `repo-governance/development/infra/bdd-spec-test-mapping.md`           | `specs/apps/crud/be/gherkin/`, `specs/apps/rhino/behavior/cli/gherkin/<flat>` |
| `repo-governance/development/infra/ci-conventions.md`                  | `specs/apps/crud/be/gherkin/`, `specs/apps/crud/fe/gherkin/`                  |
| `repo-governance/development/infra/nx-targets.md`                      | `specs/apps/crud/be/gherkin/`, `specs/apps/crud/fe/gherkin/`                  |
| `repo-governance/development/quality/specs-application-sync.md`        | `specs/apps/crud/c4/`, `crud/be/gherkin/`, `crud/fe/gherkin/`                 |
| `repo-governance/development/quality/feature-change-completeness.md`   | `crud/be/gherkin/`, `crud/fe/gherkin/`                                        |
| `repo-governance/development/quality/three-level-testing-standard.md`  | `crud/be/gherkin/`, `crud/fe/gherkin/`                                        |
| `repo-governance/workflows/specs/specs-quality-gate.md`                | `specs/apps/crud/be/`, path examples                                          |
| `repo-governance/conventions/formatting/diagrams.md`                   | `specs/apps/crud/c4/` reference                                               |
| `repo-governance/conventions/writing/dynamic-collection-references.md` | `crud/be/gherkin/` examples                                                   |
| `specs/README.md`                                                      | "Standard Folder Pattern" + App Specs links                                   |
| `README.md` (root)                                                     | Minor crud spec path references                                               |
| `plans/in-progress/add-investment-oracle-app/README.md`                | `crud/c4/`, `crud/contracts/`, `crud/be/gherkin/`, `crud/fe/gherkin/`         |
| `plans/in-progress/add-investment-oracle-app/tech-docs.md`             | Same                                                                          |
| `plans/in-progress/add-investment-oracle-app/delivery.md`              | Same                                                                          |

## Atomic Commit Discipline

Per `specs-directory-structure.md §Migration Path`:

> The atomic commit is mandatory — splitting the move and the path updates
> causes test failures between commits.

Every `git mv` commit MUST contain the corresponding path-reference sweep in
the same commit. Do not push between `git mv` and `sed` sweep.

## File Impact Map

| File                                                                            | Action                                  |
| ------------------------------------------------------------------------------- | --------------------------------------- |
| `specs/apps/crud/be/` → `specs/apps/crud/behavior/be/`                          | `git mv`                                |
| `specs/apps/crud/fe/` → `specs/apps/crud/behavior/web/`                         | `git mv` + rename                       |
| `specs/apps/crud/c4/context.md` → `system-context/context.md`                   | `git mv`                                |
| `specs/apps/crud/c4/container.md` → `containers/container.md`                   | `git mv`                                |
| `specs/apps/crud/c4/component-be.md` → `components/be/component-be.md`          | `git mv`                                |
| `specs/apps/crud/c4/component-fe.md` → `components/web/component-web.md`        | `git mv` + rename                       |
| `specs/apps/crud/contracts/` → `containers/contracts/`                          | `git mv`                                |
| `specs/apps/crud/c4/README.md`                                                  | `git rm`                                |
| `specs/apps/crud/behavior/README.md`                                            | Create (new)                            |
| `specs/apps/crud/system-context/README.md`                                      | Create (new)                            |
| `specs/apps/crud/containers/README.md`                                          | Create (new)                            |
| `specs/apps/crud/components/README.md`                                          | Create (new)                            |
| `specs/apps/crud/components/be/README.md`                                       | Create (new)                            |
| `specs/apps/crud/components/web/README.md`                                      | Create (new)                            |
| `specs/apps/crud/README.md`                                                     | Update five-folder structure block      |
| `specs/apps/crud/behavior/be/README.md`                                         | Update path cross-refs                  |
| `specs/apps/crud/behavior/web/README.md`                                        | Update (was `fe/README.md`)             |
| `specs/apps/crud/behavior/be/gherkin/README.md`                                 | Update path cross-refs                  |
| `specs/apps/crud/behavior/web/gherkin/README.md`                                | Update (was `fe/gherkin/README.md`)     |
| `specs/apps/crud/containers/contracts/README.md`                                | Update path cross-refs                  |
| `specs/apps/rhino/{product,system-context,containers,components/cli}/README.md` | Create skeletons (new)                  |
| `specs/apps/rhino/behavior/cli/gherkin/<11 domain dirs>/README.md`              | Create one-para index each              |
| `specs/apps/rhino/README.md`                                                    | Update Structure block                  |
| `specs/apps/rhino/behavior/README.md`                                           | Update Structure block                  |
| `specs/apps/rhino/behavior/cli/gherkin/README.md`                               | Update to per-domain tables             |
| 19 flat `.feature` files under rhino gherkin root                               | `git mv` into domain subdirs            |
| `apps/crud-be-*/project.json` (12 files)                                        | `be/gherkin` → `behavior/be/gherkin`    |
| `apps/crud-fe-*/project.json` (4 files)                                         | `fe/gherkin` → `behavior/web/gherkin`   |
| `apps/crud-fs-*/project.json` (1 file)                                          | Both paths updated                      |
| `repo-governance/conventions/structure/specs-directory-structure.md`            | Replace with C4-aware version           |
| 9 governance docs (see R5 table)                                                | Update path examples                    |
| `specs/README.md`                                                               | Rewrite Standard Folder Pattern section |
| `README.md` (root)                                                              | Update crud spec path reference         |
| `plans/in-progress/add-investment-oracle-app/` (3 files)                        | Update crud path references             |

## Verification Commands

```bash
# Confirm no flat-root artifacts remain in crud:
find specs/apps/crud -maxdepth 1 -type d | sort
# Must NOT include: be/, fe/, c4/, contracts/

# Confirm no flat features remain in rhino:
find specs/apps/rhino/behavior/cli/gherkin -maxdepth 1 -name '*.feature'
# Must return empty

# Confirm Nx inputs updated:
grep -r 'specs/apps/crud/be/gherkin\|specs/apps/crud/fe/gherkin\|specs/apps/crud/c4' \
  --include='*.json' apps/ | grep -v node_modules
# Must return empty

# Markdown lint:
npm run lint:md

# Spec coverage (crud backends):
npx nx run-many -t spec-coverage --projects=crud-be-golang-gin,crud-be-rust-axum
```
