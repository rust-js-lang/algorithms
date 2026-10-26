// TheAlgorithms/rust, compiled to JavaScript by rust-js one algorithm at a
// time: each algorithm's module, with only the modules it uses, built alone,
// its #[test]s compiled by rust-js and run by bun, and each test's outcome
// compared with the same test's natively.
//
//   bun scripts/the-algorithms-rust.ts native           upstream's tests natively, once
//   bun scripts/the-algorithms-rust.ts next [n]         the next n algorithms not yet the same
//   bun scripts/the-algorithms-rust.ts one <module>     one algorithm, `string::aho_corasick`
//   bun scripts/the-algorithms-rust.ts list             each algorithm and its status
//
// The upstream crate is vendored unchanged in the-algorithms-rust/. Each
// algorithm's status is in the-algorithms-rust/status.json: `same` where
// every test's outcome is native Rust's, else what differs, a rust-js gap to
// fix in rust-js.
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { spawnSync } from "node:child_process";

const root = join(import.meta.dir, "..");
const crate = join(root, "the-algorithms-rust");
const src = join(crate, "src");
const statusFile = join(crate, "status.json");
const nativeFile = join(root, "build", "native.json");
const rustJs = process.env.RUST_JS ?? join(root, "..", "rust-js", "target", "debug", "rust-js");
const rustJsCli = join(root, "..", "rust-js", "tooling", "cli.js");

type Status = { status: "same" | "differs" | "refused" | "crashed" | "needs-crate" | "64-bit"; detail?: string; tests?: number };
type Statuses = Record<string, Status>;
type Results = Record<string, "pass" | "fail">;

const readJson = <T>(file: string, empty: T): T => (existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : empty);
const writeStatuses = (statuses: Statuses) =>
  writeFileSync(statusFile, JSON.stringify(Object.fromEntries(Object.entries(statuses).sort()), null, 2) + "\n");

// The crate's modules: each file's, `a/b.rs` `a::b`, `a/mod.rs` `a`.
const modules = new Map<string, string>();
(function find(dir: string, path: string[]) {
  for (const entry of readdirSync(dir, { withFileTypes: true })) {
    if (entry.isDirectory()) {
      find(join(dir, entry.name), [...path, entry.name]);
    } else if (entry.name.endsWith(".rs") && !(path.length === 0 && entry.name === "lib.rs")) {
      const name = entry.name.replace(/\.rs$/, "");
      modules.set((name === "mod" ? path : [...path, name]).join("::"), join(dir, entry.name));
    }
  }
})(src, []);

const hasTests = (module: string) => /#\[test\]/.test(readFileSync(modules.get(module)!, "utf8"));

/** The algorithms: each module with tests of its own, a parent's too. */
function algorithms(): string[] {
  return [...modules.keys()]
    .filter(hasTests)
    .sort();
}

/** The crate's external crates, which a module names by their own names. */
const externalCrates = ["nalgebra", "ndarray", "num_bigint", "num_traits", "rand", "quickcheck", "quickcheck_macros"];

/** A `use` tree's paths, `a::{b, c::{D as E, *}}` `a::b`, `a::c::D as E`
 * and `a::c::*`: each a path's segments, its last `D as E` where renamed. */
function useTree(text: string): string[][] {
  const tokens = text.match(/\w+|::|[{},*]/g) ?? [];
  let at = 0;
  const tree = (prefix: string[]): string[][] => {
    const path = [...prefix];
    for (;;) {
      const t = tokens[at];
      if (t === "{") {
        at++;
        const paths: string[][] = [];
        while (at < tokens.length && tokens[at] !== "}") {
          paths.push(...tree(path));
          if (tokens[at] === ",") at++;
        }
        at++;
        return paths;
      }
      if (t === "*") {
        at++;
        return [[...path, "*"]];
      }
      if (t === undefined || t === "," || t === "}") return [path];
      at++;
      if (t === "::") continue;
      if (tokens[at] === "as") {
        path.push(`${t} as ${tokens[at + 1]}`);
        at += 2;
      } else {
        path.push(t);
      }
    }
  };
  return tree([]);
}

