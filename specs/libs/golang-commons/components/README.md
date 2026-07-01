---
title: "golang-commons — Components"
description: C4 Level 3 Component catalogue for golang-commons
category: specs
---

# Components — golang-commons

C4 Level 3 components for `golang-commons`.

| Package    | Export                                      | Purpose                                              |
| ---------- | ------------------------------------------- | ---------------------------------------------------- |
| `timeutil` | `Timestamp() string`                        | Current time in RFC3339 format                       |
| `timeutil` | `JakartaTimestamp() string`                 | Current time, ISO 8601, Asia/Jakarta (UTC+7)         |
| `testutil` | `CaptureStdout(t *testing.T) func() string` | Redirects and captures `os.Stdout` during a test run |

See [../behavior/gherkin/](../behavior/gherkin/) for the behavioral specs backing each package.
See [component-golang-commons.md](./component-golang-commons.md) for the C4 component diagram
placeholder.

## Related

- [golang-commons spec root](../README.md)
- [system-context/](../system-context/README.md) — C4 Level 1
- [containers/](../containers/README.md) — C4 Level 2
