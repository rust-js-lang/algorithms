# algorithms

Two ports, each checked against its original, to find and close gaps in
[rust-js](https://github.com/rust-js-lang/rust-js):

1. **[TheAlgorithms/rust](https://github.com/TheAlgorithms/rust)**, compiled
   to JavaScript by rust-js. Its crate is vendored unchanged in
   `the-algorithms-rust/` (upstream commit in `UPSTREAM`), and its own
   `#[test]`s run natively and as JS, test by test.
2. **[trekhleb/javascript-algorithms](https://github.com/trekhleb/javascript-algorithms)**,
   written in Rust and compiled by rust-js, checked against the original
   JavaScript.

Each algorithm is checked on its own: its module, with only the modules it
uses, is built alone, its `#[test]`s compiled by rust-js and run by bun, and
each test's outcome compared with the same test's natively. Natively is as
rust-js's JS should run it: built by rust-js's rustc for `wasm32-wasip1`,
whose `usize` is rust-js's 32 bits, with overflow unchecked as rust-js's is,
and run under Bun's WASI (`scripts/wasi-run.ts`); a test that panics as it
passes, `#[should_panic]` or by `catch_unwind`, aborts there, so it keeps
the host's outcome. `64-bit` is one that doesn't build where `usize` is 32
bits. One that uses an external crate (`num-bigint`, `nalgebra`, `ndarray`,
`quickcheck`) is a Cargo crate of those alone, its tests run by `rust-js
test` (rust-js's ADR 0916), and natively by `cargo test` for
`wasm32-wasip1`. Its status is in `the-algorithms-rust/status.json`: `same`,
or what differs, a gap fixed in rust-js. `needs-crate` is one that uses
`rand`, whose `getrandom` has no backend for rust-js's target yet.

## Running

It expects a rust-js checkout beside it, `../rust-js`, built with
`cargo build`, or `RUST_JS` naming its compiler.

```bash
bun install
bun scripts/the-algorithms-rust.ts native               # upstream's tests natively, once
bun scripts/the-algorithms-rust.ts next 10              # the next 10 algorithms not yet the same
bun scripts/the-algorithms-rust.ts one sorting::quick_sort
bun scripts/the-algorithms-rust.ts list                 # each algorithm's status
```

Both upstreams are MIT licensed; their licenses are kept beside their code.

## javascript-algorithms

Each JS module of `javascript-algorithms/src/` (upstream's, vendored) is a
Rust module at the same path in `javascript-algorithms/rust/src/`, its
directories and file in snake_case, built by `rust-js build`, which writes
its JS beside it. The original's own tests then run twice under bun: on
the original, and on a copy whose module at that path re-exports the
compiled one. Each test's outcome is compared, in
`javascript-algorithms/status.json`.

```bash
bun scripts/javascript-algorithms.ts
```
