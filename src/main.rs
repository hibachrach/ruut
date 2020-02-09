use ruut::parser::ParserError;
use ruut::prettify;
use structopt::StructOpt;
use exitcode;

#[derive(StructOpt)]
struct Cli {
    serialized_tree: String,
}

fn main() {
    let args = Cli::from_args();
    match prettify(args.serialized_tree) {
        Ok(prettified) => println!("{}", prettified),
        Err(ParserError::EmptyTokenSeqError) => {
            eprintln!("Error: empty input -- structure must be passed as the first argument");
            std::process::exit(exitcode::USAGE);
        }
        Err(ParserError::MissingNameError) => {
            eprintln!("Error: invalid input -- note that every open parenthesis must have a name before it");
            std::process::exit(exitcode::DATAERR);
        },
        Err(ParserError::MultipleRootsError) => {
            eprintln!("Error: invalid input -- must only have one root in structure");
            std::process::exit(exitcode::DATAERR);
        }
    }
}
