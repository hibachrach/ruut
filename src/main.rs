use exitcode;
use ruut::parser::ParserError;
use ruut::prettify;
use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    serialized_tree: Option<String>,
    /// Take input from stdin
    #[structopt(short = "i", long = "stdin")]
    take_from_stdin: bool,
}

fn main() {
    let args = Cli::from_args();
    let stdin = io::stdin();
    let serialized_tree = match args.serialized_tree {
        Some(st) => Some(st),
        None => {
            if args.take_from_stdin {
                match stdin.lock().lines().next() {
                    Some(Ok(first_line_of_stdin)) => Some(first_line_of_stdin),
                    _ => None,
                }
            } else {
                None
            }
        }
    };
    if let Some(st) = serialized_tree {
        match prettify(st) {
            Ok(prettified) => println!("{}", prettified),
            Err(ParserError::EmptyTokenSeqError) => {
                eprintln!("Error: empty input -- structure must be passed as the first argument or via stdin");
                std::process::exit(exitcode::USAGE);
            }
            Err(ParserError::MissingNameError) => {
                eprintln!("Error: invalid input -- note that every open parenthesis must have a name before it");
                std::process::exit(exitcode::DATAERR);
            }
            Err(ParserError::MultipleRootsError) => {
                eprintln!("Error: invalid input -- must only have one root in structure");
                std::process::exit(exitcode::DATAERR);
            }
        }
    } else {
        eprintln!("Error: no input -- structure must be passed as the first argument or via stdin");
        std::process::exit(exitcode::USAGE);
    }
}
