# Development Plans

This directory contains implementation plans organized by module.

## Structure

```
plans/
├── {module-name}/
│   ├── epic-00-setup.md
│   ├── epic-01-core.md
│   └── ...
└── README.md
```

## Status Legend

- 🔴 Not started
- 🟡 In progress
- 🟢 Complete

## Task Format

Each epic file uses YAML frontmatter:

```yaml
---
module: module-name
epic: 1
title: Epic Title
priority: high|normal|low
depends_on:
  - other-module/E00
---
```

## Commands

```bash
# Sync tasks from plans
claude-orch sync

# View status
claude-orch status

# Start tasks
claude-orch start
```
