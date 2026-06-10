# rhino-cli Gherkin Specs

Gherkin feature files for [rhino-cli](../../../../../../apps/rhino-cli-rust/README.md) — the Repository
Hygiene & INtegration Orchestrator CLI. 20 files, 155 scenarios across 11 domains.

## Feature Files

| Domain          | File                                                   | Command                           | Scenarios |
| --------------- | ------------------------------------------------------ | --------------------------------- | --------- |
| agents          | `agents/agents-sync.feature`                           | `agents sync`                     | 7         |
| agents          | `agents/agents-validate-claude.feature`                | `agents validate-claude`          | 5         |
| agents          | `agents/agents-validate-naming.feature`                | `agents validate-naming`          | 4         |
| contracts       | `contracts/contracts-dart-scaffold.feature`            | `contracts dart-scaffold`         | 3         |
| contracts       | `contracts/contracts-java-clean-imports.feature`       | `contracts java-clean-imports`    | 5         |
| docs            | `docs/docs-validate-links.feature`                     | `docs validate-links`             | 9         |
| docs            | `docs/docs-validate-mermaid.feature`                   | `docs validate-mermaid`           | 27        |
| docs            | `docs/docs-validate-heading-hierarchy.feature`         | `docs validate-heading-hierarchy` | 9         |
| env             | `env/env-backup.feature`                               | `env backup`                      | 18        |
| env             | `env/env-init.feature`                                 | `env init`                        | 4         |
| env             | `env/env-restore.feature`                              | `env restore`                     | 13        |
| git             | `git/git-pre-commit.feature`                           | `git pre-commit`                  | 1         |
| java            | `java/java-validate-annotations.feature`               | `java validate-annotations`       | 4         |
| repo-governance | `repo-governance/repo-governance-vendor-audit.feature` | `repo-governance vendor-audit`    | 7         |
| spec-coverage   | `spec-coverage/spec-coverage-validate.feature`         | `spec-coverage validate`          | 6         |
| system          | `system/doctor.feature`                                | `doctor`                          | 9         |
| test-coverage   | `test-coverage/test-coverage-diff.feature`             | `test-coverage diff`              | 4         |
| test-coverage   | `test-coverage/test-coverage-merge.feature`            | `test-coverage merge`             | 3         |
| test-coverage   | `test-coverage/test-coverage-validate.feature`         | `test-coverage validate`          | 10        |
| workflows       | `workflows/workflows-validate-naming.feature`          | `workflows validate-naming`       | 4         |

## Conventions

- **File naming**: `[domain]-[action].feature` (kebab-case, domain-prefixed)
- **Directory**: Each domain has its own subdirectory; file placed in `<domain>/`
- **Step language**: CLI-semantic only — no framework or library names
- **User story block**: Every `Feature:` block opens with `As a … / I want … / So that …`

## Related

- **Parent**: [rhino-cli specs](../../../README.md)
- **BDD Standards**: [behavior-driven-development-bdd/](../../../../../../docs/explanation/software-engineering/development/behavior-driven-development-bdd/README.md)
