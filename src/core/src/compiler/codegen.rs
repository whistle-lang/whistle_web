use super::encoder::*;
use super::opcodes::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Compiler {
  pub refs: HashMap<String, usize>,
  pub funcs: Vec<Function>,
  pub name: String
}

#[derive(Clone)]
pub struct Function {
  pub vars: HashMap<String, usize>,
  pub param_types: Vec<u8>,
  pub result: Vec<u8>,
  pub code: Vec<u8>
}

impl Function {
  pub fn new() -> Function {
    Function { 
      vars: HashMap::new(),
      param_types: vec![],
      result: vec![],
      code: vec![]
    }
  }
}

impl Compiler {
  pub fn new() -> Self {
    Self {
      refs: HashMap::new(),
      funcs: vec![],
      name: String::new()
    }
  }

  pub fn func(&mut self) -> &mut Function {
    let len = self.funcs.len();
    &mut self.funcs[len - 1]
  }

  pub fn set_param (&mut self, name: String) {
    let len = self.func().vars.len();
    self.func().vars.insert(name.clone(), len);
    self.func().param_types.push(Valtype::I32 as u8);
  }

  pub fn set_var (&mut self, name: String) {
    let len = self.func().vars.len();
    let offset = self.func().param_types.len();
    self.func().vars.insert(name.clone(), len + offset);
  }

  pub fn get_var (&mut self, name: String) -> usize {
    if self.func().vars.contains_key(&name) {
      if let Some(res) = self.func().vars.get(&name) {
        return *res
      }
    }
    panic!("Variable undefined!")
  }

  pub fn get_func (&mut self, name: String) -> usize {
    if self.refs.contains_key(&name) {
      if let Some(res) = self.refs.get(&name) {
        return *res
      }
    }
    panic!("Function undefined!")
  }

  pub fn set_func (&mut self, name: String) {
    let len = self.refs.len();
    self.refs.insert(name.clone(), len);
  }
}

pub fn type_section(compiler: &mut Compiler) -> Vec<u8> {
  let mut res = vec![];
  for func in &compiler.funcs {
    let mut resfunc = vec![Types::FunctionType as u8];
    resfunc.extend(encode_vector(func.param_types.to_vec()));
    resfunc.extend(encode_vector(func.result.to_vec()));
    res.push(resfunc);
  }
  let body = encode_flatten(res);
  create_section(Section::Type, body)
}

pub fn func_section(compiler: &mut Compiler) -> Vec<u8> {
  let mut res = vec![];
  for i in 0..compiler.funcs.len() {
    res.push(unsigned_leb128(i));
  }
  let body = encode_flatten(res);
  create_section(Section::Func, body)
}

// pub fn export_section() -> Vec<u8> {
//   let mut res = encode_string("run");
//   res.push(ExportType::Func as u8);
//   res.push(ExportType::Func as u8);
//   let body = encode_flatten(vec![res]);
//   create_section(Section::Export, body)
// }

pub fn code_section(compiler: &mut Compiler) -> Vec<u8> {
  let mut res = vec![];
  for func in &compiler.funcs {
    let mut code = encode_locals(func);
    code.extend(&func.code);
    code.push(Opcodes::End as u8);
    res.push(encode_vector(code));
  }
  let body = encode_flatten(res);
  create_section(Section::Code, body)
}

pub fn test_main(compiler: &mut Compiler) -> Vec<u8> {
  let mut header = MAGIC_MODULE_HEADER.to_vec();
  header.extend(MODULE_VERSION.to_vec());
  header.extend(type_section(compiler));
  header.extend(func_section(compiler));
  // header.extend(export_section());
  header.extend(code_section(compiler));
  header
}
