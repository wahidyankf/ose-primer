---
title: "ts-ui — Containers"
description: C4 Level 2 Container diagram for ts-ui
category: specs
---

# Containers — ts-ui

C4 Level 2 containers for `ts-ui`.

`ts-ui` ships as a single container: a TypeScript/React component package
(`@open-sharia-enterprise/ts-ui`) built with Tailwind CSS and Radix UI primitives, consumed at
build time by every frontend app in the workspace. It has no separate deployable runtime of its
own — Storybook (`nx run ts-ui:storybook`) is a development-time preview container, not a
production deployable.

See [container.md](./container.md) for the C4 container diagram placeholder.

## Related

- [ts-ui spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [components/](../components/README.md) — C4 Level 3
