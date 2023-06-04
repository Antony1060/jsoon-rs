mod lexer;

use crate::lexer::lexer::{analyze_lexical, LexerSymbol};
use std::io;

pub fn parse_json(json_content: &str) -> io::Result<Vec<LexerSymbol>> {
    analyze_lexical(json_content)
}

fn main() {
    println!(
        "{parsed:?}",
        parsed = parse_json(
            r##"
        {
            "foo": "bar",
            "bar": 2,
            "baz": +2.4e-6,
            "foo2": true,
            "bar2": false,
            "fooArr": [1, "hi", true, { "hi": "hello" }],
            "fooObj": { "hi": "hello" }
        }
    "##
        )
    )
}
