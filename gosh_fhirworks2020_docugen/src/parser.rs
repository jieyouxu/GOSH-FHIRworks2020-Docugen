use pom::char_class::*;
use pom::parser::*;
use pom::Parser;

use crate::document::Partial;

/// A `StringLiteral` parser combinator is responsible for parsing the following
/// fragment:
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

/// The `tag` parser combinator is responsible for parsing a `Tag(identifier)`
/// which is delimited between `{{ tag_id }}`.
///
/// ```enbf
/// <Tag> ::= "{{" <TagId> "}}"
/// <TagId> ::= [a-zA-Z][a-zA-Z0-9]*
/// ```
pub(crate) fn tag() -> Parser<u8, Partial> {
    let tag_left_delimiter = seq(b"{{").discard();
    let tag_right_delimiter = seq(b"}}").discard();

    let tag_identifier = is_a(|c| alpha(c)) + is_a(|c| alphanum(c)).repeat(0..);

    let tag = tag_left_delimiter * skip_whitespace() + tag_identifier
        - skip_whitespace()
        + tag_right_delimiter;

    tag.convert(|(((), (head, tail)), ())| {
        let (head, tail) = (
            std::str::from_utf8(std::slice::from_ref(&head)),
            std::str::from_utf8(&tail[..]),
        );
        if let (Ok(head), Ok(tail)) = (&head, &tail) {
            let mut id = String::new();
            id.push_str(head);
            id.push_str(tail);
            Ok(id)
        } else {
            Err(pom::Error::Custom {
                message: "failed to parse".to_string(),
                position: 0,
                inner: None,
            })
        }
    })
    .map(|s| Partial::Tag(s.to_string()))
}

fn skip_whitespace() -> Parser<u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_string_literal() -> Result<(), String> {
        let raw = b"HELLO_WORLD";
        let expected_string_literal =
            Partial::StringLiteral("HELLO_WORLD".to_string());

        assert_eq!(
            expected_string_literal,
            string_literal().parse(raw).unwrap()
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
    fn test_escaped_left_brace() -> Result<(), String> {
        let raw = b"\\{";
        let expected_string_literal = Partial::StringLiteral("{".to_string());

        assert_eq!(
            expected_string_literal,
            string_literal().parse(raw).unwrap()
        );

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_unescaped_right_brace() {
        let raw = b"}";
        string_literal().parse(raw).unwrap();
    }

    #[test]
    fn test_escaped_right_brace() -> Result<(), String> {
        let raw = b"\\}";
        let expected_string_literal = Partial::StringLiteral("}".to_string());

        assert_eq!(
            expected_string_literal,
            string_literal().parse(raw).unwrap()
        );

        Ok(())
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
        let expected_string_literal = Partial::StringLiteral("\\".to_string());

        assert_eq!(
            expected_string_literal,
            string_literal().parse(raw).unwrap()
        );

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_empty_tag() {
        let raw = b"{{}}";
        tag().parse(raw).unwrap();
    }

    #[test]
    fn test_tag() -> Result<(), String> {
        let raw = b"{{abc}}";
        let expected_tag = Partial::Tag("abc".to_string());
        assert_eq!(expected_tag, tag().parse(raw).unwrap());

        Ok(())
    }

    #[test]
    fn test_tag_whitespace() -> Result<(), String> {
        let raw = b"{{ \t xxxx   }}";
        let expected_tag = Partial::Tag("xxxx".to_string());
        assert_eq!(expected_tag, tag().parse(raw).unwrap());

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_malformed_tag() {
        let raw = b"{{ \t separated identifiers illegal }}";
        tag().parse(raw).unwrap();
    }
}
