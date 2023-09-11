#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Identifier(&'a str),
    KeywordIf,
    KeywordProc,
    KeywordPub,
    LiteralBoolTrue,
    OperatorBraceClose,
    OperatorBraceOpen,
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
        } else {
            let c = char_at!(i);

            match c {
                '}' => tokens.push(Token::OperatorBraceClose),
                '{' => tokens.push(Token::OperatorBraceOpen),
                ')' => tokens.push(Token::OperatorParenthesisClose),
                '(' => tokens.push(Token::OperatorParenthesisOpen),
                'i' => {
                    // If the following characters result in "if"
                    if (i + 1) < code_chars_count && char_at!(i + 1) == 'f' {
                        tokens.push(Token::KeywordIf);
                        i += 2;
                    } else {
                        if c.is_alphabetic() {
                            let identifier_begin = i;
                            i += 1;
                            'identifier_loop: while let Some(c) = code.chars().nth(i) {
                                if c.is_alphanumeric() || c == '_' {
                                    i += 1;
                                    // If there are no characters left after identifier, push the identifier
                                    if i >= code_chars_count {
                                        tokens.push(Token::Identifier(&code[identifier_begin..i]));
                                        break 'identifier_loop;
                                    }
                                } else {
                                    tokens.push(Token::Identifier(&code[identifier_begin..i]));
                                    i -= 1;
                                    break 'identifier_loop;
                                }
                            }
                        }
                    }
                }
                'p' => {
                    // If the following characters result in "proc"
                    if (i + 3) < code_chars_count
                        && char_at!(i + 1) == 'r'
                        && char_at!(i + 2) == 'o'
                        && char_at!(i + 3) == 'c'
                    {
                        tokens.push(Token::KeywordProc);
                        i += 4;
                    }
                    // If the following characters result in "pub"
                    else if (i + 2) < code_chars_count
                        && char_at!(i + 1) == 'u'
                        && char_at!(i + 2) == 'b'
                    {
                        tokens.push(Token::KeywordPub);
                        i += 3;
                    } else {
                        if c.is_alphabetic() {
                            let identifier_begin = i;
                            i += 1;
                            'identifier_loop: while let Some(c) = code.chars().nth(i) {
                                if c.is_alphanumeric() || c == '_' {
                                    i += 1;
                                    // If there are no characters left after identifier, push the identifier
                                    if i >= code_chars_count {
                                        tokens.push(Token::Identifier(&code[identifier_begin..i]));
                                        break 'identifier_loop;
                                    }
                                } else {
                                    tokens.push(Token::Identifier(&code[identifier_begin..i]));
                                    i -= 1;
                                    break 'identifier_loop;
                                }
                            }
                        }
                    }
                }
                't' => {
                    // If the following characters result in "true"
                    if (i + 3) < code_chars_count
                        && char_at!(i + 1) == 'r'
                        && char_at!(i + 2) == 'u'
                        && char_at!(i + 3) == 'e'
                    {
                        tokens.push(Token::LiteralBoolTrue);
                        i += 4;
                        continue;
                    } else {
                        if c.is_alphabetic() {
                            let identifier_begin = i;
                            i += 1;
                            'identifier_loop: while let Some(c) = code.chars().nth(i) {
                                if c.is_alphanumeric() || c == '_' {
                                    i += 1;
                                    // If there are no characters left after identifier, push the identifier
                                    if i >= code_chars_count {
                                        tokens.push(Token::Identifier(&code[identifier_begin..i]));
                                        break 'identifier_loop;
                                    }
                                } else {
                                    tokens.push(Token::Identifier(&code[identifier_begin..i]));
                                    i -= 1;
                                    break 'identifier_loop;
                                }
                            }
                        }
                    }
                }
                _ => {
                    if c.is_alphabetic() {
                        let identifier_begin = i;
                        i += 1;
                        'identifier_loop: while let Some(c) = code.chars().nth(i) {
                            if c.is_alphanumeric() || c == '_' {
                                i += 1;
                                // If there are no characters left after identifier, push the identifier
                                if i >= code_chars_count {
                                    tokens.push(Token::Identifier(&code[identifier_begin..i]));
                                    break 'identifier_loop;
                                }
                            } else {
                                tokens.push(Token::Identifier(&code[identifier_begin..i]));
                                i -= 1;
                                break 'identifier_loop;
                            }
                        }
                    }
                }
            }
            i += 1;
        }
    }
    tokens
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
