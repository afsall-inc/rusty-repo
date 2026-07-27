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
npm run ci
```

Or manually:

```bash
cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace
cargo fmt --all
```

## Release Process

1. Bump the version in `Cargo.toml` (workspace level)
2. Merge to `main` — CD workflow builds and pushes the Docker image to `ghcr.io/afsall-inc/rusty-repo`
3. Tag with `v*` (e.g., `v0.1.1`) — Release workflow creates a GitHub Release and pushes the Docker image
4. Publish to crates.io: `cargo publish --package rusty-repo-cli`