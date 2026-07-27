# Rusty-Repo

A 100% Rust template repo for starting new Rust projects. Comes with CLI, CI/CD, PRDoc, and agentic infrastructure baked in.

## Install

```bash
cargo install rusty-repo-cli
```

Or via Docker:

```bash
docker pull ghcr.io/afsall-inc/rusty-repo:latest
```

## Usage

```bash
# Create a new project
cargo run -- new my-project --template default

# Or initialize in the current directory
cargo run -- init

# List available templates
cargo run -- templates
```

Or use the Docker image:

```bash
docker pull ghcr.io/afsall-inc/rusty-repo:latest
docker run --rm -v "$PWD:/workspace" ghcr.io/afsall-inc/rusty-repo:latest new my-project
```

## Templates

- **default** — Single crate project with CI/CD, AGENTS.md, PRDoc, mise.toml
- **workspace** — Multi-package workspace with `apps/` and `packages/` directories

- **prdoc** — PRDoc template for structured PR docs

## Projects Using This Template

- [montrs](https://github.com/afsall-labs/montrs) — Full-stack Rust framework
- [changelogger](https://github.com/anomalyco/changelogger) — Changelog generation tool

## License

Apache-2.0 OR MIT