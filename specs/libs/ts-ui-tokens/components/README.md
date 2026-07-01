---
title: "ts-ui-tokens — Components"
description: C4 Level 3 Component catalogue for ts-ui-tokens
category: specs
---

# Components — ts-ui-tokens

C4 Level 3 components for `ts-ui-tokens`.

| Module          | Export        | Purpose                                                                    |
| --------------- | ------------- | -------------------------------------------------------------------------- |
| `colors.ts`     | `colorTokens` | Semantic color token names (`--color-background`, `--color-primary`, etc.) |
| `radius.ts`     | `radius`      | Border-radius scale (`DEFAULT`, `lg`, `md`, `sm`)                          |
| `spacing.ts`    | `spacing`     | 4pt spacing scale (keys `1`–`16`, mapped to Tailwind `p-*`/`m-*`/`gap-*`)  |
| `typography.ts` | `typography`  | Type scale (`xs`–`4xl`, each a `{ fontSize, lineHeight }` pair)            |
| `tokens.css`    | (CSS import)  | Tailwind v4 `@theme` custom properties + `@custom-variant dark`            |

See [../behavior/gherkin/tokens/](../behavior/gherkin/tokens/) for the behavioral spec.
See [component-ts-ui-tokens.md](./component-ts-ui-tokens.md) for the C4 component diagram
placeholder.

## Related

- [ts-ui-tokens spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [containers/](../containers/README.md) — C4 Level 2
