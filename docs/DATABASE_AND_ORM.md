# Database and ORM conventions

## Approved direction

Use SeaORM as the default ORM and SeaORM Migration for schema migrations. Do not build a new ORM.

MySQL is the initial first-class database target. Other databases require explicit approval.

## Why SeaORM

- Async Rust ORM.
- Entity and ActiveModel patterns.
- Relationship support.
- Query builder and transactions.
- Migration ecosystem.
- Better fit for a batteries-included framework than forcing every user to write raw SQL.

SQLx is permitted as a lower-level escape hatch for specialized parameterized SQL. It is not the default application data API.

## Framework conventions to build on top of SeaORM

- Generated entity modules live in `src/models/` or a clearly documented entity subfolder.
- Entity definitions remain valid SeaORM entities.
- DTOs are separate from persistence entities.
- Services own business operations and transaction boundaries.
- Repository wrappers should be introduced only where they add a real boundary; do not wrap every SeaORM call mechanically.
- Relations are explicit.
- Pagination is bounded and has safe defaults.
- Avoid implicit eager loading.
- Avoid N+1 queries; examples must show explicit relation loading.
- Use transactions for multi-step state changes.
- Use typed columns and enums where appropriate.
- Do not serialize ActiveModel or internal entities directly to public responses.

## Migrations

- Generate migration files through CLI.
- Migration names are descriptive and ordered.
- Migrations should be reversible when feasible.
- Destructive operations must be documented and reviewed.
- Production migration execution is an explicit deployment step in the initial release.
- Never auto-run migrations on every application boot.
- Test migrations from an empty database and test upgrade paths from the prior schema.
- Do not edit an already-applied migration in a released application; create a new migration.

## Query examples

The public framework API must remain idiomatic Rust. Do not imitate PHP's dynamic magic query syntax.

A SeaORM-style query may look like:

```rust
let active_users = user::Entity::find()
    .filter(user::Column::IsActive.eq(true))
    .order_by_desc(user::Column::CreatedAt)
    .all(&db)
    .await?;
```

A framework helper may improve ergonomics only if it preserves type safety and does not obscure generated SQL or error behavior.

## PHP/Laravel teaching analogy

- Eloquent Model ↔ SeaORM Entity + ActiveModel.
- Eloquent relationship ↔ SeaORM Relation definitions and relation loading.
- `DB::transaction()` ↔ SeaORM transaction API.
- Laravel migration ↔ SeaORM Migration crate.
- Eloquent collection ↔ typed Rust `Vec<Model>` / model result collection.

These are conceptual comparisons, not exact behavioral equivalences. Explain differences every time.
