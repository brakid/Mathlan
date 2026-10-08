mod mathlan;

use std::env;

use mathlan::virtualmachine::execute;
use mathlan::parser::parse;
use mathlan::preprocessor::load_program;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Program: {} [v] <program file>", args[0]);
        std::process::exit(1);
    }

    let mut verbose = false;
    let path: String;

    match args[1].as_str() {
        "v" => { verbose = true; path = args[2].clone(); },
        _ => { path = args[1].clone(); },
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

    execute(&program, verbose)?;

    Ok(())
}
