use atty::Stream;
use exitcode;
use ruut::{prettify, Error, InputFormat};
use std::io::{self, BufRead};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    serialized_tree: Option<String>,
    #[structopt(
        short,
        long,
        default_value = "lisp",
        raw(possible_values = "&[\"lisp\", \"json\", \"jsonprop\"]")
    )]
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
    let serialized_tree = if let st_arg @ Some(_) = args.serialized_tree {
        st_arg
    } else if atty::isnt(Stream::Stdin) {
        if let Some(Ok(first_line_of_stdin)) = stdin.lock().lines().next() {
            Some(first_line_of_stdin)
        } else {
            None
        }
    } else {
        None
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
