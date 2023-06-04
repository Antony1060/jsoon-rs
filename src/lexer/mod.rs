use std::io;

pub mod lexer;

#[derive(Debug)]
enum LexerSymbol {
    CurlyBracket(char),
    SquareBracket(char),
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NullLiteral,
}

fn analyze_lexical(content: &str) -> io::Result<Vec<LexerSymbol>> {
    let result = Vec::<LexerSymbol>::new();



    Ok(result)
}