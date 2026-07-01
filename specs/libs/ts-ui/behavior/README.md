---
title: "ts-ui — Behavior"
description: Index of behavioral specifications for ts-ui
category: specs
---

# Behavior — ts-ui

Gherkin behavioral specifications for [ts-ui](../../../../libs/ts-ui/README.md), the shared React
component library.

## Structure

Feature files live under `behavior/gherkin/<component>/`, one folder per component:

```
specs/libs/ts-ui/behavior/
└── gherkin/
    ├── alert/
    │   └── alert.feature
    ├── button/
    │   └── button.feature
    ├── card/
    │   └── card.feature
    ├── dialog/
    │   └── dialog.feature
    ├── input/
    │   └── input.feature
    └── label/
        └── label.feature
```

## Running the tests

```bash
nx run ts-ui:test:unit
nx run ts-ui:specs:behavior:coverage
```

Every scenario is consumed at the unit level via the matching `*.steps.tsx` file co-located with
each component under `libs/ts-ui/src/components/`. Coverage between these feature files and their
step implementations is enforced by
`cargo run -- specs behavior-coverage validate specs/libs/ts-ui/behavior/gherkin libs/ts-ui`.

## Related

- [ts-ui spec root](../README.md)
- [components/](../components/README.md) — C4 Level 3 component catalogue
