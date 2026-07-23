# Rusty-Repo — Agent Guide

This repo is a **template generator** for Rust projects. It provides a CLI (`rusty-repo`) to scaffold new Rust projects with CI/CD, PRDoc, and agentic infrastructure baked in.

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

Structured PR docs at `prdoc/`. Commands:

```bash
cargo run -- prdoc validate
cargo run -- prdoc show prdoc/pr_1.prdoc
cargo run -- prdoc generate --pr 42
cargo run -- changelog generate --from v0.1.0
```

## Skills

Composable workflows in `skills/<name>/skill.toml`. The `scaffold` skill provides a step-by-step guide for using rusty-repo to create new projects.

## How to Use This Template

1. **Create a new project**: `rusty-repo new my-project --template default`
2. **Or initialize in current dir**: `rusty-repo init`
3. **Customize templates**: Edit files under `templates/` to match your conventions
4. **Projects like montrs can be created** from the workspace template: `rusty-repo new montrs --template workspace`

## Gotchas

- Templates use `{{project_name}}` and `{{author}}` placeholders — these get replaced during scaffolding
- The CLI copies templates, so editing templates after scaffolding won't affect already-created projects
- Always run `cargo fmt` after scaffolding a new project