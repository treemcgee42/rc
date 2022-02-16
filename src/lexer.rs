use self::LiteralKind::*;
use self::TokenKind::*;
use crate::cursor::Cursor;

/* ========================= */
/* ===== Key functions ===== */
/* ========================= */

/// Takes an input string and returns a stream of tokens, represented as an
/// iterator of `Token`s.
pub fn tokenize(input: &'_ str) -> impl Iterator<Item = Token> + '_ {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || {
        if cursor.is_empty() {
            None
        } else {
            cursor.reset_len_consumed();
            Some(cursor.eat_token())
        }
    })
}

/* ====================== */
/* ===== Structures ===== */
/* ====================== */

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    // This will be useful for knowing where a token ends without
    // having to recheck conditions on a second pass-through.
    pub len: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    /* Multi-char tokens */
    Whitespace,
    Identifier,                    // Includes keywords, ...
    Literal { kind: LiteralKind }, // Includes strings, ...

    /* Single-char tokens */
    Semi,       // ;
    OpenParen,  // (
    CloseParen, // )
    OpenBrace,  // {
    CloseBrace, // }
    Exclam,     // !

    /* Anythng else */
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    // `terminated` is useful because, while in ideal cases every string literal
    // is terminated, this is of course not always the case. In this case, it is
    // likely that we have read all the input and not found the terminating
    // character.
    Str { terminated: bool }, // "hi"
}

/* =========================== */
/* ===== Implementations ===== */
/* =========================== */

impl Token {
    pub fn new(kind: TokenKind, len: usize) -> Token {
        return Token { kind, len };
    }
}

impl Cursor<'_> {
    /// Consume from a cursor until a whole token has been constructed.
    /// We assume that the Cursor is nonempty.
    ///
    /// The `len` of the `Token` is basically determined by subtracting the
    /// number of characters left in the iterator from `Cursor.initial_len`,
    /// (which is initialized to be the length of the input). Notably, the
    /// `Cursor.inital_len` is not reset within this function, so if you are
    /// iteratively calling this function you must reset `Cursor.initial_len`
    /// before each use. Before is ideal, as you don't depend on the previous
    /// call to correctly reset.
    pub fn eat_token(&mut self) -> Token {
        let first_char = self.adv().unwrap();
        // Starting from the first character, try to determine what TokenKind
        // we have. Of course, one kind of character could indicate one of many
        // TokenKinds, and we deal with these in the most direct way.
        let token_kind = match first_char {
            /* Multi-character tokens */
            c if is_whitespace(c) => self.eat_whitespace(), // Whitespace
            c if is_id_start(c) => self.eat_id_continue(),  // Identifier

            /* Literals */
            '"' => {
                let is_terminated = self.eat_double_quote_str();
                let kind = Str {
                    terminated: is_terminated,
                };

                Literal { kind }
            }

            /* Single-character tokens (reserved characters) */
            ';' => Semi,
            '(' => OpenParen,
            ')' => CloseParen,
            '{' => OpenBrace,
            '}' => CloseBrace,
            '!' => Exclam,

            /* Anything else */
            _ => Unknown,
        };

        return Token::new(token_kind, self.len_consumed());
    }

    /// Consume whitespace until next character is not a whitespace character.
    fn eat_whitespace(&mut self) -> TokenKind {
        self.adv_until(is_whitespace);
        return Whitespace;
    }

    fn eat_id_continue(&mut self) -> TokenKind {
        self.adv_until(is_id_continue);
        return Identifier;
    }

    /// Advances until the terminating '"' is found. Return value is whether such
    /// a character was found.
    fn eat_double_quote_str(&mut self) -> bool {
        loop {
            match self.adv() {
                // Reached end of input
                None => break,
                // Found terminating character
                Some('"') => {
                    return true;
                }
                // Nothing special, keep advancing
                Some(_) => (),
            }
        }

        // Couldn't find a terminating character
        return false;
    }
}

/* ============================ */
/* ===== Helper functions ===== */
/* ============================ */

/// Determine whether the provided character could be the start of an
/// identifier. Identifiers are formally defined in the Rust syntax, and
/// we appropriately delegate this task to external functions. Details can
/// be found [here](https://doc.rust-lang.org/reference/identifiers.html).
pub fn is_id_start(c: char) -> bool {
    if c == '_' || unicode_xid::UnicodeXID::is_xid_start(c) {
        return true;
    }
    return false;
}

pub fn is_id_continue(c: char) -> bool {
    return unicode_xid::UnicodeXID::is_xid_continue(c);
}

pub fn is_whitespace(c: char) -> bool {
    matches! {
        c,
        '\t'
        | '\n'
        | ' '
    }
}

/* ===================== */
/* ====== Testing ====== */
/* ===================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let hello_world = r#"fn main() {
    println!("Hello, world!");
}"#;
        let mut tokens = tokenize(hello_world);

        for (n, token) in tokens.enumerate() {
            // println!("{:?}", token);

            match n {
                0 => {
                    // "fn"
                    assert_eq!(
                        token,
                        Token {
                            kind: Identifier,
                            len: 2
                        }
                    );
                }
                1 => {
                    // " "
                    assert_eq!(
                        token,
                        Token {
                            kind: Whitespace,
                            len: 1
                        }
                    );
                }
                2 => {
                    // "main"
                    assert_eq!(
                        token,
                        Token {
                            kind: Identifier,
                            len: 4
                        }
                    );
                }
                3 => {
                    // "("
                    assert_eq!(
                        token,
                        Token {
                            kind: OpenParen,
                            len: 1
                        }
                    )
                }
                4 => {
                    // ")"
                    assert_eq!(
                        token,
                        Token {
                            kind: CloseParen,
                            len: 1
                        }
                    )
                }
                5 => {
                    // " "
                    assert_eq!(
                        token,
                        Token {
                            kind: Whitespace,
                            len: 1
                        }
                    )
                }
                6 => {
                    // "{"
                    assert_eq!(
                        token,
                        Token {
                            kind: OpenBrace,
                            len: 1
                        }
                    )
                }
                7 => {
                    // "\n\t" == "\n    "
                    assert_eq!(
                        token,
                        Token {
                            kind: Whitespace,
                            len: 5
                        }
                    )
                }
                8 => {
                    // "println"
                    assert_eq!(
                        token,
                        Token {
                            kind: Identifier,
                            len: 7
                        }
                    )
                }
                9 => {
                    // "!"
                    assert_eq!(
                        token,
                        Token {
                            kind: Exclam,
                            len: 1
                        }
                    )
                }
                10 => {
                    // "("
                    assert_eq!(
                        token,
                        Token {
                            kind: OpenParen,
                            len: 1
                        }
                    )
                }
                11 => {
                    // ""Hello, world!""
                    assert_eq!(
                        token,
                        Token {
                            kind: Literal {
                                kind: Str { terminated: true }
                            },
                            len: 15 // includes quotes
                        }
                    )
                }
                12 => {
                    // ")"
                    assert_eq!(
                        token,
                        Token {
                            kind: CloseParen,
                            len: 1
                        }
                    )
                }
                13 => {
                    // ";"
                    assert_eq!(token, Token { kind: Semi, len: 1 })
                }
                14 => {
                    // "\n"
                    assert_eq!(
                        token,
                        Token {
                            kind: Whitespace,
                            len: 1
                        }
                    )
                }
                15 => {
                    // "}"
                    assert_eq!(
                        token,
                        Token {
                            kind: CloseBrace,
                            len: 1
                        }
                    )
                }
                _ => (),
            }
        }

        // cursor.reset_len_consumed();
        // cursor.reset_len_consumed();
    }
}
