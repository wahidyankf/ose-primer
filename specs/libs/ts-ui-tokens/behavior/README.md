---
title: "ts-ui-tokens — Behavior"
description: Index of behavioral specifications for ts-ui-tokens
category: specs
---

# Behavior — ts-ui-tokens

Gherkin behavioral specifications for
[ts-ui-tokens](../../../../libs/ts-ui-tokens/README.md), the shared structural design token
library.

## Structure

```
specs/libs/ts-ui-tokens/behavior/
└── gherkin/
    └── tokens/
        └── tokens-export.feature
```

## Status

No test runner currently consumes these scenarios — `specs:behavior:coverage` is an `echo`
placeholder (see the top-level [README.md](../README.md#status)). The scenario documents the
token-export contract that `ts-ui` already relies on at compile time via
`import { colorTokens, radius, spacing, typography } from "@open-sharia-enterprise/ts-ui-tokens"`.

## Related

- [ts-ui-tokens spec root](../README.md)
- [components/](../components/README.md) — C4 Level 3 component catalogue
