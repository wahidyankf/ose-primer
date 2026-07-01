---
title: "ts-ui-tokens Specs"
description: Gherkin behavioral specifications for the ts-ui-tokens shared design token library
category: specs
---

# ts-ui-tokens Specs

Gherkin behavioral specifications for
[ts-ui-tokens](../../../libs/ts-ui-tokens/README.md), the shared structural design token library.

## Purpose

These specs define the **observable behavior** of the `ts-ui-tokens` package: which structural
token modules (`colorTokens`, `radius`, `spacing`, `typography`) it exports, so `ts-ui` and every
consuming app's `globals.css` can rely on a consistent, versioned token surface.

## Structure

```
specs/libs/ts-ui-tokens/
├── README.md
├── product/               # C4 L1 product framing
├── system-context/        # C4 L1 actors and consumers
├── containers/             # C4 L2 deployable units
├── components/             # C4 L3 component catalogue
└── behavior/
    └── gherkin/            # Gherkin feature files
        └── tokens/
```

## Spec Artifacts

- **[product/](./product/README.md)** — C4 Level 1 product framing
- **[system-context/](./system-context/README.md)** — C4 Level 1 actors and consumers
- **[containers/](./containers/README.md)** — C4 Level 2 deployable units
- **[components/](./components/README.md)** — C4 Level 3 component catalogue
- **[behavior/](./behavior/README.md)** — Gherkin feature files

## Status

`test:unit` (`nx run ts-ui-tokens:test:unit`) is currently an `echo` placeholder — `ts-ui-tokens`
is CSS plus minimal TypeScript re-exports with no dedicated test runner yet. `specs:behavior:coverage`
is likewise an `echo` placeholder until a runner is wired up. The scenario below documents the
intended token-export contract that `ts-ui` (its only consumer) already relies on at compile time.
