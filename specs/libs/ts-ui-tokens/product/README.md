---
title: "ts-ui-tokens — Product"
description: C4 Level 1 product framing for ts-ui-tokens
category: specs
---

# Product — ts-ui-tokens

C4 Level 1 product framing for `ts-ui-tokens`. See
[Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical layout.

## Overview

`ts-ui-tokens` is the shared structural design token library for the `ose-primer` monorepo. It
ships both a CSS entry point (`tokens.css`, imported by every app's `globals.css`) and a
TypeScript entry point (`colorTokens`, `radius`, `spacing`, `typography`) for programmatic token
access, consumed by `ts-ui` and directly by apps that need token values in JavaScript/TypeScript
logic rather than CSS classes.

See [overview.md](./overview.md) for the full product overview.

## Related

- [ts-ui-tokens spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
