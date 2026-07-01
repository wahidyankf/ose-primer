---
title: "ts-ui Specs"
description: Gherkin behavioral specifications for the ts-ui shared React component library
category: specs
---

# ts-ui Specs

Gherkin behavioral specifications for [ts-ui](../../../libs/ts-ui/README.md), the shared React
component library.

## Purpose

These specs define the **observable behavior** of every `ts-ui` component: what a user sees and
can do, and what accessibility contract each component honors. They are the shared contract
between design, development, and QA.

## Structure

```
specs/libs/ts-ui/
├── README.md
├── product/               # C4 L1 product framing
├── system-context/        # C4 L1 actors and consumers
├── containers/             # C4 L2 deployable units
├── components/             # C4 L3 component catalogue
└── behavior/
    └── gherkin/            # Gherkin feature files organized by component
        ├── alert/
        ├── button/
        ├── card/
        ├── dialog/
        ├── input/
        └── label/
```

## Spec Artifacts

- **[product/](./product/README.md)** — C4 Level 1 product framing
- **[system-context/](./system-context/README.md)** — C4 Level 1 actors and consumers
- **[containers/](./containers/README.md)** — C4 Level 2 deployable units
- **[components/](./components/README.md)** — C4 Level 3 component catalogue
- **[behavior/](./behavior/README.md)** — Gherkin feature files

## Running the Tests

```bash
nx run ts-ui:test:unit
```

Every scenario is consumed at the unit level via the matching `*.steps.tsx` file co-located with
each component under `libs/ts-ui/src/components/`.
