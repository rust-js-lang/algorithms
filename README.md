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

A module rust-js can't compile yet is left out of the build, with its
reason, in `the-algorithms-rust/excluded.json`. Each entry is a rust-js gap,
fixed in rust-js, after which the entry goes.

## Running

It expects a rust-js checkout beside it, `../rust-js`, built with
`cargo build`, or `RUST_JS` naming its compiler.

```bash
bun install
bun scripts/the-algorithms-rust.ts prepare   # build the JS, leaving out what rust-js refuses
bun scripts/the-algorithms-rust.ts native    # upstream's tests natively, each in its own process
```

Both upstreams are MIT licensed; their licenses are kept beside their code.
