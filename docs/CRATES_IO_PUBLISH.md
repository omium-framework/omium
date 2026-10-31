# Publishing to crates.io (`omium-*`)

Registry and monorepo use the **`omium-*`** crate names and **`omium_*`** Rust library names.

## Install (end user)

```bash
cargo install omnisan
omnisan --version
```

## Publish order

1. `omium-core`
2. `omium-view`
3. `omium-http`
4. `omium-db`
5. `omium-inertia`
6. `omium-cli`
7. `omnisan` (`Cargo/omnisan`)

```bash
ALLOW_DIRTY=1 ./scripts/publish-omium-crates.sh
```

Keep `[workspace.package].version`, `OMIUM_REGISTRY_VERSION`, and `omnisan`’s `omium-cli` dependency version aligned.

When only the CLI/generator changes, bump **`omium-cli`** and **`omnisan`** (registry path deps stay on published `omium-*` versions). Publish just those crates:

```bash
ALLOW_DIRTY=1 ./scripts/publish-omium-crates.sh omium-cli omnisan
```

When a library crate changes, bump **`[workspace.package].version`** (and `OMIUM_REGISTRY_VERSION` in `crate_deps.rs`), then publish from `omium-core` through `omnisan` in order (full script or from the first changed crate onward).

Nested scaffolds use **`Cargo.toml.template`** so `cargo package` on `omium-cli` includes generator templates.
