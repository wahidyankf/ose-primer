# Security Waivers Register

Long-lived register of dependency-bump security waivers and functional holds for this repository.
Every entry here is created by the
[Dependency Bump Stability & Safety Policy](../../repo-governance/development/workflow/dependency-bump-policy.md):
when a bump cannot satisfy both the 60-day soak rule and CVE-cleanness (Path C), or when a newer
eligible version is skipped because of a known fatal functional defect (Rule 5b / `FUNCTIONAL-HOLD`),
the decision is recorded here so the trade-off is auditable over time.

## When to add an entry

Add a row to the table below when any of the following clearance statuses is issued for a pinned
dependency (see the policy's
[CVE Clearance Process](../../repo-governance/development/workflow/dependency-bump-policy.md)):

- **WAIVER** — Path C applied: the most recent CVE-patched version is pinned even though it has not
  completed the 60-day soak (or no pre-cutoff CVE-clean version exists).
- **FUNCTIONAL-HOLD** — a newer eligible version was skipped due to a known fatal functional defect
  (yanked/deprecated, open release-blocker, or widely-reported crash/data-loss bug); an older
  eligible version is pinned instead.
- Any of the above carrying the **`(KEV-listed)`** suffix — the CVE appeared in the CISA KEV
  catalog (confirmed active exploitation) at the time of the bump.

The introducing plan's `tech-docs.md` also records the per-bump Security Clearance Status table; this
register is the cross-plan, long-lived destination that survives plan archival.

## Register

Each entry MUST capture the following columns. KEV columns are populated only for `(KEV-listed)`
entries (otherwise leave as `—`).

| Date | Package | Pinned Version | Status (WAIVER / FUNCTIONAL-HOLD) | CVE(s) + URL | Severity | Release Date | EPSS (score / pct) | KEV `dateAdded` | KEV ransomware use | Justification | Sign-off |
| ---- | ------- | -------------- | --------------------------------- | ------------ | -------- | ------------ | ------------------ | --------------- | ------------------ | ------------- | -------- |

