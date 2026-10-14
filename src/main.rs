mod mathlan;

use std::fs::write;
use std::env;
use std::path::Path;

use mathlan::lib::Error;
use mathlan::compiler::compile;
use mathlan::virtualmachine::execute;
use mathlan::parser::parse;
use mathlan::preprocessor::load_program;
use subprocess::Exec;

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

    let program_code = load_program(&path)?;
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
        println!("Constants:");
        println!("  {:?}", program.constants);
        println!("-------");
    }

    if is_run_mode {
        execute(&program, verbose)?;
    } else {
        let c_code = compile(&program, verbose)?;
        let file = Path::new(&path).file_name().unwrap().to_str().unwrap().split(".").next().unwrap();
        let output_directory = "./compiled/";
        let code_path = Path::new(output_directory).join(file.to_string() + ".c");
        let code_path = code_path.to_str().unwrap();
        let executable_path = Path::new(output_directory).join(file.to_string());
        let executable_path = executable_path.to_str().unwrap();
        println!("Written output to: {}", code_path);
        write(code_path, c_code)?;
        let status = Exec::cmd("gcc").arg("-Wall").arg("-Wextra").arg("-O3").arg("-o").arg(executable_path).arg(code_path).join()?;
        if status.success() {
            println!("Compiled to: {}", executable_path);
        } else {
            println!("Compilation of {} failed", executable_path);
            return Err(Box::new(Error{ message: "Compilation failed".to_string() }));
        }
    }

    Ok(())
}
