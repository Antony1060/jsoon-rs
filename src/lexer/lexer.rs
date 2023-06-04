use lazy_static::lazy_static;
use regex::Regex;
use std::io;
use std::io::ErrorKind;
use std::iter::Iterator;

#[derive(Debug)]
pub enum LexerSymbol {
    CurlyBracket(char),
    SquareBracket(char),
    Comma,
    Colon,
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NullLiteral,
}

lazy_static! {
    // TODO: not fully correct, by spec, -01234 should not be allowed, neither should 01234
    static ref NUMBER_REGEX: Regex = Regex::new(r"^-?\d+(\.\d+)?([eE][-+]?\d+)?$").unwrap();

    static ref STRING_ESCAPED_ALLOWED_CHARACTERS: Vec<char> = vec!['"', '\\', '/', 'b', 'f', 'n', 'r', 't'];
}

fn take_while<P>(content: &[char], cursor: usize, predicate: P) -> String
where
    P: FnMut(&&char) -> bool,
{
    content
        .iter()
        .skip(cursor)
        .take_while(predicate)
        .collect::<String>()
}

pub fn analyze_lexical(content: &str) -> Result<Vec<LexerSymbol>, io::Error> {
    let mut result = Vec::<LexerSymbol>::new();

    let chars = content.chars().collect::<Vec<char>>();

    let mut cursor: usize = 0;

    while cursor < content.len() {
        let char = *chars.get(cursor).expect("index");

        match char {
            '{' | '}' => result.push(LexerSymbol::CurlyBracket(char)),
            '[' | ']' => result.push(LexerSymbol::SquareBracket(char)),
            ',' => result.push(LexerSymbol::Comma),
            ':' => result.push(LexerSymbol::Colon),
            '"' => {
                cursor += 1;

                let mut value = String::from("");

                while cursor < content.len() {
                    let current_char = *chars.get(cursor).expect("index");

                    match current_char {
                        '\\' => {
                            // ignore backslash, out of match increment will handle the checked character
                            cursor += 1;

                            let escaped_option = chars.get(cursor);
                            match escaped_option {
                                Some(escaped)
                                    if STRING_ESCAPED_ALLOWED_CHARACTERS.contains(escaped) =>
                                {
                                    value.push(match escaped {
                                        '"' => '"',
                                        '\\' => '\\',
                                        '/' => '/',
                                        'b' => 0x8 as char,
                                        'f' => 0xC as char,
                                        'n' => '\n',
                                        'r' => '\r',
                                        't' => '\t',
                                        _ => {
                                            return Err(io::Error::new(
                                                ErrorKind::InvalidInput,
                                                "invalid string",
                                            ));
                                        }
                                    })
                                }
                                _ => {
                                    return Err(io::Error::new(
                                        ErrorKind::InvalidInput,
                                        "invalid string",
                                    ));
                                }
                            }
                        }
                        '"' => {
                            result.push(LexerSymbol::StringLiteral(value));
                            break;
                        }
                        current_char => value.push(current_char),
                    }
                    cursor += 1;
                }
            }
            char if char.is_ascii_alphabetic() => {
                let found = take_while(&chars, cursor, |x| x.is_ascii_alphabetic());

                cursor += found.len();

                match found.as_str() {
                    "true" => result.push(LexerSymbol::BooleanLiteral(true)),
                    "false" => result.push(LexerSymbol::BooleanLiteral(false)),
                    "null" => result.push(LexerSymbol::NullLiteral),
                    _ => {}
                }
                continue;
            }
            char if char == '-' || char.is_numeric() => {
                let found = take_while(&chars, cursor, |x| {
                    x.is_ascii_digit()
                        || **x == '-'
                        || **x == '+'
                        || **x == '.'
                        || x.to_ascii_lowercase() == 'e'
                });

                cursor += found.len();

                if NUMBER_REGEX.is_match(found.as_str()) {
                    let parsed = found
                        .parse::<f64>()
                        .map_err(|_| io::Error::new(ErrorKind::InvalidData, "invalid number"))?;

                    result.push(LexerSymbol::NumberLiteral(parsed))
                }
                continue;
            }
            _ => {}
        }
        cursor += 1;
    }

    Ok(result)
}
