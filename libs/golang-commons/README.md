# golang-commons

Shared Go utilities for `ose-primer` Go CLI tools.

> **Module path note**: this library's Go module path remains `github.com/wahidyankf/ose-public/libs/golang-commons` because the package was originally published from the [`ose-public`](https://github.com/wahidyankf/ose-public) sibling repository and downstream consumers may already import it under that path. The module path is intentionally not renamed to `ose-primer` to preserve import compatibility.

## Purpose

Provides common Go packages used across this repository's Go CLI applications (currently `rhino-cli`) and is portable to other OSE-family repos via its stable module path.

## Packages

### `timeutil`

Timestamp utilities shared across Go CLI tools and libraries.

**Import path**: `github.com/wahidyankf/ose-public/libs/golang-commons/timeutil`

**Exports**:

- `Timestamp() string` — current time in RFC3339 format
- `JakartaTimestamp() string` — current time as ISO 8601 in the Asia/Jakarta timezone (UTC+7)

### `testutil`

Testing utilities for Go CLI tools.

**Import path**: `github.com/wahidyankf/ose-public/libs/golang-commons/testutil`

**Exports**:

- `CaptureStdout(t *testing.T) func() string` — redirects stdout to a pipe; call the returned function to restore stdout and retrieve captured output

### `links`

Link-checking utilities for Go CLI tools.

**Import path**: `github.com/wahidyankf/ose-public/libs/golang-commons/links`

**Exports**:

- `BrokenLink` — broken link representation (source file, line, text, target)
- `CheckResult` — aggregate result (checked count, error count, errors, broken links)
- `CheckLinks(contentDir string) (*CheckResult, error)` — walks all `.md` files and validates internal links
- `OutputLinksText(result, elapsed, quiet, verbose)` — human-readable stdout report
- `OutputLinksJSON(result, elapsed) error` — JSON stdout report
- `OutputLinksMarkdown(result, elapsed)` — Markdown stdout report

## Usage

```go
import "github.com/wahidyankf/ose-public/libs/golang-commons/links"

if err != nil {
    return err
}
links.OutputLinksText(result, elapsed, quiet, verbose)
```

## Commands

```bash
# Run tests
nx run golang-commons:test:quick

# Lint
nx run golang-commons:lint

# Tidy dependencies
nx run golang-commons:install
```
