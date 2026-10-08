mod mathlan;

use std::fs::write;
use std::env;

use mathlan::compiler::compile;
use mathlan::virtualmachine::execute;
use mathlan::parser::parse;
use mathlan::preprocessor::load_program;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        eprintln!("Program: {} run|compile [v] <program file>", args[0]);
        std::process::exit(1);
    }

    let mut verbose = false;
    let path: String;

    let is_run_mode: bool;
    match args[1].as_str() {
        "run"|"r" => is_run_mode = true,
        "compile"|"c" => is_run_mode = false,
        _ => {
            eprintln!("Program: {} run|compile [v] <program file>", args[0]);
            std::process::exit(1);
        },
    }

    match args[2].as_str() {
        "v" => { verbose = true; path = args[3].clone(); },
        _ => { path = args[2].clone(); },
    }

    let program_code = load_program(path)?;
    if verbose {
        println!("Program:");
        println!("{}", program_code);
        println!("-------");
    }
    let program = parse(&program_code)?;
    
    if verbose {
        println!("Parsed program:");
        for (index, operation) in program.operations.iter().enumerate() {
            println!("  {:02}: {:?}", index, operation);
        }
        println!("\nLabels:");
        println!("  {:?}", program.labels);
        println!("-------");
    }

    if is_run_mode {
        execute(&program, verbose)?;
    } else {
        let c_code = compile(&program, verbose)?;
        write("./compiled/main.c", c_code)?;
    }

    Ok(())
}
