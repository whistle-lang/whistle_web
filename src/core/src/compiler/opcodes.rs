use crate::lexer::*;

// https://webassembly.github.io/spec/core/binary/modules.html#sections
pub enum Section {
  Custom = 0,
  Type = 1,
  Import = 2,
  Func = 3,
  Table = 4,
  Memory = 5,
  Global = 6,
  Export = 7,
  Start = 8,
  Element = 9,
  Code = 10,
  Data = 11
}

// https://webassembly.github.io/spec/core/binary/types.html
#[derive(Clone)]
pub enum Valtype {
  I32 = 0x7f,
  F32 = 0x7d
}

// https://webassembly.github.io/spec/core/binary/types.html#binary-blocktype
pub enum Blocktype {
  Void = 0x40
}

// https://webassembly.github.io/spec/core/binary/instructions.html
pub enum Opcodes {
  Block = 0x02,
  Loop = 0x03,
  Br = 0x0c,
  BrIf = 0x0d,
  End = 0x0b,
  Call = 0x10,
  GetLocal = 0x20,
  SetLocal = 0x21,

  I32Store8 = 0x3a,
  I32Const = 0x41,
  I32Add = 0x6a,
  I32Sub = 0x6b,
  I32Mul = 0x6c,
  I32Div = 0x6d,
  I32Ne = 0x47,
  I32Eqz = 0x45,
  I32Eq = 0x46,
  I32And = 0x71,
  I32TruncF3S = 0xa8,
  I32Gt = 0x4a,
  I32Lt = 0x48,
  I32Ge = 0x4e,
  I32Le = 0x4c,

  F32Const = 0x43,
  F32Add = 0x92,
  F32Eq = 0x5b,
  F32Lt = 0x5d,
  F32Gt = 0x5e,
  F32Sub = 0x93,
  F32Mul = 0x94,
  F32Div = 0x95,
}

// http://webassembly.github.io/spec/core/binary/modules.html#export-section
pub enum ExportType {
  Func = 0x00,
  Table = 0x01,
  Mem = 0x02,
  Global = 0x03
}

pub enum Types {
  FunctionType = 0x60,
  EmptyArray = 0x0
}

pub const MAGIC_MODULE_HEADER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
pub const MODULE_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

impl Opcodes {
  pub fn from(op: Operator) -> Opcodes {
    match op {
      Operator::Add => Opcodes::I32Add,
      Operator::Sub => Opcodes::I32Sub,
      Operator::Mul => Opcodes::I32Mul,
      Operator::Div => Opcodes::I32Div,
      Operator::GreaterThan => Opcodes::I32Gt,
      Operator::LessThan => Opcodes::I32Lt,
      Operator::GreaterThanOrEq => Opcodes::I32Ge,
      Operator::LessThanOrEq => Opcodes::I32Le,
      Operator::Eq => Opcodes::I32Eq,
      Operator::NotEq => Opcodes::I32Ne,
      _ => Opcodes::End,
    }
  }
}
