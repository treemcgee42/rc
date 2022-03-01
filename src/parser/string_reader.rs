use super::token::Token;

struct StringReader<'a> {
    // Source, to tokenize
    src: &'a str,
}

impl<'a> StringReader<'a> {
    fn next_token(&mut self) -> (Token)
}
