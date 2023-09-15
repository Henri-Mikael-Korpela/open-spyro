#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Assignment,
    Identifier(&'a str),
    KeywordIf,
    KeywordProc,
    KeywordPub,
    LiteralBoolFalse,
    LiteralBoolTrue,
    OperatorBraceClose,
    OperatorBraceOpen,
    OperatorEquality,
    OperatorParenthesisClose,
    OperatorParenthesisOpen,
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let code_chars_count = code.chars().count();

    let mut i = 0;
    let mut tokens = Vec::new();

    macro_rules! char_at {
        ($index:expr) => {
            code.chars().nth($index).unwrap()
        };
    }

    loop {
        if i >= code_chars_count {
            break;
        }

        let c = char_at!(i);

        match c {
            '}' => {
                tokens.push(Token::OperatorBraceClose);
                i += 1;
            }
            '{' => {
                tokens.push(Token::OperatorBraceOpen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::OperatorParenthesisClose);
                i += 1;
            }
            '(' => {
                tokens.push(Token::OperatorParenthesisOpen);
                i += 1;
            }
            '=' => {
                // If the following character results in "=="
                if (i + 1) < code_chars_count && char_at!(i + 1) == '=' {
                    tokens.push(Token::OperatorEquality);
                    i += 2;
                } else {
                    tokens.push(Token::Assignment);
                    i += 1;
                }
            }
            _ => {
                // If the character is a letter
                if c.is_alphabetic() {
                    let mut identifier_len = 0;
                    while let Some(c) = code.chars().nth(i + identifier_len) {
                        if c.is_alphanumeric() || c == '_' {
                            identifier_len += 1;
                        } else {
                            break;
                        }
                    }

                    if identifier_len > 0 {
                        let identifier = &code[i..i + identifier_len];
                        match identifier {
                            "false" => tokens.push(Token::LiteralBoolFalse),
                            "if" => tokens.push(Token::KeywordIf),
                            "proc" => tokens.push(Token::KeywordProc),
                            "pub" => tokens.push(Token::KeywordPub),
                            "true" => tokens.push(Token::LiteralBoolTrue),
                            _ => tokens.push(Token::Identifier(identifier)),
                        }
                        i += identifier_len;
                        continue;
                    }
                }
                i += 1;
            }
        }
    }
    tokens
}

#[cfg(test)]
mod tokenization_test {
    use super::*;

    #[test]
    fn should_tokenize_boolean_literals() {
        // Boolean literal false
        let tokens = tokenize("false");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::LiteralBoolFalse);

        // Boolean literal true
        let tokens = tokenize("true");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::LiteralBoolTrue);

        let tokens = tokenize("proc main(){false}");
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0], Token::KeywordProc);
        assert_eq!(tokens[1], Token::Identifier("main"));
        assert_eq!(tokens[2], Token::OperatorParenthesisOpen);
        assert_eq!(tokens[3], Token::OperatorParenthesisClose);
        assert_eq!(tokens[4], Token::OperatorBraceOpen);
        assert_eq!(tokens[5], Token::LiteralBoolFalse);
        assert_eq!(tokens[6], Token::OperatorBraceClose);
    }
    #[test]
    fn should_tokenize_equality_operator() {
        // Equality operator
        let tokens = tokenize("==");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::OperatorEquality);

        // Equality operator with identifier
        let tokens = tokenize("a==b");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Identifier("a"));
        assert_eq!(tokens[1], Token::OperatorEquality);
        assert_eq!(tokens[2], Token::Identifier("b"));

        // Equality operator with identifier and whitespaces
        let tokens = tokenize("a == b");
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Identifier("a"));
        assert_eq!(tokens[1], Token::OperatorEquality);
        assert_eq!(tokens[2], Token::Identifier("b"));
    }
    #[test]
    fn should_tokenize_identifier() {
        // Identifier with only alphabetic characters
        let tokens = tokenize("main");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Identifier("main"));

        // Identifier with underscore
        let tokens = tokenize("main_loop");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Identifier("main_loop"));

        // Identifier with underscore and number
        let tokens = tokenize("main_loop_1");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Identifier("main_loop_1"));
    }
    #[test]
    fn should_tokenize_public_procedure_definition() {
        // Procedure definition without parameters.
        let tokens = tokenize("pub proc main(){}");
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0], Token::KeywordPub);
        assert_eq!(tokens[1], Token::KeywordProc);
        assert_eq!(tokens[2], Token::Identifier("main"));
        assert_eq!(tokens[3], Token::OperatorParenthesisOpen);
        assert_eq!(tokens[4], Token::OperatorParenthesisClose);
        assert_eq!(tokens[5], Token::OperatorBraceOpen);
        assert_eq!(tokens[6], Token::OperatorBraceClose);

        // Procedure definition with parameters with a procedure name that begins with 'p'
        // that should not mess up with the keywords starting with 'p'.
        let tokens = tokenize("pub proc poll(){}");
        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0], Token::KeywordPub);
        assert_eq!(tokens[1], Token::KeywordProc);
        assert_eq!(tokens[2], Token::Identifier("poll"));
        assert_eq!(tokens[3], Token::OperatorParenthesisOpen);
        assert_eq!(tokens[4], Token::OperatorParenthesisClose);
        assert_eq!(tokens[5], Token::OperatorBraceOpen);
        assert_eq!(tokens[6], Token::OperatorBraceClose);

        // Procedure definition with body containing an if statement.
        let tokens = tokenize("pub proc main(){if true{}}");
        assert_eq!(tokens.len(), 11);
        assert_eq!(tokens[0], Token::KeywordPub);
        assert_eq!(tokens[1], Token::KeywordProc);
        assert_eq!(tokens[2], Token::Identifier("main"));
        assert_eq!(tokens[3], Token::OperatorParenthesisOpen);
        assert_eq!(tokens[4], Token::OperatorParenthesisClose);
        assert_eq!(tokens[5], Token::OperatorBraceOpen);
        assert_eq!(tokens[6], Token::KeywordIf);
        assert_eq!(tokens[7], Token::LiteralBoolTrue);
        assert_eq!(tokens[8], Token::OperatorBraceOpen);
        assert_eq!(tokens[9], Token::OperatorBraceClose);
        assert_eq!(tokens[10], Token::OperatorBraceClose);
    }
}
