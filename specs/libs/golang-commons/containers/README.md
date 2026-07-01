---
title: "golang-commons — Containers"
description: C4 Level 2 Container diagram for golang-commons
category: specs
---

# Containers — golang-commons

C4 Level 2 containers for `golang-commons`.

`golang-commons` ships as a single container: a Go module
(`github.com/wahidyankf/ose-public/libs/golang-commons`) compiled into each consuming Go CLI
binary at build time. It has no separate deployable runtime of its own — it is a library module,
not a binary.

See [container.md](./container.md) for the C4 container diagram placeholder.

## Related

- [golang-commons spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [components/](../components/README.md) — C4 Level 3
