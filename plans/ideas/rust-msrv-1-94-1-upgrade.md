# Upgrade Rust MSRV to 1.94.1

One-line summary: bump the repository's minimum-supported Rust version to 1.94.1 to pick up the
`CVE-2026-33056` fix in Cargo's tar handling, once the toolchain ships that version.

> Surfaced during `update-toolchain-versions` plan execution (deferred; original capture undated).

## Problem / context

`CVE-2026-33056` affects Cargo's tar handling and is fixed in Rust 1.94.1. The
`update-toolchain-versions` plan wanted to adopt the fix but could not: the developer toolchain at the
time pinned `rustc 1.94.0`, exactly one patch below the fixed release, so the MSRV bump was deferred
rather than landed. Nothing in the repo currently pulls in the patched Cargo, so the known CVE remains
in the pinned toolchain until the MSRV is raised.

## Why now

This is a **time-gated** deferral, not a rejected idea: it is blocked only on Rust 1.94.1+ becoming
available in the developer toolchain (`rustup update stable`). The moment that patch is installable,
the fix is a one-line version bump away, and leaving a CVE-carrying toolchain pinned longer than
necessary is avoidable exposure.

## Prior art / precedents

- **rustup** — the toolchain manager whose `rustup update stable` yielding 1.94.1+ is the sole blocker
  for this bump. [rustup](https://rust-lang.github.io/rustup/)
- **Cargo `rust-version` (MSRV) field** — the manifest mechanism for declaring the minimum supported
  Rust version this idea raises.
  [manifest](https://doc.rust-lang.org/cargo/reference/manifest.html)
- **RustSec Advisory Database** — the ecosystem's tracker for Rust security advisories that motivate
  toolchain bumps like this one. [rustsec.org](https://rustsec.org/)
- **Dependency Bump Stability & Safety Policy (repo-internal)** — the exact-pin, CVE-clean policy this
  bump must follow.
  [dependency-bump-policy](../../repo-governance/development/workflow/dependency-bump-policy.md)

## Proposed direction (sketch)

- Once `rustc 1.94.1+` is available via `rustup update stable`, raise the pinned Rust version across
  the toolchain pin(s) and any MSRV declarations.
- Run the full affected build/test/lint gate to confirm no 1.94.1 behavior change breaks the crates.
- Keep the bump exact-pinned per the dependency-bump policy.

## Rough scope & non-goals

In scope: raising the pinned Rust version to 1.94.1 (or the first available patch ≥ 1.94.1) and
re-running the affected gates.

Out of scope (for now): a broader Rust edition change; adopting new 1.94.x language features; any
unrelated Cargo dependency bumps riding along on the same change.

## Risks & open questions

- Is 1.94.1 (or later) yet published and installable in the developer toolchain? (open — the blocker)
- Does 1.94.1 introduce any lint/behavior change that trips the strict warning-and-above gates on the
  existing crates? (open — needs a build once the toolchain is available)

## What success looks like + promotion signal

Success: the pinned toolchain is on Rust ≥ 1.94.1, so `CVE-2026-33056` is no longer present in the
Cargo shipped with the repo's Rust, and all affected gates stay green. Ready to promote to a
`backlog/` plan the moment `rustup update stable` yields 1.94.1+ — the change itself is small and
well-understood; only availability blocks it.
