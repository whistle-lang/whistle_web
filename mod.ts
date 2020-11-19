import init, { lex as wasmLex, source } from "./wasm.js";

await init(source);

export function lex(text:string): any {
  return wasmLex(text);
}
