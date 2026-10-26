// trekhleb/javascript-algorithms, written in Rust and compiled by rust-js:
// each JS module of `src/` a Rust module at the same path in `rust/src/`,
// its directories snake_case, its file's name too, a directory's own,
// `factorial/factorial.js`, its `mod.rs`. The original's own tests
// run against the compiled JS, a module at the original's path re-exporting
// it, and against the original, each test's outcome compared.
//
//   bun scripts/javascript-algorithms.ts
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative } from "node:path";
import { spawnSync } from "node:child_process";

const root = join(import.meta.dir, "..");
const port = join(root, "javascript-algorithms");
const rustSrc = join(port, "rust", "src");
const build = join(root, "build", "javascript-algorithms");
const statusFile = join(port, "status.json");
const rustJsCli = join(root, "..", "rust-js", "tooling", "cli.js");

type Results = Record<string, "pass" | "fail">;

/** `fast_powering` as the original names a directory, `fast-powering`. */
const kebab = (name: string) => name.replaceAll("_", "-");
/** `fast_powering` as the original names a file, `fastPowering`. */
const camel = (name: string) => name.replace(/_([a-z0-9])/g, (_, c) => c.toUpperCase());

/** Each Rust module of `rust/src` but `lib.rs`: the original's JS path
 * it's the port of, and its compiled JS. */
function ports(): { original: string; compiled: string; rust: string }[] {
  const found: { original: string; compiled: string; rust: string }[] = [];
  (function walk(dir: string) {
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      const path = join(dir, entry.name);
      if (entry.isDirectory()) walk(path);
      else if (entry.name.endsWith(".rs") && entry.name !== "lib.rs") {
        const rel = relative(rustSrc, path).split("/");
        let file = rel.pop()!.replace(/\.rs$/, "");
        // A directory's own, `factorial/factorial.js`, is its `mod.rs`, which
        // clippy has rather than `factorial/factorial.rs`; one of `mod`s
        // alone is no module of the original's, and has no JS.
        if (file === "mod") {
          if (!existsSync(path.replace(/\.rs$/, ".js"))) continue;
          file = rel.at(-1)!;
        }
        const original = join("src", ...rel.map(kebab), `${camel(file)}.js`);
        found.push({ original, compiled: path.replace(/\.rs$/, ".js"), rust: relative(port, path) });
      }
    }
  })(rustSrc);
  return found;
}

/** Each test's outcome, from bun's JUnit report: its suite's name and its own. */
function results(report: string): Results {
  const found: Results = {};
  if (!existsSync(report)) return found;
  for (const [, attrs, body] of readFileSync(report, "utf8").matchAll(/<testcase ([^>]*?)(?:\/>|>([\s\S]*?)<\/testcase>)/g)) {
    const name = attrs.match(/name="([^"]*)"/)![1];
    const suite = attrs.match(/classname="([^"]*)"/)?.[1] ?? "";
    const file = attrs.match(/file="([^"]*)"/)?.[1] ?? "";
    found[`${basename(file)} > ${suite} > ${name}`] = body && /<(failure|error)/.test(body) ? "fail" : "pass";
  }
  return found;
}

/** The tests of `tests`, in the tree at `tree`, run by bun. */
function run(tree: string, tests: string[]): Results {
  const report = join(tree, "report.xml");
  rmSync(report, { force: true });
  spawnSync("bun", ["test", "--timeout=60000", "--reporter=junit", `--reporter-outfile=${report}`, ...tests.map((t) => join(tree, t))], {
    cwd: tree,
    encoding: "utf8",
    timeout: 280_000,
  });
  return results(report);
}

if (import.meta.main) {
  // The ports' JS, beside their Rust, as `rust-js build` writes it.
  const built = spawnSync("bun", [rustJsCli, "build"], { cwd: join(port, "rust"), encoding: "utf8", timeout: 280_000 });
  if (built.status !== 0) {
    console.log(built.stderr);
    process.exit(1);
  }
  const all = ports();
  const original = join(build, "original"), ported = join(build, "ported");
  for (const tree of [original, ported]) {
    rmSync(tree, { recursive: true, force: true });
    mkdirSync(tree, { recursive: true });
    cpSync(join(port, "src"), join(tree, "src"), { recursive: true });
  }
  const tests: string[] = [];
  const missing: string[] = [];
  for (const { original: js, compiled } of all) {
    if (!existsSync(join(port, js))) {
      missing.push(js);
      continue;
    }
    // The original's module, at its path, the port's export.
    writeFileSync(join(ported, js), `export { default } from ${JSON.stringify(compiled)};\n`);
    const test = join(dirname(js), "__test__", basename(js).replace(/\.js$/, ".test.js"));
    if (existsSync(join(port, test))) tests.push(test);
  }
  const expected = run(original, tests);
  const actual = run(ported, tests);
  const status: Record<string, string> = {};
  let same = 0;
  for (const [name, outcome] of Object.entries(expected).sort()) {
    status[name] = actual[name] === outcome ? "same" : `original ${outcome}, port ${actual[name] ?? "missing"}`;
    if (actual[name] === outcome) same++;
  }
  writeFileSync(statusFile, JSON.stringify({ ports: all.length, tests: Object.keys(expected).length, same, status }, null, 2) + "\n");
  console.log(`${all.length} ports, ${Object.keys(expected).length} tests, ${same} the same`);
  for (const [name, s] of Object.entries(status)) if (s !== "same") console.log(`  ${name}: ${s}`);
  if (missing.length) console.log(`no original of: ${missing.join(", ")}`);
}
