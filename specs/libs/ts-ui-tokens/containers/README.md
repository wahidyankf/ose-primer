---
title: "ts-ui-tokens — Containers"
description: C4 Level 2 Container diagram for ts-ui-tokens
category: specs
---

# Containers — ts-ui-tokens

C4 Level 2 containers for `ts-ui-tokens`.

`ts-ui-tokens` ships as a single container: a TypeScript/CSS package
(`@open-sharia-enterprise/ts-ui-tokens`) consumed at build time by `ts-ui` and directly by every
frontend app's `globals.css`. It has no separate deployable runtime of its own — it contains no
executable entry point, only re-exported constants and a CSS stylesheet.

See [container.md](./container.md) for the C4 container diagram placeholder.

## Related

- [ts-ui-tokens spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [components/](../components/README.md) — C4 Level 3
