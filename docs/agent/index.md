# Agent Documentation

This directory contains agent-focused documentation for rusty-repo.

For the full agentic workflow, see [AGENTS.md](../AGENTS.md).

## Key Files

- `AGENTS.md` — Agent guide with architecture, commands, and conventions
- `skills/scaffold/skill.toml` — Composable scaffold workflow
- `templates/` — Template files with `{{project_name}}` placeholders
- `Dockerfile` — Multi-stage container build for ghcr.io
- `.github/workflows/cd.yml` — CD pipeline: version bump detection → container push
- `.github/workflows/release.yml` — Release pipeline: tag push → GitHub Release + container push