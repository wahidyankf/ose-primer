# Backlog Plans

Planned projects for future implementation.

## Planned Projects

- **[Primer Polyglot Demo-App CI Restoration](./2026-06-19__primer-polyglot-codegen-ci-restoration/)** —
  Restore fresh-checkout codegen + green per-language CI for the `crud-*` demo apps. The .NET SQLite CVE
  (`NU1903`) is already fixed; Dart (dormant `rhino-cli specs scaffold dart`), Rust (nx manifest
  generation), and Go (`oapi-codegen` vs OpenAPI 3.1) need fresh-codegen fixes; Elixir CI deps failure to
  verify/transient. Surfaced when the corrected per-language matrix first built the demo apps fresh.

## Instructions

**Quick Idea Capture**: For 1-3 liner ideas not ready for formal planning, use `../ideas.md`.

When creating a new plan:

1. Create folder: `YYYY-MM-DD__[project-identifier]/`
2. Add standard files: README.md, requirements.md, tech-docs.md, delivery.md
3. Add the plan to this list
