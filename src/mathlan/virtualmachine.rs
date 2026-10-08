use std::{fmt::format};

use super::lib::{Error, OpCode, Program};

const MEMORY_SIZE: usize = 32000;

pub fn execute(program: &Program, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    let operations = &program.operations;
    let mut stack: Vec<i64> = vec![];
    let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

    let mut program_counter: usize = *program.labels.get("main").ok_or(Error{ message: "Missing main label".to_string() })?;
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
            OpCode::OpIf => {
                let op = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                if op != 1 {
                    let if_line = program_counter;
                    while program_counter < operations.len() && operations[program_counter].op_code != OpCode::OpElse {
                        //println!("Skipping: {} {:?}", index, program[index]);
                        program_counter += 1;
                    }
                    if operations[program_counter].op_code != OpCode::OpElse {
                        return Err(Box::new(Error { message: format(format_args!("[line: {}] No else branch found for if", if_line)) }));
                    }
                }
            }
            OpCode::OpElse => {
                let else_line = program_counter;
                while program_counter < operations.len() && operations[program_counter].op_code != OpCode::OpEnd {
                    //println!("Skipping: {} {:?}", index, program[index]);
                    program_counter += 1;
                }
                if operations[program_counter].op_code != OpCode::OpEnd {
                    return Err(Box::new(Error { message: format(format_args!("[line: {}] No end found for if-else", else_line)) }));
                }
            }
            OpCode::OpEnd => {
                //return Err(Error { message: format(format_args!("[line: {}] Should not end here", index)) });
            }
            OpCode::OpSwap => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                stack.push(op1);
                stack.push(op2);
            }
            OpCode::OpRot => {
                let op1 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op2 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let op3 = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                stack.push(op1);
                stack.push(op3);
                stack.push(op2);
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
                let target_address = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                program_counter = target_address.try_into()?;
                increment_program_counter = false;
            }
            OpCode::OpRet => {
                let target_address = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                program_counter = target_address.try_into()?;
            }
            OpCode::OpCall => {
                let target_address = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
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
                        let target_address = *program.labels.get(l).ok_or(Error{ message: format(format_args!("[line: {}] Unknown label: {}", program_counter, l)) })? as i64;
                        stack.push(target_address);
                    }
                    _ => {
                        return Err(Box::new(Error{ message: format(format_args!("[line: {}] Unknown Call name: {:?}", program_counter, operation.name)) }));
                    }
                }
            }
            OpCode::OpPush => {
                stack.push(operation.value);
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