use super::super::*;
use super::expr::*;
use crate::lexer::*;
use crate::parser::*;

pub fn compile_stmt(compiler: &mut Compiler, stmt: Stmt) {
  match stmt {
    // Stmt::Tip(args) => compile_tip(args),
    Stmt::While { cond, do_stmt } => compile_while(compiler, cond, *do_stmt),
    Stmt::ValDecl { ident_typed, val } => compile_var(compiler, ident_typed, *val),
    Stmt::VarDecl { ident_typed, val } => compile_var(compiler, ident_typed, *val),
    Stmt::If { cond, then_stmt, else_stmt } => compile_if(compiler, *cond, *then_stmt, else_stmt),
    Stmt::Expr(args) => compile_expr(compiler, args),
    Stmt::Block(args) => compile_block(compiler, args),
    _ => print!("stmt"),
  }
}

pub fn compile_tip(_tip: Tip) {}

pub fn compile_while(compiler: &mut Compiler, cond: Option<Box<Expr>>, do_stmt: Stmt) {
  compiler.func().code.push(Opcodes::Block as u8);
  compiler.func().code.push(Blocktype::Void as u8);
  compiler.func().code.push(Opcodes::Loop as u8);
  compiler.func().code.push(Blocktype::Void as u8);
  if let Some(expr) = cond {
    compile_expr(compiler, *expr);
    compiler.func().code.push(Opcodes::BrIf as u8);
    compiler.func().code.extend(signed_leb128(1));
  }
  compile_stmt(compiler, do_stmt);
  compiler.func().code.push(Opcodes::Br as u8);
  compiler.func().code.extend(signed_leb128(0));
  compiler.func().code.push(Opcodes::End as u8);
  compiler.func().code.push(Opcodes::End as u8);
}

pub fn compile_if(compiler: &mut Compiler, cond: Expr, then_stmt: Stmt, else_stmt: Option<Box<Stmt>>) {
  compiler.func().code.push(Opcodes::Block as u8);
  compiler.func().code.push(Blocktype::Void as u8);
  compile_expr(compiler, cond.clone());
  compiler.func().code.push(Opcodes::I32Eqz as u8);
  compiler.func().code.push(Opcodes::BrIf as u8);
  compiler.func().code.extend(signed_leb128(0));
  compile_stmt(compiler, then_stmt);
  compiler.func().code.push(Opcodes::End as u8);

  if let Some(stmts) = else_stmt {
    compiler.func().code.push(Opcodes::Block as u8);
    compiler.func().code.push(Blocktype::Void as u8);
    compile_expr(compiler, cond);
    compiler.func().code.push(Opcodes::I32Const as u8);
    compiler.func().code.extend(signed_leb128(1));
    compiler.func().code.push(Opcodes::I32Eq as u8);
    compiler.func().code.push(Opcodes::BrIf as u8);
    compiler.func().code.extend(signed_leb128(0));
    compile_stmt(compiler, *stmts);
    compiler.func().code.push(Opcodes::End as u8);
  }
}

pub fn compile_var(compiler: &mut Compiler, ident: IdentTyped, val: Expr) {
  compile_expr(compiler, val);
  compiler.func().code.push(Opcodes::SetLocal as u8);
  compiler.set_var(ident.ident.clone());
  for elem in unsigned_leb128(compiler.get_var(ident.ident)) {
    compiler.func().code.push(elem);
  }
}

pub fn compile_block(compiler: &mut Compiler, stmts: Vec<Stmt>) {
  for stmt in stmts {
    compile_stmt(compiler, stmt)
  }
}
