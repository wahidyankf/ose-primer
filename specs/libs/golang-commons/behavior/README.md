---
title: "golang-commons — Behavior"
description: Index of behavioral specifications for golang-commons
category: specs
---

# Behavior — golang-commons

Gherkin behavioral specifications for
[golang-commons](../../../../libs/golang-commons/README.md), the shared Go utility library.

## Structure

Feature files live under `behavior/gherkin/<package>/`, one folder per package:

```
specs/libs/golang-commons/behavior/
└── gherkin/
    ├── testutil/
    │   └── capture-stdout.feature
    └── timeutil/
        └── timestamp.feature
```

## Running the tests

```bash
nx run golang-commons:test:integration
```

Every scenario is consumed at the integration level via a matching `*.integration_test.go` file
co-located with each package (`libs/golang-commons/testutil/capture-stdout.integration_test.go`,
`libs/golang-commons/timeutil/timestamp.integration_test.go`). Each file wires a `godog.TestSuite`
that reads its package's `.feature` files directly from this directory and matches step text
against `godog.ScenarioContext.Step` registrations.

## Status

`specs:behavior:coverage` is currently a stubbed `echo` placeholder (Phase 0) — automated
`@covers-tag` gap detection between these scenarios and the `godog` step implementations lands in
a later phase. Until then, coverage is verified manually by running
`nx run golang-commons:test:integration` and confirming every scenario passes.

## Related

- [golang-commons spec root](../README.md)
- [components/](../components/README.md) — C4 Level 3 component catalogue
