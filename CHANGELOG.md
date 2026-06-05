# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] — 2026-06-06

Initial release.

### Added

- **`hooksmith init`** — installs a POSIX shell hook script into `.git/hooks/` for every hook defined in `.hooksmith.toml`. Each script delegates to `hooksmith run <hook>` so the real command list stays in the tracked config file.
- **`hooksmith add <hook> <command>`** — appends a command to a named hook in `.hooksmith.toml` and immediately installs the corresponding hook script. Creates the config file if it does not yet exist.
- **`hooksmith run <hook>`** — executes all commands registered for a hook sequentially. Stops at the first failure and exits with that command's exit code, causing Git to abort the operation.
- **`hooksmith status`** — prints the full list of configured hooks and their commands as read from `.hooksmith.toml`.
- **`.hooksmith.toml` config format** — TOML-based configuration at the repository root. Supports multiple hooks, each with an ordered list of shell commands.
- **Supported hooks** — `pre-commit`, `commit-msg`, `post-commit`, `pre-push`.
- **Exit code propagation** — when a hook command fails, hooksmith exits with that command's exact exit code rather than a generic `1`, enabling callers to distinguish failure types.
- **Cross-platform hook installation** — hook scripts are written as `#!/bin/sh` for compatibility across Unix-like systems; executable permissions (`0o755`) are set automatically on Unix. Windows support handled separately via the CI matrix.
- **CI/CD release pipeline** — GitHub Actions workflow that builds release binaries for `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, and `x86_64-pc-windows-msvc` on every version tag push (`v*.*.*`), packages each binary into a `.tar.gz` (Unix) or `.zip` (Windows) archive, and publishes a GitHub release with auto-generated release notes.
