---
title: "golang-commons — Product"
description: C4 Level 1 product framing for golang-commons
category: specs
---

# Product — golang-commons

C4 Level 1 product framing for `golang-commons`. See
[Specs Directory Structure Convention](../../../../repo-governance/conventions/structure/specs-directory-structure.md)
for the canonical layout.

## Overview

`golang-commons` is the shared Go utility library for `ose-primer`'s Go CLI tools. Its module
path (`github.com/wahidyankf/ose-public/libs/golang-commons`) is intentionally retained from the
sibling `ose-public` repository this library was extracted from, so it can be imported by
downstream OSE-family repos without a breaking import-path change.

See [overview.md](./overview.md) for the full product overview.

## Related

- [golang-commons spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
