# {{project_name}} — Agent Guide

## Agentic Loop

Start every session with:

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
```

## Architecture

Single crate in `app/`.

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
cargo run -- prdoc generate --pr 42
```

## Gotchas

- Always run `cargo fmt` after scaffolding
- Templates use `{{project_name}}` placeholders