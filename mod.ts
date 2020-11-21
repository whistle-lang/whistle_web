import init, { lex as wasmLex, parse as wasmParse, source } from "./wasm.js";

await init(source);

export function lex(text:string): any {
  return wasmLex(text);
}
export function parse(text:string): any {
  return wasmParse(text);
}