| 2026-06-04 | next (npm) | 16.2.7 | WAIVER | 13 CVEs incl. [GHSA next 16.2.x](https://github.com/advisories?query=next.js) (first patched 16.2.5/16.2.6) | High | 2026-06-02 | — | — | — | 13 CVEs unpatched in pinned-from 16.2.1; first fixes 16.2.5/16.2.6 released post-cutoff. | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | react + react-dom (npm) | 19.2.7 | WAIVER | [CVE-2026-23870](https://nvd.nist.gov/vuln/detail/CVE-2026-23870) (DoS) | High | 2026-06-01 | — | — | — | DoS unpatched in 19.2.4; fix 19.2.6+ post-cutoff. | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | golang.org/x/crypto (go) | v0.52.0 | WAIVER | 13x GO-2026 SSH CVEs incl. [GO-2026-5019](https://pkg.go.dev/vuln/GO-2026-5019) | High | 2026-05-22 | — | — | — | SSH-subpackage CVEs fixed only in v0.52.0 (post-cutoff); golang.org/x/crypto/ssh NOT imported by crud-be-golang-gin — unreachable, low residual risk. | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | spring-boot-starter-parent (maven) | 4.0.6 | WAIVER | [CVE-2026-40976](https://spring.io/security/cve-2026-40976/) (Actuator auth bypass) | Critical (9.1) | 2026-04-23 | — | — | — | Fix 4.0.6 released 2026-04-23 (post-cutoff); no pre-cutoff fixed version. | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | postgresql JDBC (maven/gradle — crud-be-java-vertx, crud-be-kotlin-ktor) | 42.7.11 | WAIVER | [CVE-2025-49146](https://nvd.nist.gov/vuln/detail/CVE-2025-49146) (MITM) + [CVE-2026-42198](https://nvd.nist.gov/vuln/detail/CVE-2026-42198) (SCRAM DoS) | High (8.2 / 7.5) | 2026-04-28 | — | — | — | CVE-2026-42198 fix only in 42.7.11 (post-cutoff). | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | org.postgresql/postgresql (clojure deps.edn — crud-be-clojure-pedestal) | 42.7.11 | WAIVER | [CVE-2026-42198](https://nvd.nist.gov/vuln/detail/CVE-2026-42198) (SCRAM DoS) | High (7.5) | 2026-04-28 | — | — | — | Fix 42.7.11 post-cutoff. | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | fastapi + starlette (python — crud-be-python-fastapi) | fastapi 0.136.3 / starlette 1.2.1 | WAIVER | [CVE-2026-48710](https://nvd.nist.gov/vuln/detail/CVE-2026-48710) (BadHost host-header auth bypass) | Medium (6.5) | 2026-05-31 | 0.00034 | — | — | Starlette fix 1.0.1+ post-cutoff; fastapi 0.136.3 only requires starlette>=0.46 so starlette pinned directly to 1.2.1. | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | python-multipart (python) | 0.0.26 | WAIVER | [CVE-2026-40347](https://github.com/advisories/GHSA-mj87-hwqh-73pj) (DoS) | Medium (5.3) | 2026-04-10 | — | — | — | Fix 0.0.26 post-cutoff. | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | postgrex (hex — crud-be-elixir-phoenix) | 0.22.2 | WAIVER | [CVE-2026-32687](https://hex.pm/packages/postgrex/advisories) (SQL injection) | High (7.5) | 2026-05-12 | — | — | — | Fix 0.22.2 post-cutoff; all 0.16.0–0.22.1 vulnerable. | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | bandit (hex — crud-be-elixir-phoenix) | 1.11.1 | WAIVER | [CVE-2026-39804](https://cna.erlef.org/cves/CVE-2026-39804.html) + CVE-2026-39805/39807/42786/42788 (5 CVEs) | High | 2026-05-13 | — | — | — | All 5 CVEs fixed only in 1.11.0+ (post-cutoff). | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | plug (hex — crud-be-elixir-phoenix, added explicit pin) | 1.19.2 | WAIVER | [CVE-2026-8468](https://hex.pm/packages/plug/advisories) (multipart DoS) | High (8.2) | 2026-05-14 | — | — | — | Fix 1.19.2 post-cutoff; pinned explicitly (was transitive). | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | io.pedestal/pedestal.service + pedestal.jetty (clojure — crud-be-clojure-pedestal) | 0.8.1 | WAIVER | [CVE-2026-2332](https://nvd.nist.gov/vuln/detail/CVE-2026-2332) (Jetty 12 HTTP smuggling, transitive) | Critical (9.1) | 2025-10-27 | — | — | — | No stable Pedestal bundles a fully-patched Jetty; 0.8.1 (Jetty 12.0.29) is latest stable; residual transitive Jetty risk accepted pending Pedestal 0.8.2. | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | org.clojure/clojure (clojure — crud-be-clojure-pedestal) | 1.12.5 | WAIVER | — (no CVE; post-cutoff currency) | — | 2026-05-12 | — | — | — | 1.12.5 released post-cutoff (2026-05-12); pinned for currency, no CVE driver — trivial soak waiver. | Claude Opus 4.8 (AI agent) |
| 2026-06-04 | FluentAssertions (nuget — crud-be-csharp-aspnetcore) | 7.2.2 | FUNCTIONAL-HOLD | [skipped 8.x — license, not a CVE](https://www.infoq.com/news/2025/01/fluent-assertions-v8-license/) | — | 2026-03-16 | — | — | — | 8.x switched to the Xceed paid commercial license (Rule 5b functional defect for an MIT template); held at last Apache-2.0 release 7.2.2 (downgraded from the 8.3.0 that was in the repo). | Claude Opus 4.8 (AI agent) |

## Field reference

- **Date** — the bump date (`YYYY-MM-DD`) on which the waiver/hold was issued.
- **Package** — the dependency name, and ecosystem when ambiguous (e.g., `mermaid (npm)`).
- **Pinned Version** — the exact version pinned (no `^`/`~`).
- **Status** — `WAIVER` or `FUNCTIONAL-HOLD`, plus the `(KEV-listed)` suffix when applicable.
- **CVE(s) + URL** — the CVE ID(s) with an NVD or GHSA link; for `FUNCTIONAL-HOLD`, the
  release-blocker/advisory link and the skipped version.
- **Severity** — Critical / High / Medium / Low.
- **Release Date** — the release date of the pinned version.
- **EPSS** — the FIRST.org EPSS score (0–1) and percentile for the driving CVE, when CVSS ≥ 7.0.
- **KEV `dateAdded`** — the CISA KEV `dateAdded` field for KEV-listed CVEs.
- **KEV ransomware use** — the KEV `knownRansomwareCampaignUse` value (`Known` / `Unknown`).
- **Justification** — a brief reason (e.g., "Critical RCE; no older patched version exists").
- **Sign-off** — the engineer or AI agent identity that applied the waiver.

## References

- [Dependency Bump Stability & Safety Policy](../../repo-governance/development/workflow/dependency-bump-policy.md) — the authority that issues these statuses.
- [Repository Dependency Bump Planning Workflow](../../repo-governance/workflows/repo/repo-dependency-bump-planning.md) — propagates `WAIVER` / `FUNCTIONAL-HOLD` / `KEV-listed` entries here.
- [CISA KEV catalog](https://www.cisa.gov/known-exploited-vulnerabilities) — confirmed actively-exploited CVEs.
- [FIRST.org EPSS](https://www.first.org/epss) — exploitation-probability scoring.
