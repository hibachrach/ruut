use super::tokenizer::Token;
use super::Node;
use std::fmt;
use std::vec;

pub fn parse(tokens: Vec<Token>) -> Result<Node, ParserError> {
    if tokens.is_empty() {
        return Err(ParserError::EmptyTokenSeqError);
    }
    let token_iter: &mut vec::IntoIter<Token> = &mut tokens.into_iter();
    let mut nodes = parse_iter(token_iter)?;
    if nodes.len() <= 1 {
        nodes.pop().ok_or(ParserError::EmptyTokenSeqError)
    } else {
        Err(ParserError::MultipleRootsError)
    }
}

fn parse_iter(token_iter: &mut vec::IntoIter<Token>) -> Result<Vec<Node>, ParserError> {
    let mut nodes: Vec<Node> = vec![];
    let mut cur_node: Option<&mut Node> = None;
    while let Some(token) = token_iter.next() {
        match token {
            Token::ParenOpen => {
                if let Some(node) = &mut cur_node {
                    let mut children = parse_iter(token_iter)?;
                    node.children.append(&mut children);
                } else {
                    return Err(ParserError::InvalidSeqError);
                }
            }
            Token::ParenClose => {
                break;
            }
            Token::Name(name) => {
                let node = Node::new(name);
                nodes.push(node);
                cur_node = nodes.last_mut();
            }
            Token::Comma => {}
        }
    }
    Ok(nodes)
}

#[derive(Debug, PartialEq)]
pub enum ParserError {
    EmptyTokenSeqError,
    InvalidSeqError,
    MultipleRootsError,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Why can't I do `use self::*`??
        use ParserError::*;

        match self {
            &EmptyTokenSeqError => write!(f, "token sequence cannot be empty"),
            &InvalidSeqError => write!(f, "token sequence must be valid"),
            &MultipleRootsError => write!(f, "must have one root node"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let parser_error = parse(Vec::<Token>::new()).unwrap_err();
        assert_eq!(parser_error, ParserError::EmptyTokenSeqError);
    }

    #[test]
    fn multiple_root_sequence() {
        let multiple_root_sequence = vec![
            Token::Name("papa".to_string()),
            Token::ParenOpen,
            Token::Name("bebe".to_string()),
            Token::ParenClose,
            Token::Name("popo".to_string()),
            Token::ParenOpen,
            Token::Name("bubu".to_string()),
            Token::ParenClose,
        ];
        let parser_error = parse(multiple_root_sequence).unwrap_err();
        assert_eq!(parser_error, ParserError::MultipleRootsError);
    }

    #[test]
    fn good_token_sequence() {
        let good_sequence = vec![
            Token::Name("papa".to_string()),
            Token::ParenOpen,
            Token::Name("bebe".to_string()),
            Token::ParenOpen,
            Token::Name("gege".to_string()),
            Token::ParenClose,
            Token::Comma,
            Token::Name("fefe".to_string()),
            Token::ParenClose,
        ];
        let root_node = parse(good_sequence).unwrap();
        assert_eq!(
            root_node,
            Node {
                name: "papa".to_string(),
                children: vec![
                    Node {
                        name: "bebe".to_string(),
                        children: vec![Node {
                            name: "gege".to_string(),
                            children: vec![]
                        }]
                    },
                    Node {
                        name: "fefe".to_string(),
                        children: vec![]
                    }
                ]
            }
        );
    }
}
