# ts-ui-tokens — Product Overview

`ts-ui-tokens` provides two customization layers consumed across the workspace:

- **Structural tokens** (this package) — border-radius scale, base neutral palette, semantic
  colors (muted, destructive), 4pt spacing scale, and a typography scale, consistent across every
  app.
- **Brand tokens** (overridden per-app) — primary/secondary/accent colors and app-specific tokens
  layered on top in each app's `globals.css`.

The package exports both a CSS entry point (`src/tokens.css`, imported via
`@import "@open-sharia-enterprise/ts-ui-tokens/src/tokens.css";`) and a TypeScript entry point
(`colorTokens`, `radius`, `spacing`, `typography` from `src/index.ts`) so token values are
available both as Tailwind v4 CSS custom properties and as typed JavaScript/TypeScript constants.

> **Package name note**: the npm package is published as `@open-sharia-enterprise/ts-ui-tokens`,
> matching the upstream `ose-public` repository this library was extracted from, for binary
> compatibility across the OSE family.

See [README.md](./README.md) for C4 L1 product framing.
