// TheAlgorithms/rust, compiled to JavaScript by rust-js: its own #[test]s,
// run natively and as JS, compared test by test.
//
//   bun scripts/the-algorithms-rust.ts [prepare|native|js|report|all]
//
// The upstream crate is vendored unchanged in the-algorithms-rust/. A module
// rust-js can't compile yet is left out of the copy this builds, with the
// reason, in the-algorithms-rust/excluded.json: each is a rust-js gap, to be
// fixed in rust-js, after which its entry goes.
import { cpSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { spawnSync } from "node:child_process";

const root = join(import.meta.dir, "..");
const crate = join(root, "the-algorithms-rust");
const build = join(root, "build", "the-algorithms-rust");
const out = join(root, "build", "the-algorithms-rust-js");
const excludedFile = join(crate, "excluded.json");
const rustJs = process.env.RUST_JS ?? join(root, "..", "rust-js", "target", "debug", "rust-js");

type Excluded = Record<string, string>;
const readExcluded = (): Excluded => (existsSync(excludedFile) ? JSON.parse(readFileSync(excludedFile, "utf8")) : {});
const writeExcluded = (excluded: Excluded) =>
  writeFileSync(excludedFile, JSON.stringify(Object.fromEntries(Object.entries(excluded).sort()), null, 2) + "\n");

/** The file that declares module `path`'s last part: its parent's. */
function parentFile(src: string, path: string[]): string {
  const parent = path.slice(0, -1);
  if (parent.length === 0) return join(src, "lib.rs");
  const dir = join(src, ...parent);
  return existsSync(join(dir, "mod.rs")) ? join(dir, "mod.rs") : `${dir}.rs`;
}

/** Leaves module `name` out of `file`: its `mod` and what re-exports it. */
function leaveOut(file: string, name: string) {
  const text = readFileSync(file, "utf8");
  const statements = text.split(/(?<=;)/);
  const declares = new RegExp(`^\\s*(#\\[[^\\]]*\\]\\s*)*(pub(\\([^)]*\\))?\\s+)?mod\\s+${name}\\s*;\\s*$`);
  const reexports = new RegExp(`^\\s*(pub(\\([^)]*\\))?\\s+)?use\\s+(self::)?${name}::`);
  const kept = statements.filter((s) => !declares.test(s) && !reexports.test(s));
  writeFileSync(file, kept.join(""));
}

/** Leaves out re-exports in `file` of what's at `rest` below it, `probabilistic::bloom_filter`'s. */
function leaveOutReexports(file: string, rest: string[]) {
  const text = readFileSync(file, "utf8");
  const reexports = new RegExp(`^\\s*(pub(\\([^)]*\\))?\\s+)?use\\s+(self::)?${rest.join("::")}::`);
  writeFileSync(file, text.split(/(?<=;)/).filter((s) => !reexports.test(s)).join(""));
}

function prepare(excluded: Excluded = readExcluded()) {
  rmSync(build, { recursive: true, force: true });
  mkdirSync(build, { recursive: true });
  cpSync(join(crate, "src"), join(build, "src"), { recursive: true });
  const src = join(build, "src");
  for (const module of Object.keys(excluded)) {
    const path = module.split("::");
    leaveOut(parentFile(src, path), path.at(-1)!);
    // And each ancestor's re-export of it: for `a::b::c`, `a`'s of
    // `b::c::..`, the root's of `a::b::c::..`.
    for (let k = path.length - 1; k >= 1; k--) {
      leaveOutReexports(parentFile(src, path.slice(0, k)), path.slice(k - 1));
    }
  }
}

/** The module a file of the prepared copy is. */
function moduleOf(file: string): string | undefined {
  const rel = relative(join(build, "src"), file).replace(/\.rs$/, "").split("/");
  if (rel[0] === ".." || rel.join("/") === "lib") return undefined;
  if (rel.at(-1) === "mod") rel.pop();
  return rel.join("::");
}

function compileJs(): string {
  rmSync(out, { recursive: true, force: true });
  mkdirSync(out, { recursive: true });
  const r = spawnSync(rustJs, ["--test", join(build, "src", "lib.rs"), "-o", join(out, "lib.js"), "--", "--edition=2021", "-Awarnings"], {
    encoding: "utf8",
    maxBuffer: 1 << 28,
  });
  return r.status === 0 ? "" : r.stderr;
}

/** Each error's headline and the module of the first file it points at. */
function errors(stderr: string): { module: string | undefined; message: string }[] {
  const found = [];
  for (const block of stderr.split(/\n(?=error)/)) {
    if (!block.startsWith("error")) continue;
    const message = block.split("\n")[0].replace(/^error(\[E\d+\])?: /, "");
    if (/aborting due to|could not compile/.test(message)) continue;
    const at = block.match(/-->\s+(\S+?):\d+:\d+/);
    found.push({ module: at ? moduleOf(at[1]) : undefined, message });
  }
  return found;
}

/** Prepares and compiles, leaving out each module an error is in, till it compiles. */
function prepareUntilItCompiles() {
  for (let round = 1; ; round++) {
    prepare();
    const stderr = compileJs();
    if (!stderr) {
      console.log(`compiled, round ${round}`);
      return;
    }
    const excluded = readExcluded();
    const found = errors(stderr).filter((e) => e.module !== undefined);
    if (found.length === 0) {
      console.log(stderr.slice(0, 4000));
      throw new Error("rust-js failed with no module to leave out");
    }
    for (const { module, message } of found) excluded[module!] ??= message;
    writeExcluded(excluded);
    console.log(`round ${round}: left out ${new Set(found.map((e) => e.module)).size} more`);
  }
}

type Results = Record<string, "pass" | "fail">;
const nativeFile = join(root, "build", "native.json");
const jsFile = join(root, "build", "js.json");

/** Runs `tasks` with at most `width` at once. */
async function pool<T>(tasks: (() => Promise<T>)[], width: number): Promise<T[]> {
  const results: T[] = new Array(tasks.length);
  let next = 0;
  await Promise.all(
    Array.from({ length: width }, async () => {
      while (next < tasks.length) {
        const i = next++;
        results[i] = await tasks[i]();
      }
    }),
  );
  return results;
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
  await pool(
    names.map((name) => async () => {
      const p = Bun.spawn([binary, "--exact", name, "--test-threads=1"], { stdout: "ignore", stderr: "ignore" });
      const timer = setTimeout(() => p.kill(), 60_000);
      const code = await p.exited;
      clearTimeout(timer);
      results[name] = code === 0 ? "pass" : "fail";
    }),
    8,
  );
  writeFileSync(nativeFile, JSON.stringify(results, null, 1));
  const failed = Object.values(results).filter((r) => r === "fail").length;
  console.log(`native: ${names.length} tests, ${failed} failed`);
}

if (import.meta.main) {
  const step = process.argv[2] ?? "prepare";
  if (step === "prepare") prepareUntilItCompiles();
  if (step === "native") await native();
}

export { prepare, prepareUntilItCompiles, readExcluded, build, out, crate };
