use ruut::prettify;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    serialized_tree: String,
}

fn main() {
    let args = Cli::from_args();
    let prettified = prettify(args.serialized_tree);
    println!("{}", prettified);
}
