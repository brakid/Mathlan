use std::{collections::HashMap, fmt::format};

use super::lib::{Error, Label, LabelType, LabelType::{Call, Jump, Undefined}, OpCode, Operation, Program};

pub fn parse(program_code: &str) -> Result<Program, Error> {
    let mut operations: Vec<Operation> = vec![];
    let mut labels: HashMap<String, Label> = HashMap::new();
    let mut constants: HashMap<String, i64> = HashMap::new();
    
    let lines = program_code.trim().lines();
    for (line_index, line) in lines.into_iter().enumerate() {
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        let op_codes: Vec<&str> = line.split(" ").into_iter().collect();
        let mut last_label: Option<String> = None;

        let mut index = 0;
        while index < op_codes.len() {
            let op_code = op_codes[index].trim();
            if op_code.len() == 0 {
                index += 1;
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
                "jump" => {
                    let new_last_label = last_label.take();
                    
                    if new_last_label.is_some() {
                        let label_name = new_last_label.ok_or(Error{ message: format(format_args!("[line: {}, token index: {}] No preceeding label found", line_index, index)) })?;
                        let label = labels.get_mut(&label_name).ok_or(Error{ message: format(format_args!("[line: {}, token index: {}] No label with name found: {}", line_index, index, label_name)) })?;
                        match &label.label_type {
                            Undefined | Jump => label.label_type = Jump,
                            t => {
                                return Err(Error{ message: format(format_args!("[line: {}, token index: {}] Jump Label {} is already set to type: {:?}", line_index, index, label_name, t)) });
                            }
                        }
                        operations.push(Operation { op_code: OpCode::OpJmp, ..Operation::default() });
                        
                    } else {
                        return Err(Error{ message: format(format_args!("[line: {}, token index: {}] No label preceeding jump", line_index, index)) });
                    }
                },
                "store" => operations.push(Operation { op_code: OpCode::OpStore, ..Operation::default() }),
                "load" => operations.push(Operation { op_code: OpCode::OpLoad, ..Operation::default() }),
                "call" => {
                    let new_last_label = last_label.take();
                    
                    if new_last_label.is_some() {
                        let label_name = new_last_label.ok_or(Error{ message: format(format_args!("[line: {}, token index: {}] No preceeding label found", line_index, index)) })?;
                        let label = labels.get_mut(&label_name).ok_or(Error{ message: format(format_args!("[line: {}, token index: {}] No label with name found: {}", line_index, index, label_name)) })?;
                        match &label.label_type {
                            Undefined | Call => label.label_type = Call,
                            t => {
                                return Err(Error{ message: format(format_args!("[line: {}, token index: {}] Call Label {} is already set to type: {:?}", line_index, index, label_name, t)) });
                            }
                        }
                        operations.push(Operation { op_code: OpCode::OpCall, ..Operation::default() });
                        
                    } else {
                        return Err(Error{ message: format(format_args!("[line: {}, token index: {}] No label preceeding jump", line_index, index)) });
                    }
                },
                "ret" => operations.push(Operation { op_code: OpCode::OpRet, ..Operation::default() }),
                "halt" => operations.push(Operation { op_code: OpCode::OpHalt, ..Operation::default() }),
                o if o.parse::<i64>().is_ok() => {
                    let value: i64 = o.parse().map_err(|_| Error{ message: format(format_args!("[line: {}, token index: {}] Expected i64 number: {}", line_index, index, o)) })?;
                    operations.push(Operation { op_code: OpCode::OpPush, value: value, ..Operation::default() });
                }
                o if o.ends_with(":") => {
                    let mut label_name = o.trim().to_string();
                    label_name.pop();
                    labels.insert(label_name, Label { location: operations.len(), label_type: LabelType::Undefined });
                }
                o if o.ends_with("=") => {
                    let mut constant_name = o.trim().to_string();
                    constant_name.pop();
                    let constant_value_string = op_codes[index+1].trim();
                    let constant_value: i64 = constant_value_string.parse().map_err(|_| Error{ message: format(format_args!("[line: {}, token index: {}] Expected i64 number: {}", line_index, index, constant_value_string)) })?;
                    constants.insert(constant_name, constant_value);
                    index += 1;
                }
                o if o.trim_matches('\'').parse::<char>().is_ok() => {
                    let c = o.trim_matches('\'').parse::<char>().map_err(|_| Error{ message: format(format_args!("[line: {}, token index: {}] Expected char: {}", line_index, index, o)) })?;
                    operations.push(Operation { op_code: OpCode::OpPush, value: c as i64, ..Operation::default() });
                }
                o if constants.contains_key(&o.to_string()) => {
                    operations.push(Operation { op_code: OpCode::OpPush, name: o.to_string(), ..Operation::default() });
                }
                o => {
                    operations.push(Operation { op_code: OpCode::OpWord, name: o.to_string(), ..Operation::default() });
                    last_label = Option::Some(o.to_string());
                },
            }
            println!("{}, {}, {}", line_index, index, op_code);
            index += 1;
        }
    }

    Ok(Program { operations: operations, labels: labels, constants: constants })
}