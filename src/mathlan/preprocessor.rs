use std::fs::read_to_string;
use std::path::Path;
use std::{fmt::format};
use super::lib::{Error};

use regex::{Captures, Regex};

pub fn load_program(file_path: String) -> Result<String, Error> {
    let parent_path = Path::new(&file_path).parent().ok_or(Error{ message: format(format_args!("No parent path found: {}", &file_path)) })?;
    let program_code = read_to_string(&file_path).map_err(|e| Error{ message: format(format_args!("Failure loading: {}: {:?}", &file_path, e)) })?;

    let re = Regex::new(r"(?m)^\s*!include\s+([a-z]+\.mln)\s*$").map_err(|e| Error{ message: format(format_args!("Failure parsing regex: {:?}", e)) })?;
    let result = re.replace_all(&program_code, |caps: &Captures| {
        // Welcher Dateiname wurde gefunden?
        let include_path = &caps[1];

        return load_program(parent_path.join(include_path).to_str().unwrap().to_string()).unwrap();
    });

    Ok(result.to_string())
}