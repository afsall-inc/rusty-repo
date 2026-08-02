# Rusty-Repo — Agent Guide

This repo is a **template generator** for Rust projects. It provides a CLI (`rusty-repo`) to scaffold new Rust projects with CI/CD, PRDoc, and agentic infrastructure baked in. Published to [crates.io](https://crates.io/crates/rusty-repo-cli) and [ghcr.io](https://github.com/afsall-inc/rusty-repo/pkgs/container/rusty-repo).

## Agentic Loop

Start every session with:

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
```

## Architecture

| Package | Role |
|---------|------|
| `cli` | Binary entrypoint (`rusty-repo` command) — scaffold, init, generate |

## Toolchain

- **Rust**: `nightly-2026-02-18` (pinned in `rust-toolchain.toml`)
- **Cargo**: edition 2024, resolver "2"

## Developer Commands

```bash
mise run ci       # fmt → clippy → test
mise run fmt      # cargo fmt --all
mise run clippy   # cargo clippy --workspace -- -D warnings
mise run test     # cargo test --workspace
mise run build    # cargo build --workspace
```

**Required order** (CI enforces): `fmt (--check)` → `clippy -D warnings` → `test`

## Templates

Templates in `templates/` are used by `rusty-repo new`:

| Template | Use |
|----------|-----|
| `default/` | Single crate project with CI/CD, AGENTS.md, PRDoc |
| `workspace/` | Multi-package workspace with apps/ and packages/ |
| `prdoc/` | PRDoc template for structured PR docs |

## Casing

| Item | Convention | Example |
|------|-----------|---------|
| Rust vars | snake_case | `project_name` |
| Files | kebab-case | `ci.yml` |
| Types | PascalCase | `ProjectConfig` |

## Testing

- `cargo test --workspace` for unit/integration tests
- All tests must be hermetic, deterministic, and isolated

## PRDoc

Structured PR docs at `prdoc/`. Requires [changelogger-cli](https://crates.io/crates/changelogger-cli) v0.2.1:

```bash
cargo install changelogger-cli --version 0.2.1
changelogger prdoc validate
changelogger prdoc show prdoc/pr_1.prdoc
changelogger prdoc generate --pr 42
changelogger changelog generate --from v0.1.0
```

## Documentation

- `docs/index.md` — Documentation index
- `docs/getting-started/overview.md` — Installation and usage
- `docs/architecture/overview.md` — Package design and template system
- `docs/contributor/guide.md` — Extending rusty-repo

## CI/CD

| Workflow | Trigger | Action |
|----------|---------|--------|
| `ci.yml` | Push/PR to `main` | fmt → clippy → test → build |
| `prdoc.yml` | PR opened/synchronized | auto-generate prdoc, validate, commit back |
| `command-prdoc.yml` | workflow_dispatch | manual prdoc generation with bump/audience inputs |
| `cd.yml` | Merge to `main` with version bump | Generate CHANGELOG.md + Build & push Docker image to ghcr.io |
| `release.yml` | Tag push `v*` | GitHub Release + Docker image to ghcr.io |

## Skills

Composable workflows in `skills/<name>/skill.toml`. The `scaffold` skill provides a step-by-step guide for using rusty-repo to create new projects.

## How to Use This Template

1. **Create a new project**: `rusty-repo new my-project --template default`
2. **Or initialize in current dir**: `rusty-repo init`
3. **Customize templates**: Edit files under `templates/` to match your conventions
4. **Projects like montrs can be created** from the workspace template: `rusty-repo new montrs --template workspace`

## Container Image

A Docker image is published to `ghcr.io/afsall-inc/rusty-repo` on every version bump:

- **CD**: On merge to `main` when `Cargo.toml` version changes — image is pushed with `latest`, `{version}`, and `{major}.{minor}` tags
- **Release**: On `v*` tag push — image is pushed with `{version}` and `latest` tags

Usage:
```bash
docker pull ghcr.io/afsall-inc/rusty-repo:latest
docker run --rm ghcr.io/afsall-inc/rusty-repo:latest new my-project
```

## License

Apache-2.0 OR MIT

## Gotchas

- Templates use `{{project_name}}` and `{{author}}` placeholders — these get replaced during scaffolding
- The CLI copies templates, so editing templates after scaffolding won't affect already-created projects
- Always run `cargo fmt` after scaffolding a new project