pub struct Token {
    pub kind: TokenKind,
}

pub enum TokenKind {
    /* Multi-char tokens */
    Identifier,

    /* Single-char tokens */
    Semi,       // ;
    OpenParen,  // (
    CloseParen, // )
    OpenBrace,  // {
    CloseBrace, // }
    Exclam,     // !
}
