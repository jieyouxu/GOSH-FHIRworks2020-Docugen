use pom::char_class::*;
use pom::parser::*;

use super::document::{DocumentTemplate, Partial};

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
pub fn string_literal<'a>() -> Parser<'a, u8, Partial> {
    let special_char = sym(b'\\').map(|_| b'\\')
        | sym(b'{').map(|_| b'{')
        | sym(b'}').map(|_| b'}');
    let escape_sequence = sym(b'\\') * special_char;
    let string = (none_of(b"\\}{") | escape_sequence).repeat(1..);
    string
        .convert(String::from_utf8)
        .map(Partial::StringLiteral)
}

/// The `tag` parser combinator is responsible for parsing a `Tag(identifier)`
/// which is delimited between `{{ tag_id }}`.
///
/// ```enbf
/// <Tag> ::= "{{" <TagId> "}}"
/// <TagId> ::= [a-zA-Z][_a-zA-Z0-9]*
/// ```
pub fn tag<'a>() -> Parser<'a, u8, Partial> {
    let tag_left_delimiter = seq(b"{{").discard();
    let tag_right_delimiter = seq(b"}}").discard();

    let tag = tag_left_delimiter * skip_whitespace() * tag_id()
        - skip_whitespace()
        - tag_right_delimiter;

    tag.map(Partial::Tag)
}

fn tag_id<'a>() -> Parser<'a, u8, String> {
    let id = tag_id_head() + tag_id_tail();
    id.map(|(head, tail)| {
        let mut s = String::new();
        s.push_str(&head);
        s.push_str(&tail);
        s
    })
}

fn tag_id_head<'a>() -> Parser<'a, u8, String> {
    let head = is_a(alpha) | sym(b'_');
    head.map(|v| vec![v]).convert(String::from_utf8)
}

fn tag_id_tail<'a>() -> Parser<'a, u8, String> {
    let tail = (is_a(alphanum) | sym(b'_')).repeat(0..);
    tail.convert(String::from_utf8)
}

fn skip_whitespace<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

/// A `Partial` is either a `StringLiteral` or a `Tag`.
pub fn partial<'a>() -> Parser<'a, u8, Partial> {
    string_literal() | tag()
}

/// A `DocumentTemplate` consists of a list of `Partial`s.
pub fn document_template<'a>() -> Parser<'a, u8, DocumentTemplate> {
    let partials = partial().repeat(0..) - end();
    partials.map(|ps| DocumentTemplate::with_partials(&ps))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_ascii_string_literal() {
        let raw = b"HELLO_WORLD";
        let expected_string_literal =
            Partial::StringLiteral("HELLO_WORLD".to_string());

        assert_eq!(
            expected_string_literal,
            string_literal().parse(raw).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_unescaped_left_brace() {
        let raw = b"{";
        string_literal().parse(raw).unwrap();
    }

    #[test]
    fn test_escaped_left_brace() {
        let raw = b"\\{";
        let expected_string_literal = Partial::StringLiteral("{".to_string());

        assert_eq!(
            expected_string_literal,
            string_literal().parse(raw).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_unescaped_right_brace() {
        let raw = b"}";
        string_literal().parse(raw).unwrap();
    }

    #[test]
    fn test_escaped_right_brace() {
        let raw = b"\\}";
        let expected_string_literal = Partial::StringLiteral("}".to_string());

        assert_eq!(
            expected_string_literal,
            string_literal().parse(raw).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_unescaped_backslash() {
        let raw = b"\\";
        string_literal().parse(raw).unwrap();
    }

    #[test]
    fn test_escaped_backslash() {
        let raw = b"\\\\";
        let expected_string_literal = Partial::StringLiteral("\\".to_string());

        assert_eq!(
            expected_string_literal,
            string_literal().parse(raw).unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn test_empty_tag() {
        let raw = b"{{}}";
        tag().parse(raw).unwrap();
    }

    #[test]
    fn test_tag() {
        let raw = b"{{abc}}";
        let expected_tag = Partial::Tag("abc".to_string());
        assert_eq!(expected_tag, tag().parse(raw).unwrap());
    }

    #[test]
    fn test_tag_id_with_middle_underscore() {
        let raw = b"{{ a_c }}";
        let expected_tag = Partial::Tag("a_c".to_string());
        assert_eq!(expected_tag, tag().parse(raw).unwrap());
    }

    #[test]
    fn test_tag_id_with_starting_underscore() {
        let raw = b"{{ _x }}";
        let expected_tag = Partial::Tag("_x".to_string());
        assert_eq!(expected_tag, tag().parse(raw).unwrap());
    }

    #[test]
    fn test_tag_id_with_trailing_underscore() {
        let raw = b"{{ a_ }}";
        let expected_tag = Partial::Tag("a_".to_string());
        assert_eq!(expected_tag, tag().parse(raw).unwrap());
    }

    #[test]
    fn test_tag_whitespace() {
        let raw = b"{{ \t xxxx   }}";
        let expected_tag = Partial::Tag("xxxx".to_string());
        assert_eq!(expected_tag, tag().parse(raw).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_malformed_tag() {
        let raw = b"{{ \t separated identifiers illegal }}";
        tag().parse(raw).unwrap();
    }

    #[test]
    fn test_document_template() {
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
    }
}
