# Getting Started

## Installation

```bash
git clone https://github.com/afsall/rusty-repo
cd rusty-repo
cargo build --release
```

The binary will be at `target/release/rusty-repo`.

## Usage

### Create a new project from a template

```bash
rusty-repo new my-project --template default
```

This scaffolds a single-crate project with:

- AGENTS.md — agentic workflow guide
- CI/CD via GitHub Actions
- PRDoc template for structured PR documentation
- mise.toml for task runner
- rust-toolchain.toml (nightly-2026-02-18)
- rustfmt.toml with consistent formatting rules

### Initialize in current directory

```bash
rusty-repo init
```

### List available templates

```bash
rusty-repo templates
```

## Templates

| Template | Use Case |
|----------|----------|
| `default` | Single crate project with CI/CD, AGENTS.md, PRDoc |
| `workspace` | Multi-package workspace with `apps/` and `packages/` |
| `prdoc` | PRDoc template for structured PR docs |

## Next Steps

After scaffolding:

```bash
cd my-project
cargo check --workspace
cargo fmt --all
git init && git add . && git commit -m "Initial scaffold"
```

For projects like `montrs`, use the workspace template:

```bash
rusty-repo new montrs --template workspace
```