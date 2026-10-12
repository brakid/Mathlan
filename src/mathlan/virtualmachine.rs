use std::{fmt::format};

use super::lib::{Error, OpCode, Program};

const MEMORY_SIZE: usize = 32000;

pub fn execute(program: &Program, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let operations = &program.operations;
    let mut stack: Vec<i64> = vec![];
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
    let mut string_section_pointer: usize = 0;

    let mut program_counter: usize = program.labels.get("main").ok_or(Error{ message: "Missing main label".to_string() })?.location;
    while program_counter < operations.len() {
        let operation = &operations[program_counter];
        let mut increment_program_counter = true;

        match operation.op_code {
            OpCode::OpAdd => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = op1 + op2;
                stack.push(result);
            }
            OpCode::OpSub => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = op2 - op1;
                stack.push(result);
            }
            OpCode::OpMul => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = op1 * op2;
                stack.push(result);
            }
            OpCode::OpDiv => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = op2 / op1;
                stack.push(result);
            }
            OpCode::OpMod => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = op2 % op1;
                stack.push(result);
            }
            OpCode::OpSmaller => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = if op2 < op1 { 1 } else { 0 };
                stack.push(result);
            }
            OpCode::OpEqual => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = if op2 == op1 { 1 } else { 0 };
                stack.push(result);
            }
            OpCode::OpLarger => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = if op2 > op1 { 1 } else { 0 };
                stack.push(result);
            }
            OpCode::OpAnd => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = if op1 != 0 && op2 != 0 { 1 } else { 0 };
                stack.push(result);
            }
            OpCode::OpOr => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = if op1 != 0 || op2 != 0 { 1 } else { 0 };
                stack.push(result);
            }
            OpCode::OpNot => {
                let op = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let result = if op == 0 { 1 } else { 0 };
                stack.push(result);
            }
            OpCode::OpIf => {
                let op = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                if op != 1 {
                    // find else
                    let if_line = program_counter;
                    let mut search_line = program_counter + 1;
                    let mut nested_ifs = 0;
                    while search_line < operations.len() && ((operations[search_line].op_code != OpCode::OpElse && operations[search_line].op_code != OpCode::OpEnd) || nested_ifs > 0) {
                        //println!("Skipping: {} {:?}", search_line, operations[search_line]);
                        if operations[search_line].op_code == OpCode::OpIf {
                            nested_ifs += 1;
                        }
                        if operations[search_line].op_code == OpCode::OpEnd {
                            nested_ifs -= 1;
                        }
                        search_line += 1;
                    }
                    if search_line == operations.len() || (operations[search_line].op_code != OpCode::OpElse && operations[search_line].op_code != OpCode::OpEnd) {
                        return Err(Box::new(Error { message: format(format_args!("[line: {}] No else branch or end found for if", if_line)) }));
                    }
                    program_counter = search_line;
                    // this leads to landing AFTER the else op_code
                }
            }
            OpCode::OpElse => {
                // if we land in an else then we ran through the if branch - and need to skip to the end of the else branch
                let else_line = program_counter;
                let mut search_line = program_counter + 1;
                let mut nested_ifs = 0;
                while search_line < operations.len() && (operations[search_line].op_code != OpCode::OpEnd || nested_ifs > 0) {
                    //println!("Skipping: {} {:?}", program_counter, operations[program_counter]);
                    if operations[search_line].op_code == OpCode::OpIf {
                        nested_ifs += 1;
                    }
                    if operations[search_line].op_code == OpCode::OpEnd {
                        nested_ifs -= 1;
                    }
                    search_line += 1;
                }
                if search_line == operations.len() || operations[search_line].op_code != OpCode::OpEnd {
                    return Err(Box::new(Error { message: format(format_args!("[line: {}] No end found for else-branch", else_line)) }));
                }
                program_counter = search_line;
            }
            OpCode::OpEnd => { /* NOOP */ }
            OpCode::OpSwap => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                stack.push(op1);
                stack.push(op2);
            }
            OpCode::OpRoll => {
                let mut ops: Vec<i64> = vec![];
                for _ in 0..operation.value {
                    ops.push(stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?);
                }
                stack.push(*ops.get(0).ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?);
                for _ in 1..ops.len() {
                    stack.push(ops.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?);
                }
            }
            OpCode::OpDup => {
                let op = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                stack.push(op);
                stack.push(op);
            }
            OpCode::OpDrop => {
                stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
            }
            OpCode::OpJmp => {
                let target_address = program.labels.get(&operation.name).ok_or(Error{ message: format(format_args!("[line: {}] Unknown label: {}", program_counter, operation.name)) })?.location as i64;
                program_counter = target_address.try_into()?;
                increment_program_counter = false;
            }
            OpCode::OpRet => {
                let target_address = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                program_counter = target_address.try_into()?;
            }
            OpCode::OpCall => {
                let target_address = program.labels.get(&operation.name).ok_or(Error{ message: format(format_args!("[line: {}] Unknown label: {}", program_counter, operation.name)) })?.location as i64;
                stack.push(program_counter as i64);
                program_counter = target_address.try_into()?;
                increment_program_counter = false;
            }
            OpCode::OpStore => {
                let address: usize = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?.try_into()?;
                let value = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let truncated_value: u8 = (value & 0xFF).try_into()?;
                if address > memory.len() {
                    return Err(Box::new(Error{ message: format(format_args!("[line: {}] Memory address out of bounds: {}", program_counter, address)) }));
                }
                memory[address] = truncated_value;
            }
            OpCode::OpLoad => {
                let address: usize = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?.try_into()?;
                if address > memory.len() {
                    return Err(Box::new(Error{ message: format(format_args!("[line: {}] Memory address out of bounds: {}", program_counter, address)) }));
                }
                let value = memory[address];
                stack.push(value as i64);
            }
            OpCode::OpStore64 => {
                let address: usize = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?.try_into()?;
                let value = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                if address + 7 > memory.len() {
                    return Err(Box::new(Error{ message: format(format_args!("[line: {}] Memory address out of bounds: {}", program_counter, address)) }));
                }
                memory[address] = (value & 0xFF).try_into()?;
                memory[address+1] = (value >> 8 & 0xFF).try_into()?;
                memory[address+2] = (value >> 16 & 0xFF).try_into()?;
                memory[address+3] = (value >> 24 & 0xFF).try_into()?;
                memory[address+4] = (value >> 32 & 0xFF).try_into()?;
                memory[address+5] = (value >> 40 & 0xFF).try_into()?;
                memory[address+6] = (value >> 48 & 0xFF).try_into()?;
                memory[address+7] = (value >> 56 & 0xFF).try_into()?;
            }
            OpCode::OpLoad64 => {
                let address: usize = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?.try_into()?;
                if address + 7 > memory.len() {
                    return Err(Box::new(Error{ message: format(format_args!("[line: {}] Memory address out of bounds: {}", program_counter, address)) }));
                }
                let value: u64 = (memory[address] as u64) + ((memory[address+1] as u64) << 8) + ((memory[address+2] as u64) << 16) + ((memory[address+3] as u64) << 24) + ((memory[address+4] as u64) << 32) + ((memory[address+5] as u64) << 40) + ((memory[address+6] as u64) << 48) + ((memory[address+7] as u64) << 56);
                stack.push(value as i64);
            }
            OpCode::OpWord => {
                match operation.name.as_str() {
                    "print" => {
                        let value = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                        println!("{}", value);
                    }
                    "putc" => {
                        let char_value = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                        let character: char = char::from_u32(char_value.try_into()?).ok_or(Error{ message: format(format_args!("[line: {}] Value could not be converted to Char", program_counter)) })?;
                        print!("{}", character);
                    }
                    "exp" => {
                        let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                        let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                        let result = op2.pow(op1.try_into()?);
                        stack.push(result);
                    }
                    l if program.labels.contains_key(&l.to_string()) => {
                        if program.labels.get(l).is_none() {
                            return Err(Box::new(Error{ message: format(format_args!("[line: {}] Unknown label: {}", program_counter, l)) }));
                        }
                    }
                    _ => {
                        return Err(Box::new(Error{ message: format(format_args!("[line: {}] Unknown Call name: {:?}", program_counter, operation.name)) }));
                    }
                }
            }
            OpCode::OpPush => {
                if operation.name.len() > 0 {
                    if operation.is_constant {
                        let value = *program.constants.get(&operation.name).ok_or(Error{ message: format(format_args!("[line: {}] Unknown constant: {}", program_counter, operation.name)) })?;
                        stack.push(value);
                    } else {
                        let string_value = &operation.name;
                        for (index, byte) in string_value.bytes().enumerate() {
                            memory[string_section_pointer + index] = byte;
                        }
                        stack.push(string_value.len() as i64);
                        stack.push(string_section_pointer as i64);
                        string_section_pointer += string_value.len();
                    }
                } else {
                    stack.push(operation.value);
                }
            }
            OpCode::OpHalt => {
                println!("Halting program");
                break;
            }
            //_ => {
            //    return Err(Box::new(Error{ message: format(format_args!("[line: {}] Unknown OpCode: {:?}", program_counter, operation.op_code))}));
            //}
        }
        if increment_program_counter {
            program_counter += 1;
        }
    }
    
    if verbose {
        println!("-------");
        println!("Stack");
        for (index, value) in stack.iter().take(10).enumerate() {
            println!("  {:02}: {}", index, value);
        }
        println!("\nHeap");
        for (index, value) in memory[0..10].iter().enumerate() {
            println!("  {:02}: {}", index, value);
        }
    }
    Ok(())
}