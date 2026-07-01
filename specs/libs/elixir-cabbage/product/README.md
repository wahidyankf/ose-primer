---
title: "elixir-cabbage — Product"
description: C4 Level 1 product framing for elixir-cabbage
category: specs
---

# Product — elixir-cabbage

C4 Level 1 product framing for `elixir-cabbage`. See
[Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical layout.

## Overview

`elixir-cabbage` is a vendored, OSE-maintained fork of `cabbage-ex/cabbage` (v0.4.1, MIT). It is a
story BDD tool for Elixir: `use Cabbage.Feature, file: "some_feature.feature"` compiles a Gherkin
`.feature` file into ExUnit test cases at compile time, matching each `Given`/`When`/`Then` step
text against `defgiven`/`defwhen`/`defthen` macro clauses defined in the consuming test module.

See [overview.md](./overview.md) for the full product overview.

## Related

- [elixir-cabbage spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
