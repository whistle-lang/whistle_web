import { assertEquals } from "./test_deps.ts";
import { lex } from "./mod.ts";

Deno.test({
  name: "lex",
  fn: () => {
    console.log(lex("val x: i32 = 1 + 2 * 2 ** 2"));
  },
});
