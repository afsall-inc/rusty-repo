# Architecture Overview

## Package Design

| Package | Role |
|---------|------|
| `cli` | Binary entrypoint — `rusty-repo` command with `new`, `init`, `templates` |

The CLI is intentionally minimal. It copies templates from `templates/` and replaces `{{project_name}}` and `{{author}}` placeholders.

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