const nameOf = (segment: string) => segment.split(" as ")[0];
const boundAs = (segment: string) => segment.split(" as ").at(-1)!;

/** Each `use` statement of a file: where it is, whether it's `pub`, its paths. */
function useStatements(text: string) {
  return [...text.matchAll(/(^|\n)([ \t]*)((?:#\[[^\]]*\]\s*)*)(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);/g)].map((m) => ({
    start: m.index! + m[1].length,
    end: m.index! + m[0].length,
    indent: m[2],
    attrs: m[3],
    pub: m[4] ?? "",
    paths: useTree(m[5]),
  }));
}

/** The names module `module`'s file `text` defines or re-exports. */
function exported(text: string, module: string, read: (m: string) => string, seen = new Set<string>()): Set<string> {
  const names = new Set<string>();
  if (seen.has(module)) return names;
  seen.add(module);
  const items = /pub(?:\([^)]*\))?\s+(?:(?:unsafe|const|async|extern)\s+)*(?:fn|struct|enum|trait|type|const|static|mod|union)\s+(\w+)/g;
  for (const [, name] of text.matchAll(items)) names.add(name);
  for (const statement of useStatements(text).filter((u) => u.pub)) {
    for (const path of statement.paths) {
      const last = path.at(-1)!;
      if (last !== "*") {
        names.add(boundAs(last));
        continue;
      }
      const base = module ? module.split("::") : [];
      const from = (path[0] === "self" ? [...base, ...path.slice(1, -1)] : path.slice(0, -1)).join("::");
      if (modules.has(from)) for (const n of exported(read(from), from, read, seen)) names.add(n);
    }
  }
  return names;
}

const readSource = (m: string) => readFileSync(modules.get(m)!, "utf8");

/** The modules a path from the crate's root names: its longest prefix
 * that's a module, and where its next segment is an item that module
 * re-exports, the module it's from. */
function moduleOfPath(path: string[], seen = new Set<string>()): string[] {
  let k = path.length;
  while (k > 0 && !modules.has(path.slice(0, k).join("::"))) k--;
  if (k === 0) return [];
  const module = path.slice(0, k).join("::");
  const item = path[k];
  if (item === undefined || seen.has(`${module}::${item}`)) return [module];
  seen.add(`${module}::${item}`);
  const found = [module];
  for (const statement of useStatements(readSource(module)).filter((u) => u.pub)) {
    for (const p of statement.paths) {
      const last = p.at(-1)!;
      if (last !== "*" && boundAs(last) !== item) continue;
      const rel = p[0] === "self" ? p.slice(1) : p;
      const from = p[0] === "crate" ? p.slice(1) : [...module.split("::"), ...rel];
      found.push(...moduleOfPath([...from.slice(0, -1).map(nameOf), last === "*" ? item : nameOf(last)], seen));
    }
  }
  return found;
}

/** The modules `module`'s file names, `crate::a::b` and `super::c`: not a
 * parent's re-exports of its children, `pub use self::c::f`, which only
 * name what's there. */
function uses(module: string): string[] {
  const text = readSource(module);
  const path = module.split("::");
  const parent = modules.get(module)!.endsWith("mod.rs");
  const here = parent ? path : path.slice(0, -1);
  const found = new Set<string>();
  const add = (base: string[], rest: string[]) => {
    for (const m of moduleOfPath([...base, ...rest.map(nameOf).filter((s) => s !== "*" && s !== "self")])) found.add(m);
  };
  const paths: string[][] = [];
  for (const statement of useStatements(text)) paths.push(...statement.paths);
  for (const [, rest] of text.matchAll(/\b((?:crate|super)(?:::\w+)+)/g)) paths.push(rest.split("::"));
  for (const p of paths) {
    if (p[0] === "crate") add([], p.slice(1));
    if (p[0] === "super") {
      const up = p.findIndex((s) => s !== "super");
      // From the file's top, or from its `mod tests`, one deeper.
      for (const depth of [up, up - 1]) {
        if (depth >= 1 && depth - 1 <= here.length) add(here.slice(0, here.length - (depth - 1)), p.slice(up));
      }
    }
  }
  found.delete(module);
  return [...found];
}

