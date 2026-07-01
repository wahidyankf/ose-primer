---
title: "golang-commons Specs"
description: Gherkin behavioral specifications for the golang-commons shared Go utility library
category: specs
---

# golang-commons Specs

Gherkin behavioral specifications for [golang-commons](../../../libs/golang-commons/README.md),
the shared Go utility library for `ose-primer`'s Go CLI tools (currently `rhino-cli`).

## Purpose

These specs define the **observable behavior** of every exported `golang-commons` package
function: given a set of inputs, what the function returns or writes. They are the contract
between the library and every Go CLI that depends on it.

## Structure

```
specs/libs/golang-commons/
├── README.md
├── product/               # C4 L1 product framing
├── system-context/        # C4 L1 actors and consumers
├── containers/             # C4 L2 deployable units
├── components/             # C4 L3 component catalogue
└── behavior/
    └── gherkin/            # Gherkin feature files organized by package
        ├── testutil/
        └── timeutil/
```

## Spec Artifacts

- **[product/](./product/README.md)** — C4 Level 1 product framing
- **[system-context/](./system-context/README.md)** — C4 Level 1 actors and consumers
- **[containers/](./containers/README.md)** — C4 Level 2 deployable units
- **[components/](./components/README.md)** — C4 Level 3 component catalogue
- **[behavior/](./behavior/README.md)** — Gherkin feature files

## Running the Tests

```bash
nx run golang-commons:test:unit          # go test ./...
nx run golang-commons:test:integration   # godog-driven Gherkin scenarios
```

Every scenario is consumed at the integration level via a matching `*.integration_test.go` file
co-located with each package (`godog.TestSuite` reads the `.feature` files directly from
`specs/libs/golang-commons/behavior/gherkin/<package>/`).
