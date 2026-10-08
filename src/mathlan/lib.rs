use std::{arch::asm, collections::HashMap, fmt};

#[derive(Debug)]
pub struct Error {
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {})", self.message)
    }
}

impl std::error::Error for Error {}

#[derive(Default, Debug, PartialEq)]
pub enum OpCode {
    #[default] OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpWord,
    OpPush,
    OpSmaller,
    OpEqual,
    OpLarger,
    OpIf,
    OpElse,
    OpEnd,
    OpSwap,
    OpRot,
    OpDup,
    OpDrop,
    OpJmp,
    OpCall,
    OpRet,
    OpStore,
    OpLoad,
    OpHalt,
}

#[derive(Default, Debug)]
pub struct Operation {
    pub op_code: OpCode,
    pub value: i64, // Standard: 0
    pub name: String, // Standard: ""
}

#[derive(Debug)]
pub struct Program {
    pub operations: Vec<Operation>,
    pub labels: HashMap<String, usize>,
}

#[allow(dead_code)]
pub fn print_text(text: &str) -> Result<(), Error> {
    let text_length: u64 = text.len() as u64;
    let mut result: u64;
    unsafe {
        asm!(
            "mov x0, #1",
            "mov x1, {0}",
            "mov x2, {1}",
            "mov x16, #4",
            "svc #0x80;",
            in(reg) text.as_ptr(),
            in(reg) text_length,
            out("x0") result,
            out("x1") _,
            out("x2") _,
            out("x3") _,
            
        );
    }

    if result != text_length {
        return Err(Error{ message: "Writing failed".to_string()});
    }
    Ok(())
}