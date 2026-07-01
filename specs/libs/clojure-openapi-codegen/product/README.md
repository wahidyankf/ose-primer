---
title: "clojure-openapi-codegen — Product"
description: C4 Level 1 product framing for clojure-openapi-codegen
category: specs
---

# Product — clojure-openapi-codegen

C4 Level 1 product framing for `clojure-openapi-codegen`. See
[Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical layout.

## Overview

`clojure-openapi-codegen` generates [Malli](https://github.com/metosin/malli) schema definition
files from the `components.schemas` section of a bundled OpenAPI 3.x YAML spec, so Clojure
backends in this workspace can validate and coerce request/response data against a shared,
contract-generated schema instead of hand-writing Malli schemas.

See [overview.md](./overview.md) for the full product overview.

## Related

- [clojure-openapi-codegen spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
