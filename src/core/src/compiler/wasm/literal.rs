use super::super::*;
use crate::parser::*;

pub fn compile_literal(compiler: &mut Compiler, lit: Literal) {
  match lit {
    Literal::Int(int) => compile_int_literal(compiler, int),
    _ => print!("lit"),
  }
}

pub fn compile_int_literal(compiler: &mut Compiler, int: usize) {
  compiler.func().code.push(Opcodes::I32Const as u8);
  compiler.func().code.extend(unsigned_leb128(int));
}
