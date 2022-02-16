use std::str::Chars;

const EOF_CHAR: char = '\0';

/* ====================== */
/* ===== Structures ===== */
/* ====================== */

/// How we walk through an input, e.g. a `&str`.
pub struct Cursor<'a> {
    // Useful for checking ...
    initial_len: usize,
    // Iterator over (Unicode) characters, lives as long as Cursor
    chars: Chars<'a>,
    // Enables peeking
    prev: char,
}

/* =========================== */
/* ===== Implementations ===== */
/* =========================== */

impl<'a> Cursor<'a> {
    /// Create a new instance of `Cursor`.
    pub fn new(input: &'a str) -> Cursor<'a> {
        return Cursor {
            initial_len: input.len(),
            chars: input.chars(),
            prev: '\0',
        };
    }

    /// Check if there is anything left in the `Cursor`. The reason to prefer this
    /// over using `adv()` and matching `None` is that we don't consume the iterator,
    /// and we don't want to use `peek()` because it is inefficient in that it
    /// requires cloning the interator.
    pub fn is_empty(&self) -> bool {
        return self.chars.as_str().len() == 0;
    }

    pub fn len_consumed(&self) -> usize {
        return self.initial_len - self.chars.as_str().len();
    }

    pub fn reset_len_consumed(&mut self) {
        self.initial_len = self.chars.as_str().len();
    }

    /// Advance the Cursor by one character, consuming one in the process and
    /// storing the consumed character in `self.prev`.
    pub fn adv(&mut self) -> Option<char> {
        let consumed_char = self.chars.next()?;

        self.prev = consumed_char;

        return Some(consumed_char);
    }

    /// When we have no more characters to peek, we return an EOF, consistent with
    /// how we expect this to be used. Unlike `adv()`, we do not return `None`
    /// because...
    pub fn peek(&self) -> char {
        let c: char;
        match self.chars.clone().next() {
            Some(ch) => {
                c = ch;
            }
            None => {
                c = EOF_CHAR;
            }
        }
        return c;
    }

    /// Advance cursor until a condition, provided as a parameter, is
    /// no longer satisfied.
    ///
    /// `condition`: it needs to be mutable so that we can call it. It is of
    ///      type `FnMut` because the function we pass into it may take a
    ///      mutable reference as an argument...
    pub fn adv_until(&mut self, mut condition: impl FnMut(char) -> bool) {
        while condition(self.peek()) && !self.is_empty() {
            self.adv();
        }
    }
}

/* =================== */
/* ===== Testing ===== */
/* =================== */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adv() {
        let input = "ab";
        let mut cursor = Cursor::new(input);

        assert_eq!(cursor.adv(), Some('a'));
        assert_eq!(cursor.adv(), Some('b'));
        // Try calling when there is nothing left
        assert_eq!(cursor.adv(), None);
    }

    #[test]
    fn test_peek() {
        let input = "a";
        let mut cursor = Cursor::new(input);

        assert_eq!(cursor.peek(), 'a');
        // Peek when there is nothing there
        cursor.adv();
        assert_eq!(cursor.peek(), EOF_CHAR);
    }

    #[test]
    fn test_adv_until() {
        // Advance until we hit a digit
        let input = "abcds9";
        let mut cursor = Cursor::new(input);

        fn condition_notdigit(c: char) -> bool {
            if c.is_digit(10) {
                return false;
            }
            return true;
        }
        cursor.adv_until(condition_notdigit);

        assert_eq!(cursor.adv(), Some('9'));
    }
}
