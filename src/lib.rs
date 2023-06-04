use std::collections::HashMap;

mod lexer;
mod parser;

pub fn parse_json(json_content: &str) -> HashMap<String, String> {
    HashMap::<String, String>::new()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//     }
// }
