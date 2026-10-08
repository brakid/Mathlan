use std::{collections::HashMap, fmt::format};

use super::lib::{Error, OpCode, Operation, Program};

pub fn parse(program_code: &str) -> Result<Program, Error> {
    let mut operations: Vec<Operation> = vec![];
    let mut labels: HashMap<String, usize> = HashMap::new();
    
    let lines = program_code.trim().lines();
    for (line_index, line) in lines.into_iter().enumerate() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let op_codes = line.split(" ").into_iter();
        for (index, op_code) in op_codes.enumerate() {
            let op_code = op_code.trim();
            if op_code.len() == 0 {
                continue;
            }
            if op_code.starts_with("#") {
                break;
            }
            match op_code {
                "+" => operations.push(Operation { op_code: OpCode::OpAdd, ..Operation::default() }),
                "-" => operations.push(Operation { op_code: OpCode::OpSub, ..Operation::default() }),
                "*" => operations.push(Operation { op_code: OpCode::OpMul, ..Operation::default() }),
                "/" => operations.push(Operation { op_code: OpCode::OpDiv, ..Operation::default() }),
                "%" => operations.push(Operation { op_code: OpCode::OpMod, ..Operation::default() }),
                "<" => operations.push(Operation { op_code: OpCode::OpSmaller, ..Operation::default() }),
                "=" => operations.push(Operation { op_code: OpCode::OpEqual, ..Operation::default() }),
                ">" => operations.push(Operation { op_code: OpCode::OpLarger, ..Operation::default() }),
                "if" => operations.push(Operation { op_code: OpCode::OpIf, ..Operation::default() }),
                "else" => operations.push(Operation { op_code: OpCode::OpElse, ..Operation::default() }),
                "end" => operations.push(Operation { op_code: OpCode::OpEnd, ..Operation::default() }),
                "swap" => operations.push(Operation { op_code: OpCode::OpSwap, ..Operation::default() }),
                "rot" => operations.push(Operation { op_code: OpCode::OpRot, ..Operation::default() }),
                "dup" => operations.push(Operation { op_code: OpCode::OpDup, ..Operation::default() }),
                "drop" => operations.push(Operation { op_code: OpCode::OpDrop, ..Operation::default() }),
                "jump" => operations.push(Operation { op_code: OpCode::OpJmp, ..Operation::default() }),
                "store" => operations.push(Operation { op_code: OpCode::OpStore, ..Operation::default() }),
                "load" => operations.push(Operation { op_code: OpCode::OpLoad, ..Operation::default() }),
                "call" => operations.push(Operation { op_code: OpCode::OpCall, ..Operation::default() }),
                "ret" => operations.push(Operation { op_code: OpCode::OpRet, ..Operation::default() }),
                "halt" => operations.push(Operation { op_code: OpCode::OpHalt, ..Operation::default() }),
                o if o.parse::<i64>().is_ok() => {
                    let value: i64 = o.parse().map_err(|_| Error{ message: format(format_args!("[line: {}, token index: {}] Expected i64 number: {}", line_index, index, o)) })?;
                    operations.push(Operation { op_code: OpCode::OpPush, value: value, ..Operation::default() });
                }
                o if o.ends_with(":") => {
                    let mut label_name = o.trim().to_string();
                    label_name.pop();
                    labels.insert(label_name, operations.len());
                }
                o if o.trim_matches('\'').parse::<char>().is_ok() => {
                    let c = o.trim_matches('\'').parse::<char>().map_err(|_| Error{ message: format(format_args!("[line: {}, token index: {}] Expected char: {}", line_index, index, o)) })?;
                    operations.push(Operation { op_code: OpCode::OpPush, value: c as i64, ..Operation::default() });
                }
                o => {
                    operations.push(Operation { op_code: OpCode::OpWord, name: o.to_string(), ..Operation::default() });
                },
            }
        }
    }

    Ok(Program { operations: operations, labels: labels })
}