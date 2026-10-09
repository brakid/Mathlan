use std::{collections::HashMap, fmt::format};

use super::lib::{Error, LabelType::{Call, Jump}, OpCode, Program};

pub fn compile(program: &Program, verbose: bool) -> Result<String, Box<dyn std::error::Error>> {
    let operations = &program.operations;
    let mut c_code = String::new();
    c_code += "#include <stdio.h>\n";
    c_code += "#include <stdint.h>\n";
    c_code += "#include <stdlib.h>\n";
    c_code += "#include <math.h>\n";
    c_code += "#define STACK_SIZE 16000\n";
    c_code += "#define MEMORY_SIZE 16000\n";

    c_code += "int64_t stack[STACK_SIZE];\n";
    c_code += "size_t stackPointer = 0;\n";
    c_code += "uint8_t memory[MEMORY_SIZE];\n";

    let mut jump_labels_by_index: HashMap<i64, String> = HashMap::new();
    for (_, (label_name, label)) in program.labels.iter().enumerate() {
        if label.label_type == Jump {
            jump_labels_by_index.insert(label.location as i64, label_name.to_string());
        }
    }

    for (label_name, label) in program.labels.iter() {
        if label.label_type == Call {
            c_code += format(format_args!("void {}(void);\n", label_name)).as_str();
        }
    }

    for (constant_name, value) in program.constants.iter() {
        c_code += format(format_args!("int64_t {} = {};\n", constant_name, value)).as_str();
    }

    let mut var_name_counter: i64 = 0;

    for (_, (label_name, label)) in program.labels.iter().enumerate() {
        if verbose { 
            println!("Processing: {}: {:?}", label_name, label); 
        }
        if label_name == "main" || label.label_type == Call {
            if label_name == "main" {
                if verbose { 
                    println!("Processing main");
                }
                c_code += format(format_args!("int {}() {{\n", label_name)).as_str();
            } else {        
                c_code += format(format_args!("void {}() {{\n", label_name)).as_str();
            }
            let mut operation_index = label.location;
            let mut operation = &operations[operation_index];
            while operation_index < operations.len() && operation.op_code != OpCode::OpRet {
                if verbose { 
                    println!("Processing: {}: {:?}", operation_index, operation);
                }
                operation = &operations[operation_index];
                if jump_labels_by_index.contains_key(&(operation_index as i64)) {
                    let name = jump_labels_by_index.get(&(operation_index as i64)).unwrap();
                    c_code += format(format_args!("{}:;\n", name)).as_str();
                }
                match operation.op_code {
                    OpCode::OpAdd => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{} + var{};\n", counter2, counter1)).as_str();
                        var_name_counter += 2;
                    }
                    OpCode::OpSub => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{} - var{};\n", counter2, counter1)).as_str();
                        var_name_counter += 2;
                    }
                    OpCode::OpMul => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{} * var{};\n", counter2, counter1)).as_str();
                        var_name_counter += 2;
                    }
                    OpCode::OpDiv => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{} / var{};\n", counter2, counter1)).as_str();
                        var_name_counter += 2;
                    }
                    OpCode::OpMod => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{} % var{};\n", counter2, counter1)).as_str();
                        var_name_counter += 2;
                    }
                    OpCode::OpSmaller => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{} < var{} ? 1 : 0;\n", counter2, counter1)).as_str();
                        var_name_counter += 2;
                    }
                    OpCode::OpEqual => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{} == var{} ? 1 : 0;\n", counter2, counter1)).as_str();
                        var_name_counter += 2;
                    }
                    OpCode::OpLarger => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{} > var{} ? 1 : 0;\n", counter2, counter1)).as_str();
                        var_name_counter += 2;
                    }
                    OpCode::OpIf => {
                         c_code += "if (stack[--stackPointer]) {\n";
                    }
                    OpCode::OpElse => {
                         c_code += "} else {\n";
                    }
                    OpCode::OpEnd => {
                         c_code += "}\n";
                    }
                    OpCode::OpSwap => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{};\n", counter1)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{};\n", counter2)).as_str();
                        var_name_counter += 2;
                    }
                    OpCode::OpRot => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        let counter3 = var_name_counter + 2;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter3)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{};\n", counter1)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{};\n", counter3)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{};\n", counter2)).as_str();
                        var_name_counter += 3;
                    }
                    OpCode::OpDup => {
                        let counter1 = var_name_counter;
                        c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{};\n", counter1)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = var{};\n", counter1)).as_str();
                        var_name_counter += 1;
                    }
                    OpCode::OpDrop => {
                        c_code += "--stackPointer;\n";
                    }
                    OpCode::OpJmp => {
                        let previous_operation = &operations[operation_index - 1];
                        if previous_operation.op_code != OpCode::OpWord {
                            return Err(Box::new(Error{ message: format(format_args!("[line: {}] Expected Label/Word before: {:?}", operation_index, previous_operation)) }));
                        }
                        c_code += format(format_args!("goto {};\n", previous_operation.name)).as_str();
                    }
                    OpCode::OpRet => {
                        c_code += "stackPointer--;\n"; // ignore return address
                        c_code += "return; }\n";
                        break;
                    }
                    OpCode::OpCall => {
                        let previous_operation = &operations[operation_index - 1];
                        if previous_operation.op_code != OpCode::OpWord {
                            return Err(Box::new(Error{ message: format(format_args!("[line: {}] Expected Label/Word before: {:?}", operation_index, previous_operation)) }));
                        }
                        c_code += "stack[stackPointer++] = 0xDEAD;\n";
                        c_code += format(format_args!("{}();\n", previous_operation.name)).as_str();
                    }
                    OpCode::OpStore => {
                        let counter1 = var_name_counter;
                        let counter2 = var_name_counter + 1;
                        c_code += format(format_args!("int64_t address{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("uint8_t value{} = stack[--stackPointer] & 0xFF;\n", counter2)).as_str();
                        c_code += format(format_args!("memory[address{}] = value{};\n", counter1, counter2)).as_str();
                        var_name_counter += 2;
                    }
                    OpCode::OpLoad => {
                        let counter1 = var_name_counter;
                        c_code += format(format_args!("size_t address{} = stack[--stackPointer];\n", counter1)).as_str();
                        c_code += format(format_args!("stack[stackPointer++] = memory[address{}];\n", counter1)).as_str();
                        var_name_counter += 1;
                    }
                    OpCode::OpPush => {
                        if operation.name.len() > 0 {
                            c_code += format(format_args!("stack[stackPointer++] = {};\n", operation.name)).as_str();
                        } else {
                            c_code += format(format_args!("stack[stackPointer++] = {};\n", operation.value)).as_str();
                        }
                    }
                    OpCode::OpHalt => {
                        c_code += "exit(0);\n"
                    }
                    OpCode::OpWord => {
                        match operation.name.as_str() {
                            "print" => {
                                let counter1 = var_name_counter;
                                c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                                c_code += format(format_args!("printf(\"%lld\\n\", var{});\n", counter1)).as_str();
                                var_name_counter += 1;
                            }
                            "putc" => {
                                let counter1 = var_name_counter;
                                c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                                c_code += format(format_args!("printf(\"%c\", (char)var{});\n", counter1)).as_str();
                                var_name_counter += 1;
                            }
                            "exp" => {
                                let counter1 = var_name_counter;
                                let counter2 = var_name_counter + 1;
                                c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter1)).as_str();
                                c_code += format(format_args!("int64_t var{} = stack[--stackPointer];\n", counter2)).as_str();
                                c_code += format(format_args!("stack[stackPointer++] = (int64_t)pow(var{}, var{});\n", counter2, counter1)).as_str();
                                var_name_counter += 2;
                            }
                            l if program.labels.contains_key(&l.to_string()) => { /* NOOP */ }
                            _ => {
                                return Err(Box::new(Error{ message: format(format_args!("[line: {}] Unknown Word name: {:?}", operation_index, operation.name)) }));
                            }
                        }
                    }
                }

                operation_index += 1;
            }
        }
        if label_name == "main" {
            c_code += "return 0; }\n";
        }
    }

    Ok(c_code)
}