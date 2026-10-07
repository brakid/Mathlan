use std::fmt::format;

use super::lib::Error;

#[derive(Default, Debug, PartialEq)]
pub enum OpCode {
    #[default] OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpCall,
    OpPush,
    OpSmaller,
    OpEqual,
    OpLarger,
    OpIf,
    OpElse,
    OpEnd,
    OpSwap,
    OpDup,
    OpJmp,
}

#[derive(Default, Debug)]
pub struct Operation {
    pub op_code: OpCode,
    pub value: i64, // Standard: 0
    pub name: String, // Standard: ""
}

pub fn parse(program_code: &str) -> Result<Vec<Operation>, Error> {
    let mut program: Vec<Operation> = vec![];
    let lines = program_code.trim().lines();
    for (line_index, line) in lines.into_iter().enumerate() {
        let operations = line.trim().split(" ").into_iter();
        for (index, operation) in operations.enumerate() {
            let operation = operation.trim();
            match operation {
                "+" => program.push(Operation { op_code: OpCode::OpAdd, ..Operation::default() }),
                "-" => program.push(Operation { op_code: OpCode::OpSub, ..Operation::default() }),
                "*" => program.push(Operation { op_code: OpCode::OpMul, ..Operation::default() }),
                "/" => program.push(Operation { op_code: OpCode::OpDiv, ..Operation::default() }),
                "<" => program.push(Operation { op_code: OpCode::OpSmaller, ..Operation::default() }),
                "=" => program.push(Operation { op_code: OpCode::OpEqual, ..Operation::default() }),
                ">" => program.push(Operation { op_code: OpCode::OpLarger, ..Operation::default() }),
                "if" => program.push(Operation { op_code: OpCode::OpIf, ..Operation::default() }),
                "else" => program.push(Operation { op_code: OpCode::OpElse, ..Operation::default() }),
                "end" => program.push(Operation { op_code: OpCode::OpEnd, ..Operation::default() }),
                "swap" => program.push(Operation { op_code: OpCode::OpSwap, ..Operation::default() }),
                "dup" => program.push(Operation { op_code: OpCode::OpDup, ..Operation::default() }),
                "jump" => program.push(Operation { op_code: OpCode::OpJmp, ..Operation::default() }),
                o if o.parse::<i64>().is_ok() => {
                    let value: i64 = operation.parse().map_err(|_| Error{ message: format(format_args!("[line: {}, token index: {}] Expected i64 number: {}", line_index, index, operation)) })?;
                    program.push(Operation { op_code: OpCode::OpPush, value: value, ..Operation::default() });
                }
                _ => {
                    program.push(Operation { op_code: OpCode::OpCall, name: operation.to_string(), ..Operation::default() });
                },
            }
        }
    }

    Ok(program)
}

pub fn execute(program: &[Operation]) -> Result<(), Box<dyn std::error::Error>> {
    let mut stack: Vec<i64> = vec![];

    let mut program_counter: usize = 0;
    while program_counter < program.len() {
        let operation = &program[program_counter];
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
                    while program_counter < program.len() && program[program_counter].op_code != OpCode::OpElse {
                        //println!("Skipping: {} {:?}", index, program[index]);
                        program_counter += 1;
                    }
                    if program[program_counter].op_code != OpCode::OpElse {
                        return Err(Box::new(Error { message: format(format_args!("[line: {}] No else branch found for if", if_line)) }));
                    }
                }
            }
            OpCode::OpElse => {
                let else_line = program_counter;
                while program_counter < program.len() && program[program_counter].op_code != OpCode::OpEnd {
                    //println!("Skipping: {} {:?}", index, program[index]);
                    program_counter += 1;
                }
                if program[program_counter].op_code != OpCode::OpEnd {
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
            OpCode::OpDup => {
                let op = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                stack.push(op);
                stack.push(op);
            }
            OpCode::OpJmp => {
                let target_diff = stack.pop().ok_or(Error{ message: format(format_args!("[line: {}] No value on stack", program_counter)) })?;
                let target = program_counter as i64 + target_diff;
                program_counter = target.try_into()?;
                program_counter -= 1;
            }
            OpCode::OpCall => {
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
                    _ => {
                        return Err(Box::new(Error{ message: format(format_args!("[line: {}] Unknown Call name: {:?}", program_counter, operation.name)) }));
                    }
                }
            }
            OpCode::OpPush => {
                stack.push(operation.value);
            }
            //_ => {
            //    return Err(Box::new(Error{ message: format(format_args!("[line: {}] Unknown OpCode: {:?}", program_counter, operation.op_code))}));
            //}
        }
        program_counter += 1;
    }

    Ok(())
}