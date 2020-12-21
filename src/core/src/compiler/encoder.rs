use super::{Function, Section, Types, Valtype};

pub fn signed_leb128(i: isize) -> Vec<u8> {
  let mut n = i.clone();
  let mut buffer = Vec::new();
  let mut more = true;
  while more {
    let mut byte = (n & 0x7f) as u8;
    n >>= 7;
    if (n == 0 && (byte & 0x40) == 0) || (n == -1 && (byte & 0x40) != 0) {
      more = false;
    } else {
      byte |= 0x80;
    }
    buffer.push(byte);
  }
  buffer
}

pub fn unsigned_leb128(i: usize) -> Vec<u8> {
  let mut n = i.clone();
  let mut buffer = Vec::new();
  loop {
    let mut byte = (n & 0x7f) as u8;
    n >>= 7;
    if n != 0 {
      byte |= 0x80;
    }
    buffer.push(byte);
    if n == 0 {
      break;
    }
  }
  buffer
}

// // https://webassembly.github.io/spec/core/binary/conventions.html#binary-vec
// // Vectors are encoded with their length followed by their element sequence

pub fn encode_string(s: &str) -> Vec<u8> {
  let chars: Vec<u8> = s.chars().map(|i| i as u8).collect();
  let mut res = vec![chars.len() as u8];
  res.extend(chars);
  res
}

// // https://webassembly.github.io/spec/core/binary/conventions.html#binary-vec
// // Vectors are encoded with their length followed by their element sequence
pub fn encode_vector(data: Vec<u8>) -> Vec<u8> {
  let mut res = unsigned_leb128(data.len());
  res.extend(data);
  res
}

pub fn encode_flatten(data: Vec<Vec<u8>>) -> Vec<u8> {
  let mut res = unsigned_leb128(data.len());
  let vec: Vec<u8> = data.into_iter().flatten().collect();
  res.extend(vec);
  res
}

// // https://webassembly.github.io/spec/core/binary/modules.html#sections
// // sections are encoded by their type followed by their vector contents
pub fn create_section(section_type: Section, data: Vec<u8>) -> Vec<u8> {
  let mut res = vec![section_type as u8];
  res.extend(encode_vector(data));
  res
}

pub fn encode_locals(func: &Function) -> Vec<u8> {
  if func.vars.len() > 0 {
    let mut vars = unsigned_leb128(func.vars.len());
    vars.push(Valtype::I32 as u8);
    encode_flatten(vec![vars])
  } else {
    vec![Types::EmptyArray as u8]
  }
}
