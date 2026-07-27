# Architecture Overview

## Package Design

| Package | Role |
|---------|------|
| `cli` | Binary entrypoint — `rusty-repo` command with `new`, `init`, `templates` |

The CLI is intentionally minimal. It copies templates from `templates/` and replaces `{{project_name}}` and `{{author}}` placeholders.

## Container Image

The `Dockerfile` uses a multi-stage build:
1. `rust:1.84-slim-bookworm` — compiles the binary
2. `gcr.io/distroless/cc-debian12` — minimal runtime image

The `.dockerignore` excludes source control, build artifacts, and docs from the build context.

On every version bump (detected from `Cargo.toml`), the CD workflow pushes to `ghcr.io/afsall-inc/rusty-repo` with `latest`, `{version}`, and `{major}.{minor}` tags.

## License

Dual-licensed under Apache-2.0 OR MIT.

## Template System

Templates live under `templates/`:

```
templates/
  default/          # Single crate project
  workspace/        # Multi-package workspace
  prdoc/            # PRDoc skeleton
  changelog/        # CHANGELOG.md template
```

Each template is a directory tree. The CLI copies it recursively, applying placeholder substitution to all files.

## Placeholder Substitution

| Placeholder | Replaced With |
|-------------|--------------|
| `{{project_name}}` | The project name argument |
| `{{author}}` | `Afsall` (configurable) |

## CLI Internals

The CLI (`packages/cli/src/bin/rusty_repo.rs`) uses `clap` for argument parsing with three commands:

- **`new`** — Scaffolds a new project in a specified directory
- **`init`** — Scaffolds in the current directory
- **`templates`** — Lists available templates

Template resolution checks multiple paths relative to the binary and the working directory, making it work both during development (`cargo run`) and after installation.