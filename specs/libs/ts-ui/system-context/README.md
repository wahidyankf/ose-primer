---
title: "ts-ui — System Context"
description: C4 Level 1 System Context diagram for ts-ui
category: specs
---

# System Context — ts-ui

C4 Level 1 system context for `ts-ui`.

## Actors and consumers

- **Frontend developers** — import components directly from `@open-sharia-enterprise/ts-ui`.
- **Consuming apps** — every TypeScript frontend that renders shared UI in this workspace.
- **Storybook** — hosts an isolated visual catalogue of every component for manual review
  (`nx run ts-ui:storybook`).
- **`ts-ui-tokens`** — upstream design-token dependency; `ts-ui` consumes its color, radius,
  spacing, and typography tokens rather than hardcoding values.

`ts-ui` has no runtime dependency on any backend; it is a pure presentation-layer library.

See [context.md](./context.md) for the C4 context diagram placeholder.

## Related

- [ts-ui spec root](../README.md)
- [containers/](../containers/README.md) — C4 Level 2
- [components/](../components/README.md) — C4 Level 3
