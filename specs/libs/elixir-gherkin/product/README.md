---
title: "elixir-gherkin — Product"
description: C4 Level 1 product framing for elixir-gherkin
category: specs
---

# Product — elixir-gherkin

C4 Level 1 product framing for `elixir-gherkin`. See
[Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical layout.

## Overview

`elixir-gherkin` is a vendored, OSE-maintained fork of `cabbage-ex/gherkin` (v2.0.0, MIT). It
parses `.feature` files (binary or streamed) into native Elixir structs
(`Gherkin.Elements.Feature`, `Scenario`, `ScenarioOutline`, `Step`) and is the sole parsing
dependency of [`elixir-cabbage`](../../elixir-cabbage/README.md).

See [overview.md](./overview.md) for the full product overview.

## Related

- [elixir-gherkin spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
