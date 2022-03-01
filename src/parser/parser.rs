
pub struct Parser {}

impl Parser {
    /// Advance the parser by one token
    pub fn bump(&mut self) {}
    /// Consume the next token if it matches the provided token kind
    /// (`eat()` in `rustc`)
    pub fn eat_if_tok(&mut self, tok: &TokenKind) {}
    /// Expects the next token to be the `tok`, and consumes it.
    /// 
    /// Unlike `eat_if_tok()` which does nothing if the next token doesn't match `tok`, this 
    /// will throw an error. 
    pub fn eat_tok(&mut self, tok: &TokenKind) -> Result<bool> {}
    /// Check if the next token is the one specified
    /// (`check()` in `rustc`)
    fn check_next_tok(&mut self, tok: &TokenKind) -> bool {}
    /// Look ahead `n` tokens from the current token (`self.token`). By "looking" we 
    /// mean that you can provide a function `looker` that returns something about a 
    /// token, for example one of its struct fields. 
    fn look_nth_tok<R>(&self, n: usize, looker: impl FnOnce(&Token) -> R) -> R {}
}
