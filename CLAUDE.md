# CLAUDE.md

Guidance for working in this repository.

## What this is

Node.js bindings for the Rust crate [`rust_xlsxwriter`](https://github.com/jmcnamara/rust_xlsxwriter)
(v0.95.0), built with [napi-rs](https://napi.rs/) v3. The JavaScript API mirrors the
Rust API: `Workbook` → `Worksheet` → cell writes, with `Format`, `Image`, `Chart`,
`Table`, … helper objects. Goal is broad coverage of the crate, built incrementally.

## Build / test commands

```bash
napi build --platform            # debug build (fast); regenerates index.js + index.d.ts
napi build --platform --release  # release build (npm run build)
node --test __test__/<file>.test.mjs   # run a single test file
npm test                         # run all __test__/*.test.mjs
```

- `@napi-rs/cli` (`napi`) is installed globally. `npm install` is only needed for the
  local devDependency / CI.
- A successful build produces `xlsxwriter-node.linux-x64-gnu.node`, `index.js` (loader),
  and `index.d.ts` (auto-generated TypeScript types — never hand-edit).
- The crate sets `#![deny(clippy::all)]`, so builds fail on clippy warnings. Keep code clean.

## Architecture (read before adding features)

`rust_xlsxwriter` hands out worksheets and chart/table sub-objects as `&mut`
references borrowed from their parent, which **cannot** cross into a GC-owned JS
object. The binding works around this:

- **`Workbook`** holds `Rc<RefCell<rust_xlsxwriter::Workbook>>` (+ a `next_index`
  counter). See `src/workbook.rs`.
- **`Worksheet`** is a lightweight handle: a clone of that `Rc` + the sheet `index`.
  Every method re-borrows the sheet via `worksheet_from_index` inside the
  `self.with(|s| ...)` helper. See `src/worksheet.rs`.
- **Helper objects** (`Format`, `Image`, `Chart`, `Table`, …) are **owned** wrappers
  (`pub(crate) inner: x::Type`) passed into worksheet calls by reference
  (`&self.inner`), since the upstream `insert_*` / `add_*` methods take `&T`.
- **Nested builders** (chart series/axes/legend, table columns) are NOT exposed as
  separate JS objects. Flatten them: take a `#[napi(object)]` options struct and do
  all the wiring inside one method call (see `Chart::add_series` in `src/chart.rs`).

Safe because Node runs these calls on a single thread; do not share workbooks across
worker threads.

## Conventions

- One feature area per module under `src/` (`format.rs`, `image.rs`, `chart.rs`, …).
  Wire it in with `mod <name>;` in `src/lib.rs` (alphabetical).
- Errors: convert `XlsxError` with the `.js()` combinator (`crate::error::XlsxResultExt`)
  or `to_napi`. The orphan rule blocks a `From` impl, so don't try to add one.
- Builder setters consume `self` upstream and return `Self`. Wrap them in place with
  `std::mem::replace(&mut self.inner, x::Type::new())` (or `self.inner.clone()` if the
  type is `Clone`) and return `&Self` — napi maps `&Self` returns to `this`, giving
  JS method chaining.
- Enums: mirror the upstream enum in `src/enums.rs` with a `#[napi]` enum and a
  `From<Mirror> for x::Enum` impl. Variant names/order must match upstream — grep the
  source, don't guess.
- Rows are `u32` (0–1,048,575), columns are `u16` (0–16,383). Colors are 24-bit RGB
  ints (`0xRRGGBB`). A JS `Date` is taken as `chrono::DateTime<Utc>` and written as its
  UTC wall-clock value (`naive_utc()`); `chrono::NaiveDateTime` can't be a napi param.

## Finding exact upstream signatures

docs.rs struct pages 404 via WebFetch in this environment. Read the **vendored source**
instead:

```
~/.cargo/registry/src/index.crates.io-*/rust_xlsxwriter-0.95.0/src/
```

Grep it for exact signatures and enum variants before writing bindings.

## Adding a new feature module (checklist)

1. Grep the vendored source for the exact API.
2. Create `src/<feature>.rs` with the `#[napi]` wrapper (owned `inner`, flattened
   builders, `&Self`-returning setters).
3. Add any enums to `src/enums.rs`.
4. `mod <feature>;` in `src/lib.rs`.
5. Add the `insert_*` / `add_*` method to `src/worksheet.rs` via `self.with(...)`.
6. `napi build --platform` — must compile clean.
7. Add `__test__/<feature>.test.mjs` and run it.
8. Extend `examples/showcase.mjs` to exercise the new class/methods.
9. Update the docs (see below).

## Keep the docs and showcase in sync (required)

Any change to the public JS API **must** update all of these in the same change —
they are not optional and they must not drift from the actual `index.d.ts`:

- **`README.md`** — the "API surface (current)" section (classes, methods, enums) and
  the usage example if relevant.
- **`COVERAGE.md`** — the feature-by-feature coverage vs. upstream `rust_xlsxwriter`
  and the roadmap. Move items from a backlog/⬜/🟡 state to ✅ as you implement them,
  update the summary table counts, and add any newly-discovered gaps to the backlog.
- **`examples/showcase.mjs`** — must call **every** exposed class and method (it is the
  "open it in Excel and eyeball everything" example). When you add or change API,
  add/adjust the corresponding lines. Verify with `npm run example` — it must write the
  file without throwing.

When in doubt, derive the surface from the generated `index.d.ts` so the docs and the
showcase match what's actually exposed.

## Commit conventions

Use [Conventional Commits](https://www.conventionalcommits.org/) for every commit.

Format: `type(optional-scope): summary` — imperative mood, lowercase summary, no
trailing period. Add a body (blank line first) to explain the why when it isn't
obvious; mark breaking changes with `!` after the type/scope (e.g. `feat!:`) and/or a
`BREAKING CHANGE:` footer.

Types used in this repo:

- `feat` — a new binding feature / public API addition
- `fix` — a bug fix
- `docs` — README/COVERAGE/CLAUDE or other docs
- `test` — tests only
- `build` — Cargo/npm/deps, packaging, `package.json`
- `ci` — GitHub Actions / workflow changes
- `refactor`, `perf`, `chore` — as usual

Suggested scopes: a module name (`chart`, `table`, `worksheet`, `format`, …),
`ci`, `deps`, `examples`.

Examples:

```
feat(chart): add data-label configuration
fix(worksheet): clamp row/col to Excel limits
docs(coverage): mark data validation as implemented
ci: run tests via npm test (no shell glob)
```

Keep the `Co-Authored-By` trailer on agent-made commits.
