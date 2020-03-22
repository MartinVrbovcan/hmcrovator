#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate colored;
extern crate subprocess;

use std::env;
use std::io::Read;
use std::str;
use subprocess::{Exec, Redirection};
use std::time::Duration;
use colored::*;
use std::path::PathBuf;
use std::fs;

mod arguments;
mod get_test_cases;
mod thanks;

enum TestcaseResult<'a> {
    AC,
    TLE,
    WA(&'a str, String),
}

enum TestcaseError {
    TLE,
}


fn run_process(command: &str, input: fs::File, time_limit: u64) -> Result<String, TestcaseError> {
    let mut p = Exec::cmd(command)
        .stdin(Redirection::File(input))
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .popen()
        .unwrap();

    
    if let Some(_status) = p.wait_timeout(Duration::new(time_limit, 0)).unwrap() {
        let mut output = String::new();
        p.stdout.as_ref().unwrap().read_to_string(&mut output).unwrap();
        return Ok(output);
    } else {
        p.kill().unwrap();
        p.wait().unwrap();
        return Err(TestcaseError::TLE);
    }
}

fn evaluate<'a, 'b>(command: &'a str, input: fs::File, expected: &'b str, time_limit: u64) -> TestcaseResult<'b> {
    let out = run_process(command, input, time_limit);

    match out {
        Ok(output) => {
            if output != expected {
                return TestcaseResult::WA(expected, output);
            } else {
                return TestcaseResult::AC;
            }
        },
        Err(err) => {
            match err {
                TestcaseError::TLE => return TestcaseResult::TLE,
            }
        },
    }
}

fn print_testcase(colored: bool, verbose: bool, tr: TestcaseResult) {
    if colored {
        match tr {
            TestcaseResult::AC => println!("{}", "ACCEPTED".green().bold()),
            TestcaseResult::WA(expected, out) => {
                println!("{}", "WRONG ANSWER".red().bold());
                if verbose == true {
                    println!("-------------");
                    println!("Expected:\n{}", expected);
                    println!("\n\nIncorrectly returned:\n{}", out);
                }
            }
            TestcaseResult::TLE => println!("{}", "TIME LIMIT EXCEEDED".yellow().bold()),
        }
    } else {
        match tr {
            TestcaseResult::AC => println!("{}", "ACCEPTED"),
            TestcaseResult::WA(expected, out) => {
                println!("{}", "WRONG ANSWER");
                if verbose == true {
                    println!("-------------");
                    println!("Expected:\n{}", expected);
                    println!("\n\nIncorrectly returned:\n{}", out);
                }
            }
            TestcaseResult::TLE => println!("{}", "TIME LIMIT EXCEEDED"),
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if let None = arguments::handle_misc_args(&args) {
        return;
    }

    let args = arguments::EvaluatorArgs::new(&args).unwrap();
    let action = args.action.trim_matches('"');
    
    let io_map = get_test_cases::get_test_cases(&args).unwrap();
    let mut inputs: Vec<&str> = io_map.keys().map(|s| s.as_str()).collect();
    inputs.sort();

    for input_path in inputs.iter() {
        let input_path = *input_path;
        let output_path = io_map.get(input_path).unwrap();
        let program_input = fs::File::open(input_path).expect("Error reading program input!");
        
        let expected_output = fs::read_to_string(output_path)
            .expect("Error reading expected output!");
        
        let input_path = PathBuf::from(input_path);
        let input_path = input_path.file_name().unwrap().to_str().unwrap();
        let output_path = PathBuf::from(output_path);
        let output_path = output_path.file_name().unwrap().to_str().unwrap();
        
        if args.colored {
            println!("{} - {}", input_path.bright_blue().italic(), output_path.bright_blue().italic());
        } else {
            println!("{} - {}", input_path, output_path);
        }
        
        let result = evaluate(action, program_input, &expected_output, args.time_limit);
        print_testcase(args.colored, args.verbose, result);
    }
}
