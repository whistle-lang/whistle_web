//#[macro_use]
use wasm_bindgen::prelude::wasm_bindgen;
//use whistle_compiler::*;
//use whistle_ast::Grammar;
//use whistle_common::TokenItem;
use whistle_scripts::lexthing;
use whistle_lexer::*;
use whistle_parser::*;
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
  let tokens = lexthing(&text);
  let parser = &mut Parser::new(tokens);

  match parse_all(parser) {
    Ok(val) => {
      format!("{:#?}", val)
    }
    Err(err) => {
      format!("{:?}", err)
    }
  }
}

