
pub struct Token {
    pub kind: TokenKind,
}

pub struct Lit {
    pub kind: LitKind,
}

pub enum TokenKind {
    Ident,
    OpenDelim(DelimToken),
    CloseDelim(DelimToken),
    Literal(Lit),
    Semi
}

pub enum DelimToken {
    // '(' or ')'
    Paren,
    // '{' or '}'
    Brace,
    // empty delimiter
    NoDelim,
}

pub enum LitKind {
    Str,
    Err,
}