/** `module` and each module it uses, through what they use, and their parents. */
function needed(module: string): Set<string> {
  const kept = new Set<string>();
  const todo = [module];
  while (todo.length) {
    const m = todo.pop()!;
    if (kept.has(m)) continue;
    kept.add(m);
    todo.push(...uses(m));
  }
  for (const m of [...kept]) {
    const path = m.split("::");
    for (let k = 1; k < path.length; k++) kept.add(path.slice(0, k).join("::"));
  }
  return kept;
}

/** A copy of the crate at `dir` with only `kept`'s modules: in each parent
 * kept, deepest first, each left-out child's `mod` goes, and each path of a
 * `use` of what's no longer there. */
function prepare(dir: string, kept: Set<string>) {
  rmSync(dir, { recursive: true, force: true });
  mkdirSync(dir, { recursive: true });
  cpSync(src, join(dir, "src"), { recursive: true });
  const fileOf = (m: string) => (m === "" ? join(dir, "src", "lib.rs") : join(dir, "src", relative(src, modules.get(m)!)));
  const read = (m: string) => readFileSync(fileOf(m), "utf8");
  const parents = ["", ...[...kept].filter((m) => modules.get(m)!.endsWith("mod.rs"))].sort(
    (a, b) => (b ? b.split("::").length : 0) - (a ? a.split("::").length : 0),
  );
  for (const parent of parents) {
    let text = read(parent);
    const base = parent ? parent.split("::") : [];
    const childOf = (name: string) => [...base, name].join("::");
    text = text.replace(/^[ \t]*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;[ \t]*\n?/gm, (all, name) =>
      modules.has(childOf(name)) && !kept.has(childOf(name)) ? "" : all,
    );
    const there = (p: string[]): boolean => {
      const rel = p[0] === "self" ? p.slice(1) : p;
      if (rel.length === 0 || !modules.has(childOf(nameOf(rel[0])))) return true; // std's, a crate's, or the file's own
      let module = childOf(nameOf(rel[0]));
      if (!kept.has(module)) return false;
      for (let k = 1; k < rel.length; k++) {
        const next = `${module}::${nameOf(rel[k])}`;
        if (modules.has(next)) {
          if (!kept.has(next)) return false;
          module = next;
        } else {
          return rel[k] === "*" || exported(read(module), module, read).has(nameOf(rel[k]));
        }
      }
      return true;
    };
    for (const u of useStatements(text).reverse()) {
      const paths = u.paths.filter(there);
      if (paths.length === u.paths.length) continue;
      const written = paths.map((p) => `${u.indent}${u.attrs}${u.pub}use ${p.join("::")};`).join("\n");
      text = text.slice(0, u.start) + written + text.slice(u.end);
    }
    writeFileSync(fileOf(parent), text);
  }
}

/** A Cargo.toml for the prepared copy at `dir` of only the external crates
 * `crates` its modules use, as upstream's has each: the whole list would
 * build `rand`'s `getrandom` for every one. `big-math`, upstream's default,
 * where `num-bigint` or `num-traits` is one. */
function writeManifest(dir: string, crates: string[]) {
  const text = readFileSync(join(crate, "Cargo.toml"), "utf8");
  const section = (name: string) => text.match(new RegExp(`\\[${name}\\]\\n([\\s\\S]*?)(?=\\n\\[|$)`))?.[1] ?? "";
  const wanted = new Set(crates.map((c) => c.replaceAll("_", "-")));
  const big = wanted.has("num-bigint") || wanted.has("num-traits");
  if (big) wanted.add("num-bigint").add("num-traits");
  const pick = (lines: string) =>
    lines.split("\n").filter((line) => wanted.has(line.split("=")[0].trim().replaceAll("_", "-"))).join("\n");
  const manifest = [
    `[package]\nname = "the_algorithms_rust"\nversion = "0.1.0"\nedition = "2021"\n`,
    `[dependencies]\n${pick(section("dependencies"))}\n`,
    `[dev-dependencies]\n${pick(section("dev-dependencies"))}\n`,
    big ? `[features]\ndefault = ["big-math"]\nbig-math = ["dep:num-bigint", "dep:num-traits"]\n` : "",
  ].join("\n");
  writeFileSync(join(dir, "Cargo.toml"), manifest);
  cpSync(join(crate, "Cargo.lock"), join(dir, "Cargo.lock"));
}

