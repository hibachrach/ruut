use exitcode;
use ruut::{prettify, Error, InputFormat};
use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    serialized_tree: Option<String>,
    /// Take input from stdin
    #[structopt(short = "i", long = "stdin")]
    take_from_stdin: bool,
    #[structopt(short, long, default_value = "lisp")]
    format: InputFormat,
    /// The property containing the name of the given node
    /// (only applies to `jsonprop` format)
    #[structopt(short, long, default_value = "name")]
    name: String,
    /// The property containing the children of the given node
    /// (only applies to `jsonprop` format)
    #[structopt(short, long, default_value = "children")]
    children: String,
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
        match prettify(st, args.format, args.name, args.children) {
            Ok(prettified) => println!("{}", prettified),
            Err(Error::EmptyInputError) => {
                eprintln!("Error: empty input -- structure must be passed as the first argument or via stdin");
                std::process::exit(exitcode::USAGE);
            }
            Err(Error::MissingNameError) => {
                eprintln!("Error: invalid input -- an item is missing a name");
                std::process::exit(exitcode::DATAERR);
            }
            Err(Error::MultipleRootsError) => {
                eprintln!("Error: invalid input -- must only have one root in structure");
                std::process::exit(exitcode::DATAERR);
            }
            Err(Error::FormatSpecificError(error_msg)) => {
                eprintln!("Error: invalid input -- {}", error_msg);
                std::process::exit(exitcode::DATAERR);
            }
        }
    } else {
        eprintln!("Error: no input -- structure must be passed as the first argument or via stdin");
        std::process::exit(exitcode::USAGE);
    }
}
