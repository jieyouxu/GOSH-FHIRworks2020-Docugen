use pom::parser::*;
use pom::Parser;

use crate::document::{DocumentTemplate, Partial};

/// A `StringLiteral` is:
///
/// ```ebnf
/// <StringLiteral> ::= <StringLiteralCharacter>+
/// <StringLiteralCharacter> ::= <EscapedCharacter>
///                          ::= <UnescapedCharacter>
///
/// <UnescapedCharacter> ::= [^\\{}]
/// ```
pub(crate) fn string_literal() -> Parser<u8, Partial> {
    let special_char = sym(b'\\')
        | sym(b'{').map(|_| b'{')
        | sym(b'}').map(|_| b'}')
        | sym(b'f').map(|_| b'\x0C')
        | sym(b'r').map(|_| b'\r')
        | sym(b'b').map(|_| b'\x08')
        | sym(b'n').map(|_| b'\n')
        | sym(b't').map(|_| b'\t');
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
        let parsed_string_literal = match string_literal().parse(raw_string_literal) {
            Ok(Partial::StringLiteral(s)) => s,
            Err(e) => {
                return Err(e.to_string());
            }
            _ => unreachable!(),
        };

        assert_eq!(
            std::str::from_utf8(raw_string_literal).map_err(|e| e.to_string())?,
            parsed_string_literal
        );
        Ok(())
    }

    #[test]
    fn test_unescaped_left_brace() -> Result<(), String> {
        unimplemented!()
    }
}
