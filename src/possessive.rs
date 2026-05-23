//! English possessive filter — removes trailing 's and s'.

use alloc::borrow::Cow;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Removes English possessive suffixes ('s and s').
///
/// Examples: "John's" → "John", "cats'" → "cats"
#[derive(Clone, Debug, Default)]
pub struct EnglishPossessiveFilter;

impl EnglishPossessiveFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for EnglishPossessiveFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        let len = text.len();
        if len < 3 {
            return (false, None);
        }

        // Check for 's or 's (right single quotation mark)
        if text.ends_with("'s") || text.ends_with("\u{2019}s") {
            let end = if text.as_bytes()[len - 2] == b'\'' {
                len - 2
            } else {
                // UTF-8: \u{2019} is 3 bytes + 's' is 1 byte = 4 bytes to remove
                len - 4
            };
            token.term = Cow::Owned(text[..end].to_string());
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possessive() {
        let f = EnglishPossessiveFilter::new();
        let mut token = Token::new("John's", 0, 6, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "John");
    }

    #[test]
    fn test_no_possessive() {
        let f = EnglishPossessiveFilter::new();
        let mut token = Token::new("hello", 0, 5, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "hello");
    }
}
