# Contributor Guide

## Extending rusty-repo

### Adding a new template

1. Create a directory under `templates/<name>/`
2. Add the project files with `{{project_name}}` placeholders
3. Register it in the CLI if needed (currently auto-discovers)

### Template conventions

- All templates should include `AGENTS.md`, `rust-toolchain.toml`, `rustfmt.toml`, and `mise.toml`
- CI workflows go in `.github/workflows/`
- Use `{{project_name}}` and `{{author}}` for dynamic content
- Follow the casing conventions in `AGENTS.md`

### Adding a new CLI command

Commands are defined in `packages/cli/src/bin/rusty_repo.rs` using the `clap` derive API. Add a new variant to the `Commands` enum and a handler in the `run` function.

## Development

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo fmt --all
```

## Release Process

1. Tag with `v*` (e.g., `v0.1.0`)
2. Push — GitHub Actions builds and creates a release