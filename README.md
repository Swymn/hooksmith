# hooksmith

A minimal, dependency-light Git hooks manager written in Rust. Define your hook commands once in `.hooksmith.toml`, commit the file, and let every contributor install the same hooks with a single command.

## Why hooksmith?

Git stores hooks in `.git/hooks/`, which is never tracked by version control. This means every developer on a project must manually configure their own hooks, and teams have no reliable way to enforce consistent pre-commit checks or other automation.

hooksmith solves this by separating the *configuration* (`.hooksmith.toml`, which is committed) from the *installation* (`.git/hooks/`, which is local). The installed hooks are thin shell scripts that delegate execution back to hooksmith, keeping the real logic in the tracked config.

## Installation

### From source

Requires the Rust toolchain (stable).

```sh
cargo install --path .
```

### From release binaries

Pre-built binaries for Linux (x86_64), macOS (x86_64 and Apple Silicon), and Windows (x86_64) are attached to each [GitHub release](../../releases). Download the archive for your platform, extract it, and place the `hooksmith` binary somewhere on your `PATH`.

```sh
# Example for macOS Apple Silicon
tar -xzf hooksmith-v0.1.0-aarch64-apple-darwin.tar.gz
mv hooksmith /usr/local/bin/
```

## Quick start

```sh
# 1. Add a command to the pre-commit hook (creates .hooksmith.toml if absent)
hooksmith add pre-commit "cargo fmt --check"
hooksmith add pre-commit "cargo test"

# 2. Install the hooks into .git/hooks/ so Git triggers them
hooksmith init

# 3. Commit .hooksmith.toml so teammates can reproduce your setup
git add .hooksmith.toml
git commit -m "chore: add hooksmith config"

# On any other machine, after cloning:
hooksmith init
```

## Commands

### `hooksmith init`

Reads `.hooksmith.toml` and installs a Git hook script for every hook that has at least one command configured. Each installed script is a POSIX shell script that calls `hooksmith run <hook-name>` when Git fires that hook.

Run this after cloning a repository that already has a `.hooksmith.toml`, or after adding a new hook entry via `hooksmith add`.

```sh
hooksmith init
# ✓ Hook 'pre-commit' installed.
# ✓ Hooksmith initialized!
```

### `hooksmith add <hook> <command>`

Appends a shell command to the named hook's command list in `.hooksmith.toml`, then installs (or reinstalls) that hook's script in `.git/hooks/`. Creates `.hooksmith.toml` if it does not yet exist.

```sh
hooksmith add pre-commit "cargo clippy -- -D warnings"
hooksmith add pre-push "cargo test --release"
hooksmith add commit-msg "scripts/validate-message.sh"
```

**Supported hooks**

| Hook | When Git fires it |
|---|---|
| `pre-commit` | Before a commit is recorded, after staging |
| `commit-msg` | After the commit message is written |
| `post-commit` | After a commit is successfully created |
| `pre-push` | Before `git push` transfers objects to the remote |

Passing an unsupported hook name is an error.

### `hooksmith run <hook>`

Executes every command registered for the named hook in the order they appear in `.hooksmith.toml`. Commands run sequentially via `sh -c`; if any command exits with a non-zero status, hooksmith stops immediately and exits with the same code. This is the command that the installed hook scripts call internally — you rarely invoke it by hand.

```sh
hooksmith run pre-commit
# ▶ cargo fmt --check
# ▶ cargo test
```

If the named hook has no configuration, hooksmith exits successfully without doing anything.

### `hooksmith status`

Prints every configured hook and its command list, read directly from `.hooksmith.toml`. Useful for a quick audit of what will run before each Git operation.

```sh
hooksmith status
# [pre-commit]
#         - cargo fmt --check
#         - cargo test
```

## Configuration

hooksmith reads and writes `.hooksmith.toml` at the root of the Git repository. You can edit this file by hand or manage it exclusively through `hooksmith add`.

```toml
[hooks.pre-commit]
commands = [
    "cargo fmt --check",
    "cargo clippy -- -D warnings",
    "cargo test",
]

[hooks.pre-push]
commands = [
    "cargo test --release",
]
```

The file must be committed to version control. `.git/hooks/` must not be committed (it is ignored by Git by default).

## Exit codes

| Code | Meaning |
|---|---|
| `0` | All commands succeeded (or no commands were configured) |
| `1` | hooksmith internal error (no Git repo, malformed config, I/O failure) |
| *n* | The failing hook command exited with code *n* |

When a hook command fails, hooksmith propagates its exact exit code. This lets Git correctly abort the operation (commit, push, etc.) and lets CI distinguish between a lint failure and a hooksmith configuration error.
