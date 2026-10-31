# Instructions for AI coding agents

You are implementing Omium, a Rust-native web framework. The owner is highly experienced in PHP/Laravel and web application engineering, but is learning Rust from zero.

## Mandatory reading

Before every task:
1. Read `rules/00-global.md`.
2. Read all rule files relevant to the requested change.
3. Read the relevant architecture and decision documents.
4. Inspect the current codebase and tests. Never assume this blueprint describes code that has already been implemented.

## Authority and approval

- `docs/DECISIONS.md` is the approved decision register.
- A decision marked **APPROVED** is binding.
- A decision marked **PROPOSED** is not permission to implement.
- Any item in `docs/OPEN_DECISIONS.md` requires explicit owner approval.
- Stop and ask the owner before changing a public API, framework-wide abstraction, crate choice, database support matrix, authentication/session design, frontend protocol, licensing, naming, release policy, or roadmap scope.
- Do not interpret silence, a vague response, or an AI-generated suggestion as approval.
- When blocked, continue only with independent work that cannot constrain the unresolved decision. Otherwise stop.

## Implementation behavior

- Implement one small, reviewable vertical slice at a time.
- Prefer mature crates over custom implementations, subject to the approved stack and dependency policy.
- Keep third-party dependencies behind framework-owned adapters where practical.
- Do not fork or copy third-party code without an explicit license and maintenance review.
- Do not add dependencies without checking maintenance, license, security advisories, MSRV, feature flags, and compatibility.
- Do not use `unsafe`.
- Do not use `unwrap()` or `expect()` in production paths. Tests and examples may use them when justified.
- Do not use blocking I/O on async request paths.
- Never log passwords, tokens, cookies, session IDs, authorization headers, or sensitive personal data.
- Never return internal errors, SQL details, stack traces, or secrets to clients.
- Do not store secrets in source control or example `.env` files.
- Preserve backward compatibility for published public APIs unless the owner approves a breaking change.

## Required workflow for every task

1. **Understand:** summarize the requested task and the relevant existing architecture.
2. **Plan:** list files/modules to change and tests to add. If a decision is required, ask first.
3. **Implement:** make the smallest complete change.
4. **Verify:** run formatting, linting, tests, and relevant integration checks. Report exact commands and results. Never fabricate successful output.
5. **Review:** inspect the diff for security, API consistency, unnecessary dependencies, and documentation.
6. **Document:** update relevant docs and examples.
7. **Teach:** provide the Turkish PHP/Laravel comparison and Rust lesson required by `docs/OWNER_LEARNING_PROTOCOL.md`.
8. **Next step:** propose one specific next task; do not start it without authorization if it crosses a milestone or approval gate.

## Required completion report

Every completed task response must use this structure:

### 1. What was built
Plain-language summary in Turkish.

### 2. PHP/Laravel comparison
Show the closest PHP/Laravel equivalent and explain what is similar and what is different. If no direct equivalent exists, say so.

### 3. Rust concepts learned
Explain only the Rust concepts actually used, such as ownership, borrowing, traits, generics, `Result`, `Option`, lifetimes, async/await, `Send`/`Sync`, or macros.

### 4. Files changed
List files and purpose.

### 5. Verification
List exact commands, pass/fail status, and any unrun checks.

### 6. Risks / limitations
State known limitations honestly.

### 7. Approval needed
Explicitly state either “No approval needed” or ask one clearly scoped approval question.

### 8. Suggested next task
Suggest one next task without silently beginning it.

## Language

- Code, identifiers, comments, commit messages, technical docs, and API names: English.
- Owner-facing explanations, learning notes, approval questions, and completion reports: Turkish.
- Be direct, precise, and respectful. Do not overwhelm the owner with Rust terminology without defining it.
