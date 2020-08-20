mod parser;
mod tokenizer;
use super::{Error, Node};

pub fn deserialize(serialized: String) -> Result<Node, Error> {
    let tokens = tokenizer::tokenize(&serialized);
    parser::parse(tokens)
}
