
/* ========================== */
/* ===== Key Structures ===== */
/* ========================== */

pub struct TokenStream(Vec<TreeAndSpacing>);

pub enum Spacing {
    Alone,
    Joint,
}

pub enum TokenTree {
    /// A single token.
    Token(Token),
    /// A delimited sequence of token trees.
    Delimited(DelimToken, TokenStream),
}

type TreeAndSpacing = (TokenTree, Spacing)