/** The prepared Cargo crate's tests natively, as `nativeOf`'s are, built by
 * Cargo for wasm32-wasip1. */
async function cargoNativeOf(dir: string, texts: string, host: Results): Promise<Results | string> {
  const built = spawnSync("cargo", [`+${toolchain}`, "test", "--no-run", "--lib", "--target", "wasm32-wasip1", "--message-format=json", "-q"], {
    cwd: dir,
    encoding: "utf8",
    maxBuffer: 1 << 28,
    timeout: 280_000,
    env: { ...process.env, RUSTFLAGS: "-Coverflow-checks=off -Awarnings", CARGO_TARGET_DIR: join(root, "target", "wasm-native") },
  });
  const wasm = (built.stdout ?? "")
    .split("\n")
    .filter((l) => l.startsWith("{"))
    .map((l) => JSON.parse(l))
    .find((m) => m.reason === "compiler-artifact" && m.profile?.test && m.executable)?.executable;
  if (!wasm) return (built.stderr ?? "").split("\n").filter((l) => l.startsWith("error")).slice(0, 4).join("\n") || "no test binary";
  return runWasmTests(wasm, texts, host);
}

/** Each JS test's outcome, from bun's JUnit report. */
function jsResults(report: string): Results {
  const results: Results = {};
  for (const [, attrs, body] of readFileSync(report, "utf8").matchAll(/<testcase ([^>]*?)(?:\/>|>([\s\S]*?)<\/testcase>)/g)) {
    const name = attrs.match(/name="([^"]*)"/)![1].replace(/&quot;/g, '"').replace(/&amp;/g, "&").replace(/&lt;/g, "<").replace(/&gt;/g, ">");
    results[name] = body && /<(failure|error)/.test(body) ? "fail" : "pass";
  }
  return results;
}

/** One algorithm: built alone, compiled, its tests run and compared. */
// rust-js's toolchain, whose rustc builds the native side as rust-js's does.
const toolchain = readFileSync(join(root, "..", "rust-js", "rust-toolchain.toml"), "utf8").match(/channel = "([^"]+)"/)![1];

/** Each test function of `text` that panics as it passes, which wasm32
 * can't, as it aborts on a panic: a `#[should_panic]` one's name, and one
 * that catches it, `catch_unwind`. */
