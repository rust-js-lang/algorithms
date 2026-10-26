// A binary built for wasm32-wasip1, run once under Bun's WASI with the
// arguments given: what it prints, and its exit code, a trap's 134 as an
// abort's (wasm32-wasip1 aborts on a panic).
//
//   bun scripts/wasi-run.ts <binary.wasm> [args...]
import { WASI } from "node:wasi";
import { readFileSync } from "node:fs";

const [wasm, ...args] = process.argv.slice(2);
const wasi = new WASI({ version: "preview1", args: [wasm, ...args], env: {}, returnOnExit: true });
let code: number;
try {
  const module = await WebAssembly.compile(readFileSync(wasm));
  const instance = await WebAssembly.instantiate(module, { wasi_snapshot_preview1: wasi.wasiImport });
  code = wasi.start(instance) ?? 0;
} catch {
  code = 134;
}
process.exit(code);
