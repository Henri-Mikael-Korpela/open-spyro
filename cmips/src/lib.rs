#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Identifier(&'a str),
    KeywordProc,
    KeywordPub,
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
    macro_rules! collect_identifier {
        ($index:expr) => {
            let identifier_begin = $index;
            $index += 1;
            'identifier_loop: while let Some(c) = code.chars().nth($index) {
                if c.is_alphabetic() {
                    $index += 1;
                } else {
                    tokens.push(Token::Identifier(&code[identifier_begin..$index]));
                    $index -= 1;
                    break 'identifier_loop;
                }
            }
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
                            collect_identifier!(i);
                        }
                    }
                }
                _ => {
                    if c.is_alphabetic() {
                        collect_identifier!(i);
                    }
                }
            }
            i += 1;
        }
    }
    tokens
}

#[test]
fn should_tokenize() {
    // Procedure definition without parameters.
    let code = "pub proc main(){}";
    let tokens = tokenize(code);
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
    let code = "pub proc poll(){}";
    let tokens = tokenize(code);
    assert_eq!(tokens.len(), 7);
    assert_eq!(tokens[0], Token::KeywordPub);
    assert_eq!(tokens[1], Token::KeywordProc);
    assert_eq!(tokens[2], Token::Identifier("poll"));
    assert_eq!(tokens[3], Token::OperatorParenthesisOpen);
    assert_eq!(tokens[4], Token::OperatorParenthesisClose);
    assert_eq!(tokens[5], Token::OperatorBraceOpen);
    assert_eq!(tokens[6], Token::OperatorBraceClose);
}
