mod mathlan;

use std::{env, fs::read_to_string};

use mathlan::virtualmachine::execute;
use mathlan::parser::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    let program = parse(r#"
        35 34 + print
        72 2 / 2 * print
        2 2 exp print
        1 2 < print
        1 2 = print
        1 2 > print
        1 1 = if 100 print else 200 print end
        1 2 = if 101 print else 202 print end
        1 dup print 2 + print
        1 2 - print 1 2 swap - print
    "#)?;*/

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        eprintln!("Program: {} [v] <program file>", args[0]);
        std::process::exit(1);
    }

    let mut verbose = false;
    let path: &String;

    match args[1].as_str() {
        "v" => { verbose = true; path = &args[2]; },
        _ => { path = &args[1]; },
    }

    let program_code = read_to_string(path)?;
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
