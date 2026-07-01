---
title: "golang-commons — System Context"
description: C4 Level 1 System Context diagram for golang-commons
category: specs
---

# System Context — golang-commons

C4 Level 1 system context for `golang-commons`.

## Actors and consumers

- **`rhino-cli`** — the Rust repository-governance CLI does not import `golang-commons` directly
  (it is a Rust binary); `golang-commons` instead exists to serve **Go** CLI tooling in this
  workspace and sibling OSE-family repos that build Go binaries.
- **Go CLI test suites** — `*.integration_test.go` files under `libs/golang-commons/*/` import
  `github.com/cucumber/godog` and drive the Gherkin scenarios in this spec tree directly against
  `timeutil` and `testutil`.

`golang-commons` has no runtime dependency on any backend or network service; every exported
function operates on local process state (the clock, `os.Stdout`).

See [context.md](./context.md) for the C4 context diagram placeholder.

## Related

- [golang-commons spec root](../README.md)
- [containers/](../containers/README.md) — C4 Level 2
- [components/](../components/README.md) — C4 Level 3
