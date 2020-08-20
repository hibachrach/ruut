const PAREN_OPEN: char = '(';
const PAREN_CLOSE: char = ')';
const COMMA: char = ',';

#[derive(Debug, PartialEq)]
pub enum Token {
    ParenOpen,
    ParenClose,
    Comma,
    Name(String),
}

pub fn tokenize(serialized: &str) -> Vec<Token> {
    let mut cur_name = String::from("");
    let mut tokens = Vec::new();

    for s in serialized.chars() {
        if let PAREN_OPEN | PAREN_CLOSE | COMMA = s {
            if cur_name.trim().is_empty() {
                // Previous name token is just whitespace
                cur_name = String::from("");
            } else if !cur_name.is_empty() {
                // This is the end of the last name token
                tokens.push(Token::Name(cur_name.trim().to_string()));
                cur_name = String::from("");
            }
        }
        match s {
            PAREN_OPEN => {
                tokens.push(Token::ParenOpen);
            }
            PAREN_CLOSE => {
                tokens.push(Token::ParenClose);
            }
            COMMA => {
                tokens.push(Token::Comma);
            }
            ch => {
                cur_name.push(ch);
            }
        }
    }
    if !cur_name.trim().is_empty() {
        tokens.push(Token::Name(cur_name));
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(tokenize(""), Vec::<Token>::new());
    }

    #[test]
    fn simple_single() {
        assert_eq!(
            tokenize("(cool beans)"),
            vec![
                Token::ParenOpen,
                Token::Name("cool beans".to_string()),
                Token::ParenClose
            ]
        );
    }

    #[test]
    fn simple_multiple() {
        assert_eq!(
            tokenize("(cool beans, better beans)"),
            vec![
                Token::ParenOpen,
                Token::Name("cool beans".to_string()),
                Token::Comma,
                Token::Name("better beans".to_string()),
                Token::ParenClose
            ]
        );
    }

    #[test]
    fn simple_newlines() {
        assert_eq!(
            tokenize("(cool beans,\nbetter beans)\n"),
            vec![
                Token::ParenOpen,
                Token::Name("cool beans".to_string()),
                Token::Comma,
                Token::Name("better beans".to_string()),
                Token::ParenClose
            ]
        );
    }

    #[test]
    fn invalid_combinations() {
        assert_eq!(
            tokenize("))  ((,(,(shit"),
            vec![
                Token::ParenClose,
                Token::ParenClose,
                Token::ParenOpen,
                Token::ParenOpen,
                Token::Comma,
                Token::ParenOpen,
                Token::Comma,
                Token::ParenOpen,
                Token::Name("shit".to_string()),
            ]
        );
    }
}
