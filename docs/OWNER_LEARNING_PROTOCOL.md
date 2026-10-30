# Owner learning protocol — mandatory

The owner is an experienced PHP/Laravel developer and a Rust beginner. The project must teach Rust while it is being built.

## Required language and tone

- Explain to the owner in Turkish.
- Use PHP/Laravel as the first comparison point.
- Assume strong knowledge of HTTP, MVC, ORM, SQL, queues, middleware, API design, and software architecture.
- Do not explain basic programming or web concepts unless needed.
- Never assume the owner understands Rust ownership, borrowing, traits, lifetimes, generics, async execution, or compiler diagnostics.
- Be concise but technically accurate.
- Do not hide Rust differences to make the framework appear more familiar.

## Required explanation after each completed task

### A. PHP/Laravel karşılığı
Show the closest PHP/Laravel concept.

Example:
- Laravel Controller method ↔ Rust async handler function.
- Eloquent Model ↔ SeaORM Entity/ActiveModel.
- Laravel Service Provider ↔ application bootstrap/state composition (not always a one-to-one match).
- Laravel Middleware ↔ Tower Layer/Service.
- Laravel `config()` ↔ typed Rust config struct loaded at startup.

### B. PHP örneği
Show a small PHP/Laravel example where useful.

### C. Rust karşılığı
Show the actual Rust code from this project, not invented pseudocode.

### D. Rust'ta yeni olan kavram
Explain the Rust-specific concept used:
- ownership: who owns a value;
- borrowing: temporary access by reference;
- lifetimes: how long references remain valid;
- traits: shared behavior contracts;
- generics: reusable types/functions;
- `Result`: success or error;
- `Option`: value or absence;
- async/await and futures;
- `Send`/`Sync` when relevant;
- macros only when actually used.

### E. Neden böyle yaptık?
Explain the trade-off and why the Rust approach differs from PHP.

### F. Ben bunu nasıl test ederim?
Give exact command(s), expected result, and one small experiment the owner can perform.

## Rust learning progression

Teach concepts as they naturally appear. Do not dump a full Rust course before coding.

Suggested progression:
1. Cargo project and modules.
2. Structs, enums, functions, `impl`.
3. `Result`, `Option`, error propagation.
4. Ownership and borrowing.
5. Traits and generics.
6. Async/await and Tokio.
7. Axum extractors and typed state.
8. SeaORM entities and relations.
9. Lifetimes only when code genuinely needs them.
10. `Send`/`Sync` and concurrency.
11. Macros and procedural macros only if approved and required.

## Important teaching rule

If a Rust compiler error occurs, explain:
1. What the compiler is protecting.
2. Why a similar PHP pattern may work.
3. What the Rust type/ownership rule is.
4. The smallest idiomatic fix.
5. Whether the fix changes runtime behavior.

Do not merely paste a corrected file without teaching the concept.
