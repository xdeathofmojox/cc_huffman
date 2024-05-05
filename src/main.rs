use std::env;
use std::process::ExitCode;
use std::fs::File;
use std::io::{self, Error, BufReader, BufRead};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let filenames = parse_args(args);
    let mut status = 0;

    if filenames.is_empty() {
        let stdin = io::stdin();
        let result = handle_compress(&mut stdin.lock());
        if result.is_err() {
            println!("Invalid: stdin - {:?}", result.err().unwrap().to_string());
            status = 1;
        } else {
            println!("Valid: stdin");
        }
    }

    for filename in filenames {
        let result = handle_file(filename.as_str());
        if result.is_err() {
            println!("Invalid: {:?} - {:?}", filename, result.err().unwrap().to_string());
            status = 1;
        } else {
            println!("Valid: {:?}", filename);
        }
    }

    ExitCode::from(status)
}

fn parse_args(args: Vec<String>) -> Vec<String> {
    let mut file_result: Vec<String> = vec![];
    for arg in args {
        file_result.push(arg);
    }

    file_result
}

fn handle_file(filename: &str) -> Result<String, Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    handle_compress(&mut reader)
}

fn handle_compress<R: BufRead>(reader: &mut R) -> Result<String, Error> {
    let mut s = String::new();
    reader.read_to_string(&mut s)?;
    Ok(s)
}