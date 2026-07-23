# CLAUDE.md

This file exists so Claude Code (and other coding agents) pick up the project's
guidance automatically. To keep a **single source of truth**, it does not repeat
that guidance — it points at it.

## Read these, in order

1. [`AGENTS.md`](AGENTS.md) — what the project is, the commands, the green gate,
   and the house rules. **Start here.**
2. [`AGENTS/`](AGENTS/architecture.md) — operational detail:
   [architecture](AGENTS/architecture.md),
   [conventions](AGENTS/conventions.md),
   [code generation](AGENTS/code-generation.md),
   [testing](AGENTS/testing.md), and the [glossary](AGENTS/glossary.md).
3. [`spec/index.md`](spec/index.md) — the **living specifications**, which are
   the source of truth for behaviour. Code and spec must not drift; when they
   disagree, reconcile them.

## The one rule that matters most

Keep the crate **green** before considering any task done:

```sh
cargo build --all-targets
cargo test                                    # unit tests + doctests
cargo clippy --all-targets -- -D warnings     # zero warnings (pedantic is on)
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

## Three FHIR releases, and only one is on by default

The crate models **R5** (`fhir::r5`, feature `r5`, default), **R4**
(`fhir::r4`), and **R3/STU3** (`fhir::r3`). The commands above therefore see
only R5. If you touched the generator, the derive macros, the crate-root
modules, or `src/r3`/`src/r4`, run the gate with those releases enabled too:

```sh
cargo test --features "r3 r4 xml client"
cargo clippy --all-targets --features "r3 r4 xml client" -- -D warnings
```

And note which tree you are editing:

- **`src/r3/` and `src/r4/` are generated.** Change `src/codegen/`, then
  `cargo run -- r3` / `cargo run -- r4`.
- **`src/r5/` is hand-documented.** Never regenerate over it; `cargo run -- r5`
  refuses without an explicit `--out`.

Everything else — the conventions, the cardinality mapping, the generator, the
release checklist — lives in the documents linked above.
