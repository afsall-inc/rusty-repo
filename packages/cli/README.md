# Rusty-Repo CLI

Scaffold new Rust projects from templates — with CI/CD, PRDoc, and agentic infrastructure baked in.

## Install

```bash
cargo install rusty-repo-cli
```

## Usage

```bash
# Create a new project
rusty-repo new my-project --template default

# Initialize in the current directory
rusty-repo init

# List available templates
rusty-repo templates
```

## Templates

- **default** — Single crate project with CI/CD, AGENTS.md, PRDoc, mise.toml
- **workspace** — Multi-package workspace with `apps/` and `packages/`
- **prdoc** — PRDoc template for structured PR docs

## License

Apache-2.0 OR MIT