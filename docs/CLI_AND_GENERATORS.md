# CLI and project generators

## CLI goals

Provide a Laravel Artisan-like workflow while remaining a Cargo-native Rust project. Clap is the selected CLI parsing library. The public CLI binary name is not approved yet.

## Proposed commands

The command prefix below is a placeholder and must not be published until the CLI name is approved.

```bash
<cli> new my-app
<cli> serve
<cli> make:controller ProductController
<cli> make:model Product
<cli> make:migration create_products_table
<cli> migrate
<cli> migrate:status
<cli> route:list
<cli> test
<cli> doctor
```

## `new` command

Must prompt for:
1. Application name.
2. Frontend mode: Standard / Vue / React / API.
3. Database: initially MySQL only.
4. Optional starter features only when approved and implemented.

The command must:
- generate valid Rust/Cargo project;
- create the correct frontend package only for Vue/React;
- generate config files and `.env.example`;
- generate a health endpoint and one example resource;
- generate tests;
- generate `README.md` and local AI rules;
- print next steps and required tools.

## Generators

Generators must produce idiomatic, readable Rust. Generated files are starting points, not hidden magic.

- Controller generator: async handler skeleton with typed response.
- Model generator: SeaORM entity scaffold or documented entity generation workflow.
- Migration generator: SeaORM Migration scaffold.
- Service generator: plain Rust service module.
- Test generator: appropriate unit/integration test scaffold.

Do not invent a custom Rust parser or source rewriter for generators. Use templates for initial scaffolding. If AST editing is proposed, request approval.

## CLI behavior

- Errors must explain what failed and how to fix it.
- Destructive commands require confirmation unless an explicit `--yes` flag is provided.
- `--help` must be complete.
- Commands should be deterministic and testable.
- Support non-interactive CI usage.
- Never silently overwrite user files.
