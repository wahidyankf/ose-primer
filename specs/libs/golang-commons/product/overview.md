# golang-commons — Product Overview

`golang-commons` provides common Go packages consumed by this repository's Go CLI applications
(currently `rhino-cli`). Two packages exist today:

- **`timeutil`** — `Timestamp() string` returns the current time in RFC3339 format;
  `JakartaTimestamp() string` returns the current time as ISO 8601 in the Asia/Jakarta timezone
  (UTC+7).
- **`testutil`** — `CaptureStdout(t *testing.T) func() string` redirects `os.Stdout` to a pipe for
  the duration of a test and returns a function that restores stdout and returns whatever was
  captured, so Go CLI tests can assert on printed output without wiring their own pipe plumbing.

See [README.md](./README.md) for C4 L1 product framing.
