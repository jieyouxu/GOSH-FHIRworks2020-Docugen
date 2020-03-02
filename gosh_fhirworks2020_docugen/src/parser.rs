use pom::char_class::*;
use pom::parser::*;
use pom::Parser;

use crate::document::{DocumentTemplate, Partial};

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
pub fn string_literal() -> Parser<u8, Partial> {
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
pub fn tag() -> Parser<u8, Partial> {
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

/// A `Partial` is either a `StringLiteral` or a `Tag`.
pub fn partial() -> Parser<u8, Partial> {
    string_literal() | tag()
}

/// A `DocumentTemplate` consists of a list of `Partial`s.
pub fn document_template() -> Parser<u8, DocumentTemplate> {
    let partials = skip_whitespace() * partial().repeat(0..) - end();
    partials.map(|ps| DocumentTemplate::with_partials(&ps))
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

    #[test]
    fn test_document_template() -> Result<(), String> {
        let raw = b"abc {{def}} ghi";
        let expected_document_template =
            DocumentTemplate::with_partials(&vec![
                Partial::StringLiteral("abc ".to_string()),
                Partial::Tag("def".to_string()),
                Partial::StringLiteral(" ghi".to_string()),
            ]);

        assert_eq!(
            expected_document_template,
            document_template().parse(raw).unwrap()
        );

        Ok(())
    }
}
