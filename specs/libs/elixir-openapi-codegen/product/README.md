---
title: "elixir-openapi-codegen — Product"
description: C4 Level 1 product framing for elixir-openapi-codegen
category: specs
---

# Product — elixir-openapi-codegen

C4 Level 1 product framing for `elixir-openapi-codegen`. See
[Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical layout.

## Overview

`elixir-openapi-codegen` generates Elixir struct modules from the `components.schemas` section of
a bundled OpenAPI 3.x YAML spec, so Elixir backends in this workspace can consume a shared,
contract-generated set of request/response types instead of hand-writing them.

See [overview.md](./overview.md) for the full product overview.

## Related

- [elixir-openapi-codegen spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
