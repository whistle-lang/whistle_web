use super::super::*;
use crate::lexer::*;
use crate::parser::*;

pub fn compile_expr(compiler: &mut Compiler, expr: Expr) {
	match expr {
		Expr::Binary { op, rhs, lhs } => compile_bin_expr(compiler, op, rhs, lhs),
		Expr::Unary(expr) => compile_un_expr(compiler, expr),
		_ => print!("exp"),
	}
}

pub fn compile_bin_expr(compiler: &mut Compiler, op: Operator, rhs: Box<Expr>, lhs: Box<Expr>) {
	match op {
		Operator::Assign => compile_assign(compiler, *rhs, *lhs),
		_ => compile_arithmetic(compiler, op, *rhs, *lhs)
	}
}

pub fn compile_assign(compiler: &mut Compiler, rhs: Expr, lhs: Expr) {
	//FIXME: lhs should be an identifier not an expression
	compile_expr(compiler, rhs);
	if let Expr::Unary(UnaryExpr::Primary(PrimaryExpr::Operand(Operand::Ident(idt)))) = lhs {
		compiler.func().code.push(Opcodes::SetLocal as u8);
		let var = compiler.get_var(idt);
		compiler.func().code.extend(unsigned_leb128(var));
	}
}

pub fn compile_arithmetic(compiler: &mut Compiler, op: Operator, rhs: Expr, lhs: Expr) {
	compile_expr(compiler, lhs);
	compile_expr(compiler, rhs);
	compiler.func().code.push(Opcodes::from(op) as u8);
}

pub fn compile_un_expr(compiler: &mut Compiler, expr: UnaryExpr) {
	match expr {
		UnaryExpr::Primary(expr) => compile_prim_expr(compiler, expr),
		_ => print!("un"),
	}
}

pub fn compile_prim_expr(compiler: &mut Compiler, expr: PrimaryExpr) {
	match expr {
		PrimaryExpr::Operand(op) => compile_operand(compiler, op),
		PrimaryExpr::Arguments { prim, args } => compile_arguments(compiler, *prim, args),
		_ => print!("prim"),
	}
}

pub fn compile_operand(compiler: &mut Compiler, op: Operand) {
	match op {
		Operand::Ident(idt) => compile_ident(compiler, idt),
		Operand::Literal(lit) => compile_literal(compiler, lit),
		Operand::Grouping(grp) => compile_expr(compiler, *grp),
	}
}

pub fn compile_ident(compiler: &mut Compiler, idt: String) {
	compiler.func().code.push(Opcodes::GetLocal as u8);
	let var = compiler.get_var(idt);
	compiler.func().code.extend(unsigned_leb128(var));
}

pub fn compile_arguments(compiler: &mut Compiler, prim: PrimaryExpr, args: Vec<Expr>) {
	for expr in args {
		compile_expr(compiler, expr)
	}
	//FIXME: function name should be an identifier not an expression
	if let PrimaryExpr::Operand(Operand::Ident(idt)) = prim {
		compiler.func().code.push(Opcodes::Call as u8);
		let var = compiler.get_func(idt);
		compiler.func().code.extend(unsigned_leb128(var));
	}
}
