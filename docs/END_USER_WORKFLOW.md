# End-user workflow and distribution roadmap

This document separates **framework repository layout** (what maintainers need) from **application layout** (what product developers should see) and lists the steps to reach a Laravel-style `composer create-project` experience with Cargo.

## Two worlds

| Role | Sees | Purpose |
| --- | --- | --- |
| **Framework maintainer** | Full monorepo: `crates/`, `examples/`, `docs/`, `rules/`, `scripts/` | Implement and ship `rustbridge-*` libraries and `omnisan` |
| **Application developer** | One project folder: `src/`, `config/`, `resources/`, `migration/`, `./omnisan` | Build a product; no framework source tree |

Vue application **scaffold source** lives inside the CLI crate:

`crates/omium-cli/templates/vue-inertia-app/`

It is not copied into the developer’s tree; `./omnisan new` copies it to the chosen destination.

## Today (Phase 7 — monorepo)

1. Clone the Framework repository (contains `crates/`).
2. From repo root: `./omnisan new my-shop --frontend vue --yes`  
   - Creates `my-shop/` (or `examples/my-shop/`) with a full app.  
   - `Cargo.toml` uses **path dependencies** to `crates/rustbridge-*` (computed relative to the app path).
3. If the app sits in the workspace, add `my-shop` and `my-shop/migration` to root `Cargo.toml` `[workspace.members]`.
4. `cd my-shop && ./omnisan install && ./omnisan dev`.

**Limitation:** The Framework checkout (or a tree that contains `crates/omium-core` above the app) must exist. An empty directory on disk with only `omnisan` is **not** enough yet.

**Do not** run `./omnisan new` into the template directory inside `crates/omium-cli/templates/`.

## Target experience (product goal)

```text
mkdir ~/Projects/acme-shop && cd ~/Projects/acme-shop
omnisan new . --frontend vue --app-name "Acme Shop" --yes
./omnisan install
./omnisan dev
```

Developer sees only the application. Framework libraries come from **crates.io version pins**, not `../../crates`.

Requires owner approval for **O-001** (naming, license) and a **published crate policy** before claiming this is shipped.

## Decision for Phase 7 (current milestone)

**Work in the monorepo model until crates are published.** Do not block Vue/Inertia, CLI, or app features on `cargo install omnisan` from crates.io.

| Question | Answer for *now* |
| --- | --- |
| Where is the framework? | `crates/rustbridge-*` in the Framework git checkout |
| Where is *your* product code? | Only the folder created by `./omnisan new …` (then `cd` there for features) |
| Can you use `omnisan new .` today? | **Yes**, if the directory is empty **and** a parent path contains `crates/omium-core` (usually inside the cloned Framework repo) |
| Random empty folder + global `omnisan` only? | **No** — `new` still needs the monorepo for path dependencies |
| Does `cargo install omnisan` alone fix that? | **No** — needs published `rustbridge-*` libs + generator version deps (Steps 3–5). Use **`omnisan self-install`** today for a global CLI inside the monorepo. |

**Mental model:** `crates/` = framework engine (our code). The app from `new` = your product (like a Laravel project tree).

**Unlock “empty folder anywhere” later:** O-001 + crates.io publish + version-pinned generator (Steps 2–5 below).

## Roadmap (recommended order)

### Step 1 — Scaffold hygiene (in progress)

- [x] Move Vue template under `crates/omium-cli/templates/`.
- [x] Rewrite path dependencies on `new` using `detect_crate_path`.
- [x] Global CLI from monorepo: `omnisan self-install` → `~/.cargo/bin/omnisan`; `./omnisan` wrappers delegate to PATH when present.
- [ ] Optional: `omnisan remove` with `--yes` (app directory only).
- [ ] Optional: auto-patch root `Cargo.toml` `[workspace.members]` after `new` inside monorepo.

### Step 2 — Standalone app manifest (no workspace)

Generated apps outside the monorepo need a **self-contained** root `Cargo.toml` (pinned versions, no `edition.workspace = true`) while still using path deps during dogfooding—or dual templates for “in-repo” vs “standalone”.

### Step 3 — Publish framework crates

Publish to crates.io (names per **O-001** / **D-018**):

- `omium-core`, `omium-http`, `omium-db`, `omium-inertia`, … (public names TBD; may become `omium-*`)
- CLI: crate **[`omnisan`](https://crates.io/crates/omnisan)** — owner placeholder **0.1.0** published; monorepo **`omium-cli`** maps here on release (`cargo install omnisan`).

Generator switches from `{ path = "…" }` to `{ version = "…" }` for released versions.

### Step 4 — Install CLI globally

**Today (monorepo, no crates.io):**

```bash
# From Framework repo root or any app inside the clone:
./omnisan self-install
# equivalent:
cargo install --path crates/omium-cli --bin omnisan --locked
```

After that, run **`omnisan`** (no `./`) anywhere on PATH; app `./omnisan` scripts call the same binary.

**After Step 3 publish (target — requires O-001):**

```bash
cargo install omnisan
```

Notes for packaging (decide under **O-001**):

| Topic | Today | Target |
| --- | --- | --- |
| Binary name | `omnisan` (`[[bin]]` in `omium-cli`) | Same |
| Crate on crates.io | Not published (`publish = false` workspace) | Either crate **`omnisan`** *or* `omium-cli` with docs saying `cargo install omium-cli --bin omnisan` |
| `omnisan new` without monorepo | Needs ancestor `crates/omium-core` | Step 5: version-pinned deps only |

The CLI binary working globally is **already implemented** for monorepo development. **`cargo install omnisan` from crates.io** is Step 3 + O-001, not a separate CLI rewrite.

App wrapper `./omnisan` can remain for teams that prefer repo-local fallback via `cargo run`.

### Step 5 — `new` in an empty directory

Support:

```bash
omnisan new . --frontend vue
```

with no Framework clone on disk—only crates.io deps.

### Step 6 — Optional distribution channels

- GitHub template repository (“Use this template”).
- `cargo generate` integration.
- Installer script (discouraged until crates.io path is stable).

## What stays in the monorepo root

| Path | Stays at root? | Reason |
| --- | --- | --- |
| `crates/` | Yes | All Rust libraries + CLI + embedded templates |
| `examples/` | Yes (maintainers) | Demo apps; not shipped to end users |
| `docs/`, `rules/` | Yes | Engineering and AI agent rules |
| `scripts/` | Yes (for now) | Root `./omnisan` wrapper for repo developers |
| `templates/` | **No** | Moved under `crates/omium-cli/templates/` |

End-user projects should **not** contain `crates/`, `docs/`, or `rules/` from the framework.

## Related decisions

- `docs/OPEN_DECISIONS.md` **O-001** — public names and license before publish.
- `docs/IMPLEMENTATION_ROADMAP.md` — feature phases.
- `docs/CLI_AND_GENERATORS.md` — current CLI commands.
