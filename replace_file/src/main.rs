use std::fs;
use regex::Regex;

fn main() {
    let args = parse_args();
    
    // read data from file
    let data = match fs::read_to_string(&args.filename) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            std::process::exit(1);
        }
    };
    
    // replace
    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error replacing data: {}", e);
            std::process::exit(1);
        }
    };

    // write data to file
    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {
            println!("Successfully wrote to file {}", args.output);
        },
        Err(err) => {
            eprintln!("Error writing to file: {}", err);
            std::process::exit(1);
        }
    };
}

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn parse_args() -> Arguments {
    let args: Vec<String> = std::env::args().skip(1).collect();

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let reg = Regex::new(target)?;
    Ok(reg.replace_all(text, replacement).to_string())
}