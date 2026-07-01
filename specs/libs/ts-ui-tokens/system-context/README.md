---
title: "ts-ui-tokens — System Context"
description: C4 Level 1 System Context diagram for ts-ui-tokens
category: specs
---

# System Context — ts-ui-tokens

C4 Level 1 system context for `ts-ui-tokens`.

## Actors and consumers

- **`ts-ui`** — imports `colorTokens`, `radius`, `spacing`, `typography` and depends on
  `tokens.css` being imported upstream by the consuming app.
- **Every TypeScript frontend app** — imports `tokens.css` in its `globals.css` and layers
  brand-specific overrides (primary/accent colors) on top via Tailwind v4 `@theme`.
- **Storybook** (via `ts-ui`) — renders components against the structural token set.

`ts-ui-tokens` has no runtime dependency on any backend; it is a pure CSS/TypeScript constants
package with zero external dependencies.

See [context.md](./context.md) for the C4 context diagram placeholder.

## Related

- [ts-ui-tokens spec root](../README.md)
- [containers/](../containers/README.md) — C4 Level 2
- [components/](../components/README.md) — C4 Level 3
