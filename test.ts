import { assertEquals } from "./test_deps.ts";
import { lex, parse } from "./mod.ts";

Deno.test({
  name: "lex",
  fn: () => {
    let code = "val x: i32 = 1 + 2 * 2 ** 2"
    console.log(`lexer: ${lex(code)}`);
    console.log(`parser: ${parse(code)}`);
  },
});
