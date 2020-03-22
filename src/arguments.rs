use super::thanks;
use std::process;

const HELP_MSG: &'static str = "
A code evaluator written in rust.
Usage: hmcrovator.exe [command to run program] [...options]

* The \"command to run program\" is quoted as to be able to span across multiple spaces
  (see example below)

Example use: ./hmcrovator.exe \"python ./myfile.py\" -iod \"./test_cases/myfile\" -t 5 -c

Options:
    -t The time limit for each test case (a whole number, in seconds, the default is 1)
    -i Test case input (if using on 1 test case)
    -o Test case correct output (if using on 1 test case)
    -iod Input/Ouput directory (overwrites -i and -o) - a directory containing the program 
        (test case) inputs and outputs, in the format (something).in.[identifier],
        and (something).out.[identifier].
        (NOTE: also works if the .in and .out extensions are at the end of the filename)
        The corresponding inputs and outputs are matched according to the identifier.
        The test cases are processed alphabetically according to the identifiers.
    -v Verbose output - output the expected output and output from the program in the case of a wrong
       answer.
    -c Colored output - highlight output with colors and styling (may not display properly on all consoles)

* A little credit for the project's dependencies is at: \"hmcrovator.exe thanks\"
";

fn expect_on_outofbounds(ind: usize, length: usize, msg: &'static str) {
    if ind >= length {
        println!("{}", msg);
        process::exit(1);
    }
}
pub struct EvaluatorArgs<'a> {
    pub action: &'a str,
    pub time_limit: u64,
    pub input_file_path: &'a str,
    pub correct_output_file_path: &'a str,
    pub io_directory: &'a str,
    pub verbose: bool,
    pub colored: bool,
}

impl<'a> EvaluatorArgs<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<EvaluatorArgs, &'static str> {
        expect_on_outofbounds(1, args.len(), "Expected action. Try --help for example usage.");
        let action = &args[1];
        let mut index = 0;
        let mut time_limit = 1;
        let mut input_file_path = "";
        let mut correct_output_file_path = "";
        let mut io_directory = "";
        let mut verbose = false;
        let mut colored = false;

        while index < args.len() {
            if args[index] == "-i" {
                expect_on_outofbounds(index + 1, args.len(), "No test case input file specified after -i argument!");
                input_file_path = &args[index + 1];
            } else if args[index] == "-o" {
                expect_on_outofbounds(index + 1, args.len(), "No test case correct output file specified after -o argument!");
                correct_output_file_path = &args[index + 1];
            } else if args[index] == "-t" {
                expect_on_outofbounds(index + 1, args.len(), "No time limit specified after -t argument!");
                time_limit = args[index + 1].parse().expect("The time limit is not a valid whole number!");
            } else if args[index] == "-iod" {
                expect_on_outofbounds(index + 1, args.len(), "No test case input output directory specified after the -iod argument!");
                io_directory = &args[index + 1];
            } else if args[index] == "-v" {
                verbose = true;
            } else if args[index] == "-c" {
                colored = true;
            }
            index += 1;
        }
        
        if io_directory.len() < 1 && (input_file_path.len() < 1 || correct_output_file_path.len() < 1) {
            return Err("No test case specified (hint: use either the -iod, or the -i and -o arguments)!");
        } else {
            return Ok(EvaluatorArgs{
                action, time_limit,
                input_file_path,
                correct_output_file_path,
                io_directory,
                verbose,
                colored,
            });
        }
    }
}

pub fn handle_misc_args(args: &Vec<String>) -> Option<()> {
    expect_on_outofbounds(1, args.len(), "Expected action. Try --help for example usage.");
    let action = &args[1];
    let mut is_help: bool = action == "help";

    if !is_help {
        for arg in args.iter() {
            if arg == "-h" || arg == "--help" {
                is_help = true;
            }
        }
    }

    if is_help {
        println!("{}", HELP_MSG);
        return None;
    } else if action == "thanks" {
        println!("{}", thanks::THANKS_MSG);
        return None;
    }
    return Some(());
}