const panicking = (text: string) => {
  const names = [...text.matchAll(/#\[should_panic[^\]]*\]\s*(?:#\[[^\]]*\]\s*)*fn\s+(\w+)/g)].map((m) => m[1]);
  const fns = text.split(/(?=\bfn\s+\w+)/);
  for (const f of fns) {
    const name = f.match(/^fn\s+(\w+)/)?.[1];
    if (name && /catch_unwind/.test(f)) names.push(name);
  }
  return new Set(names);
};

/** The prepared crate's tests natively, as rust-js's JS should run them:
 * built for wasm32-wasip1, whose `usize` is rust-js's 32 bits, overflow
 * unchecked as rust-js's is (ADR 0090), each run on its own under Bun's
 * WASI. One that panics as it passes aborts there, so its outcome is the
 * host's, `host`. An error where it doesn't build for 32 bits. */
async function nativeOf(dir: string, texts: string, host: Results): Promise<Results | string> {
  const wasm = join(dir, "tests.wasm");
  const built = spawnSync(
    "rustc",
    [`+${toolchain}`, "--edition=2021", "--test", "--target=wasm32-wasip1", "-Coverflow-checks=off", "-Awarnings", "--cfg", 'feature="big-math"', "--crate-name=the_algorithms_rust", join(dir, "src", "lib.rs"), "-o", wasm],
    { encoding: "utf8", timeout: 240_000 },
  );
  if (built.status !== 0) {
    const errors = (built.stderr ?? "").split(/\n(?=error)/).filter((b) => b.startsWith("error") && !/aborting due to/.test(b));
    return errors.slice(0, 2).map((b) => b.split("\n").slice(0, 6).join("\n")).join("\n\n");
  }
  return runWasmTests(wasm, texts, host);
}

/** Each test of the wasm32-wasip1 test binary `wasm`, run on its own under
 * Bun's WASI; one that panics as it passes, `texts` say, the host's. */
async function runWasmTests(wasm: string, texts: string, host: Results): Promise<Results> {
  const runner = join(root, "scripts", "wasi-run.ts");
  const names = spawnSync("bun", [runner, wasm, "--list", "--format", "terse"], { encoding: "utf8" })
    .stdout.split("\n")
    .filter((l) => l.endsWith(": test"))
    .map((l) => l.slice(0, -": test".length));
  const panics = panicking(texts);
  const results: Results = {};
  let next = 0;
  await Promise.all(
    Array.from({ length: 8 }, async () => {
      while (next < names.length) {
        const name = names[next++];
        if (panics.has(name.split("::").at(-1)!) && host[name]) {
          results[name] = host[name];
          continue;
        }
        const p = Bun.spawn(["bun", runner, wasm, name, "--exact", "--test-threads=1"], { stdout: "ignore", stderr: "ignore" });
        const timer = setTimeout(() => p.kill(), 60_000);
        const code = await p.exited;
        clearTimeout(timer);
        results[name] = code === 0 ? "pass" : "fail";
      }
    }),
  );
  return results;
}

async function one(module: string, host: Results): Promise<Status> {
  const kept = needed(module);
  const texts = [...kept].map(readSource).join("\n");
  const crates = externalCrates.filter((c) => new RegExp(`\\b${c}::|use\\s+${c}\\b`).test(texts));
  const dir = join(root, "build", "one", module.replaceAll("::", "-"));
  prepare(dir, kept);
  // `rand`'s `getrandom` has no backend for rust-js's target yet.
  if (crates.includes("rand")) return { status: "needs-crate", detail: crates.join(", ") };
  if (crates.length) writeManifest(dir, crates);
  const native = crates.length ? await cargoNativeOf(dir, texts, host) : await nativeOf(dir, texts, host);
  if (typeof native === "string") return { status: "64-bit", detail: native };
  const report = join(dir, "js", "report.xml");
  mkdirSync(join(dir, "js"), { recursive: true });
  const runner = ["--timeout=60000", "--reporter=junit", `--reporter-outfile=${report}`];
  // Of external crates, Cargo's build, `rust-js test`'s (rust-js's ADR 0914);
  // else the compiler alone, `rust-js --test`, and bun.
  const compiled = crates.length
    ? spawnSync("bun", [rustJsCli, "test", "--", ...runner], {
        cwd: dir,
        encoding: "utf8",
        maxBuffer: 1 << 26,
        timeout: 280_000,
        env: { ...process.env, RUST_JS_COMPILER: rustJs, CARGO_TARGET_DIR: join(root, "target", "rust-js-test") },
      })
    : spawnSync(rustJs, ["--test", join(dir, "src", "lib.rs"), "-o", join(dir, "js", "lib.js"), "--", "--edition=2021", "-Awarnings", "--cfg", 'feature="big-math"'], {
        encoding: "utf8",
        maxBuffer: 1 << 26,
        timeout: 240_000,
      });
  const stderr = compiled.stderr ?? "";
  if (/overflowed its stack|panicked at|SIGSEGV/.test(stderr) || compiled.signal) {
    return { status: "crashed", detail: (compiled.signal ?? "") + stderr.split("\n").filter((l) => /overflowed|panicked|fatal/.test(l)).join("\n") };
  }
  if (!crates.length && compiled.status !== 0 || crates.length && !existsSync(report)) {
    const errors = stderr.split(/\n(?=error)/).filter((b) => b.startsWith("error") && !/aborting due to|could not compile/.test(b));
    return { status: "refused", detail: errors.slice(0, 3).map((b) => b.split("\n").slice(0, 3).join("\n")).join("\n\n") || stderr.slice(0, 600) };
  }
  if (!crates.length) {
    spawnSync("bun", ["test", ...runner, join(dir, "js", "lib.test.js")], { cwd: root, encoding: "utf8", timeout: 280_000 });
  }
  const js = existsSync(report) ? jsResults(report) : {};
  const own = Object.entries(native).filter(([name]) => {
    // A test is the module's whose path, less its function and any `mod
    // tests`, is the module itself.
    const path = name.split("::").slice(0, -1);
    while (path.length && !modules.has(path.join("::"))) path.pop();
    return path.join("::") === module;
  });
  const differs = own
    .filter(([name, outcome]) => js[name] !== outcome)
    .map(([name, outcome]) => `${name}: native ${outcome}, JS ${js[name] ?? "missing"}`);
  return differs.length ? { status: "differs", detail: differs.join("\n"), tests: own.length } : { status: "same", tests: own.length };
}

/** Upstream's tests natively, each in its own process: one that aborts is its own failure. */
async function native() {
  const built = spawnSync("cargo", ["test", "--no-run", "--message-format=json", "-q"], { cwd: crate, encoding: "utf8", maxBuffer: 1 << 28 });
  const binary = built.stdout
    .split("\n")
    .filter(Boolean)
    .map((l) => JSON.parse(l))
    .find((m) => m.reason === "compiler-artifact" && m.profile?.test && m.target?.kind?.includes("lib"))?.executable;
  if (!binary) throw new Error(`no test binary:\n${built.stderr}`);
  const names = spawnSync(binary, ["--list", "--format", "terse"], { encoding: "utf8", maxBuffer: 1 << 26 })
    .stdout.split("\n")
    .filter((l) => l.endsWith(": test"))
    .map((l) => l.slice(0, -": test".length));
  const results: Results = {};
  let next = 0;
  await Promise.all(
    Array.from({ length: 8 }, async () => {
      while (next < names.length) {
        const name = names[next++];
        const p = Bun.spawn([binary, "--exact", name, "--test-threads=1"], { stdout: "ignore", stderr: "ignore" });
        const timer = setTimeout(() => p.kill(), 60_000);
        const code = await p.exited;
        clearTimeout(timer);
        results[name] = code === 0 ? "pass" : "fail";
      }
    }),
  );
  mkdirSync(join(root, "build"), { recursive: true });
  writeFileSync(nativeFile, JSON.stringify(results, null, 1));
  console.log(`native: ${names.length} tests, ${Object.values(results).filter((r) => r === "fail").length} failed`);
}

async function run(list: string[]) {
  const host = readJson<Results>(nativeFile, {});
  if (Object.keys(host).length === 0) throw new Error("no native results: run `native` first");
  const statuses = readJson<Statuses>(statusFile, {});
  for (const module of list) {
    const started = Date.now();
    const status = await one(module, host);
    statuses[module] = status;
    writeStatuses(statuses);
    console.log(`${status.status.padEnd(11)} ${module} (${((Date.now() - started) / 1000).toFixed(1)} s)`);
    if (status.detail) console.log(status.detail.replace(/^/gm, "    "));
  }
}

if (import.meta.main) {
  const [step = "list", arg] = process.argv.slice(2);
  if (step === "native") await native();
  if (step === "one") await run([arg]);
  if (step === "next") {
    const statuses = readJson<Statuses>(statusFile, {});
    await run(algorithms().filter((m) => statuses[m]?.status !== "same" && statuses[m]?.status !== "needs-crate").slice(0, Number(arg ?? 1)));
  }
  if (step === "list") {
    const statuses = readJson<Statuses>(statusFile, {});
    const all = algorithms();
    for (const m of all) console.log(`${(statuses[m]?.status ?? "-").padEnd(11)} ${m}`);
    const counts: Record<string, number> = {};
    for (const m of all) counts[statuses[m]?.status ?? "not run"] = (counts[statuses[m]?.status ?? "not run"] ?? 0) + 1;
    console.log(counts);
  }
}
