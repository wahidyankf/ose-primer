# Business Requirements Document — Update and Pin All npm Dependencies

## Business Goal

Ensure every npm dependency declared in `ose-primer` is pinned to an exact, reproducible
version that is current, CVE-free, and more than two months past its release date — eliminating
silent version drift, non-reproducible builds, and latent security exposure.

## Business Rationale

### Pain Points

**Non-reproducible builds**: Range prefixes (`^`, `~`) allow `npm install` to silently resolve
to a newer version than the one last tested. When that version introduces a breaking change or
a regression, the symptom appears in CI or production with no obvious cause.

**Stale and vulnerable packages**: Without a deliberate update pass, packages accumulate
unresolved minor upgrades. Some of those upgrades include security patches. Undiscovered CVEs
in transitive or direct dependencies are a compliance and reliability risk for any downstream
fork built from this template.

**Template credibility**: `ose-primer` is a reference template. Forks and consumers mirror its
dependency declarations. Stale, range-prefixed declarations in the template propagate as
anti-patterns into every project that starts from it.

### Expected Benefits

- `npm install` produces bit-for-bit identical resolutions on every machine and in every CI run.
- All direct dependencies are at a known-good, CVE-audited version.
- The template demonstrates the exact-pinning practice to downstream forks.
- Future dependency updates are deliberate and visible as diff-reviewable changes.

## Affected Roles

This is a solo-maintainer repository. The maintainer wears the following hats for this work:

- **Infrastructure maintainer**: owns the dependency manifest and lockfile.
- **Security reviewer**: validates CVE status and release-date eligibility.
- **Template author**: ensures the template reflects best practices for downstream consumers.

No sign-off ceremonies or stakeholder approval gates apply.

## Business-Level Success Metrics

All metrics below are observable facts verifiable by shell command — no fabricated numeric targets.

| Metric                                                       | How to verify                                                   |
| ------------------------------------------------------------ | --------------------------------------------------------------- |
| Zero packages with `^` or `~` prefixes in any `package.json` | `grep -rn '"\^\|"~' apps/ libs/ package.json` returns no output |
| Zero npm audit findings at high or critical severity         | `npm audit --audit-level=high` exits 0                          |
| All installed packages match their declared exact versions   | `npm ls --json` shows no `invalid` entries                      |
| CI passes on `main` after the change                         | All GitHub Actions checks green                                 |

_Judgment call_: The template's credibility benefit is qualitative and not separately measurable
beyond the observable technical facts above.

## Business-Scope Non-Goals

- Docker base image pinning in `infra/dev/` Dockerfiles — out of scope for this plan.
- Non-npm manifests (`go.mod`, `Cargo.toml`, `.csproj`, etc.) — separate toolchain, separate plan.
- Downgrading any package below its current resolved version.
- Automating ongoing dependency refresh (Renovate / Dependabot) — separate plan if desired.

## Business Risks and Mitigations

| Risk                                                                          | Likelihood | Mitigation                                                                                            |
| ----------------------------------------------------------------------------- | ---------- | ----------------------------------------------------------------------------------------------------- |
| A pinned upgrade introduces a breaking API change                             | Low–Medium | Run full quality gates (typecheck + lint + test:quick) before push; fix regressions before proceeding |
| A package version passes the cutoff date but has an undiscovered CVE          | Low        | Run `npm audit` after all pins applied; block push if any high/critical finding present               |
| Nx major migration required alongside minor Nx upgrade                        | Very Low   | Nx 22.5.2 → 22.6.2 is a patch-level bump; review Nx changelog before applying                         |
| App-level packages contain versions with no releases inside the cutoff window | Low        | Keep the current resolved version and pin to it as-is; document in delivery notes                     |
