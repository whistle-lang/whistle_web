#[macro_use]
use wasm_bindgen::prelude::wasm_bindgen;
use whistle_core::compiler::*;
use whistle_core::lexer::*;
use whistle_core::parser::*;
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn lex(text: String) -> String {
    let lexer = Lexer::new(&text);
    let mut toks = Vec::new();
    for tok in lexer {
      match tok {
        Ok(tok) => {
          println!("{:?}", tok);
          toks.push(tok.clone())
        },
        Err(err) => {
          println!("{:?}", err);
          break;
        }
      }
    }

    format!("{:?}", toks)
}
#[wasm_bindgen]
pub fn parse(text: String) -> String {
  let lexer = Lexer::new(&text);
  let mut parser = Parser::from(lexer);
  let ast = parse_grammar(&mut parser);
  format!("{:#?}", ast)
}


#[wasm_bindgen]
pub fn compile(text: String) -> String {
  let lexer = Lexer::new(&text);
  let mut parser = Parser::from(lexer);
  let stmts = parse_grammar(&mut parser);
  let mut compiler = Compiler::new();
  compile_program(&mut compiler, stmts);
  let vec = test_main(&mut compiler);
  format!("{:?}",vec)
}