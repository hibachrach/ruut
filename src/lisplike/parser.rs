use super::super::{Error, Node};
use super::tokenizer::Token;
use std::vec;

pub fn parse(tokens: Vec<Token>) -> Result<Node, Error> {
    if tokens.is_empty() {
        return Err(Error::EmptyInputError);
    }
    let token_iter: &mut vec::IntoIter<Token> = &mut tokens.into_iter();
    let mut nodes = parse_iter(token_iter)?;
    if nodes.len() <= 1 {
        nodes.pop().ok_or(Error::EmptyInputError)
    } else {
        Err(Error::MultipleRootsError)
    }
}

fn parse_iter(token_iter: &mut vec::IntoIter<Token>) -> Result<Vec<Node>, Error> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut cur_node: Option<&mut Node> = None;
    while let Some(token) = token_iter.next() {
        match token {
            Token::ParenOpen => {
                if let Some(node) = &mut cur_node {
                    let mut children = parse_iter(token_iter)?;
                    node.children.append(&mut children);
                } else {
                    return Err(Error::MissingNameError);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let parser_error = parse(Vec::<Token>::new()).unwrap_err();
        assert_eq!(parser_error, Error::EmptyInputError);
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
        assert_eq!(parser_error, Error::MultipleRootsError);
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
                            children: Vec::new()
                        }]
                    },
                    Node {
                        name: "fefe".to_string(),
                        children: Vec::new()
                    }
                ]
            }
        );
    }
}
