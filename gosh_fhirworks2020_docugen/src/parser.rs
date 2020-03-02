use pom::parser::*;
use pom::Parser;

use crate::document::{DocumentTemplate, Partial};

/// A `StringLiteral` is:
///
/// ```ebnf
/// <StringLiteral> ::= <StringLiteralCharacter>+
///
/// <StringLiteralCharacter> ::= <EscapedCharacter>
///                          |   <UnescapedCharacter>
///
/// <EscapedCharacter> ::= '\\{'
///                    |   '\\}'
///                    |   '\\'
///                    |   '\\t'
///                    |   '\\b'
///                    |   '\\f'
///
/// <UnescapedCharacter> ::= [^\\{}]
/// ```
pub(crate) fn string_literal() -> Parser<u8, Partial> {
    let special_char = sym(b'\\').map(|_| b'\\')
        | sym(b'{').map(|_| b'{')
        | sym(b'}').map(|_| b'}');
    let escape_sequence = sym(b'\\') * special_char;
    let string = (none_of(b"\\}{") | escape_sequence).repeat(1..);
    string
        .convert(String::from_utf8)
        .map(|s| Partial::StringLiteral(s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_string_literal() -> Result<(), String> {
        let raw_string_literal = br#"HELLO_WORLD"#;
        let parsed_string_literal =
            match string_literal().parse(raw_string_literal) {
                Ok(Partial::StringLiteral(s)) => s,
                Err(e) => {
                    return Err(e.to_string());
                }
                _ => unreachable!(),
            };

        assert_eq!(
            std::str::from_utf8(raw_string_literal)
                .map_err(|e| e.to_string())?,
            parsed_string_literal
        );
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_unescaped_left_brace() {
        let raw = b"{";
        string_literal().parse(raw).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_unescaped_right_brace() {
        let raw = b"}";
        string_literal().parse(raw).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_unescaped_backslash() {
        let raw = b"\\";
        string_literal().parse(raw).unwrap();
    }

    #[test]
    fn test_escaped_backslash() -> Result<(), String> {
        let raw = b"\\\\";
        let parsed_literal = string_literal().parse(raw).unwrap();
        let parsed_literal = match parsed_literal {
            Partial::StringLiteral(s) => s,
            _ => {
                return Err("unexpected branch".to_string());
            }
        };

        assert_eq!("\\", parsed_literal);

        Ok(())
    }
}
