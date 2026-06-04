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

_No waivers recorded yet._

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
