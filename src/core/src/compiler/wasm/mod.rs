use crate::parser::*;
use super::codegen::*;
use super::opcodes::*;
mod expr;
pub use expr::*;
mod ident;
pub use ident::*;
mod literal;
pub use literal::*;
mod stmt;
use super::Compiler;
pub use stmt::*;

pub fn compile_program(compiler: &mut Compiler, grammar: Grammar) {
	let mut code = Vec::new();
	for stmt in grammar {
		match stmt {
			Stmt::FunDecl {
				ident,
				params,
				ret_type,
				stmt,
			} => compile_function(compiler, ident, params, ret_type, *stmt),
			_ => code.push(stmt),
		}
	}
	compile_main(compiler, code);
}

pub fn compile_function(
  compiler: &mut Compiler,
  ident: String,
  params: Option<Vec<IdentTyped>>,
  _ret_type: String,
  stmt: Stmt,
) {
	compiler.funcs.push(Function::new());
  	if let Some(res) = params {
		for param in res {
			compiler.set_param(param.ident);
		}
	}
	compiler.set_func(ident);
	compiler.func().result.push(Valtype::I32 as u8);
	compile_stmt(compiler, stmt);
}

pub fn compile_main(compiler: &mut Compiler,stmts: Vec<Stmt>) {
	compiler.funcs.push(Function::new());
	for stmt in stmts {
		compile_stmt(compiler, stmt);
	}
}
