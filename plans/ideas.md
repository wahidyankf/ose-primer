# Ideas

Quick ideas and todos that haven't been formalized into plans yet.

When an idea is ready for implementation, create a proper plan folder in `backlog/` and remove it from this list.

## Ideas List

- **Upgrade Rust MSRV to 1.94.1** (fixes CVE-2026-33056 in Cargo tar handling): deferred from
  `update-toolchain-versions` plan because local rustc was 1.94.0. Upgrade when Rust 1.94.1+ is
  available in the developer toolchain (`rustup update stable`).
