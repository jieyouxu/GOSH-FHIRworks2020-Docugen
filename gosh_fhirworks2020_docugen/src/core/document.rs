/// A `DocumentTemplate` mimics a [mustache](https://mustache.github.io/)
/// template. A template consists of a list of `Partial`s.
#[derive(Debug, PartialEq)]
pub struct DocumentTemplate {
    pub partials: Vec<Partial>,
}

impl DocumentTemplate {
    pub fn new() -> Self {
        DocumentTemplate::default()
    }

    pub fn with_partials(partials: &[Partial]) -> Self {
        Self {
            partials: partials.to_vec(),
        }
    }

    pub fn add_partial(&mut self, partial: &Partial) {
        self.partials.push(partial.clone());
    }
}

impl Default for DocumentTemplate {
    fn default() -> Self {
        Self {
            partials: Vec::new(),
        }
    }
}

pub type Identifier = String;

/// Each `Partial` is either a UTF-8 `StringLiteral`, or a `Tag`.
#[derive(Debug, PartialEq, Clone)]
pub enum Partial {
    StringLiteral(String),
    Tag(Identifier),
}

/// A `FilledDocument` is generated from a `DocumentTemplate` with the required
/// `Tag`s filled in.
#[derive(Debug, PartialEq)]
pub struct FilledDocument(String);

impl FilledDocument {
    pub fn document(&self) -> &str {
        &self.0
    }
}

/// A `TagPair` is an association between the tag name `key` and the `value`
/// that should be used to fill its place.
#[derive(Debug, PartialEq)]
pub struct TagPair {
    pub key: String,
    pub value: String,
}

/// Cause of error when trying to fill a `DocumentTemplate`.
#[derive(Debug, PartialEq)]
pub enum TemplateError {
    MissingRequiredTagValue(Identifier),
    NonExhaustiveTags(Vec<Identifier>),
}

impl DocumentTemplate {
    pub fn saturate(
        &self,
        tag_pairs: &[TagPair],
    ) -> Result<FilledDocument, TemplateError> {
        let mut content = String::new();

        // TODO: replace this `O(n^2)` loop with a `O(1)` `HashMap`. Currently
        // this requires iterating over `self.partials` in the outer loop and
        // iterating over `tag_pairs` in the inner loop in the worst case
        // scenario.
        for partial in &self.partials[..] {
            match partial {
                Partial::StringLiteral(s) => content.push_str(s),
                Partial::Tag(id) => {
                    let tag_value = saturate_or_error(tag_pairs, id)?;
                    content.push_str(tag_value);
                }
            }
        }

        Ok(FilledDocument(content))
    }
}

fn saturate_or_error<'a>(
    tag_pairs: &'a [TagPair],
    tag_key: &'a str,
) -> Result<&'a str, TemplateError> {
    match tag_pairs.iter().find(|t| t.key == tag_key) {
        Some(TagPair { value, .. }) => Ok(value),
        None => {
            Err(TemplateError::MissingRequiredTagValue(tag_key.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_tags() {
        let template = DocumentTemplate::new();
        let saturated = template.saturate(&[]);
        assert!(saturated.is_ok());
    }

    #[test]
    fn test_one_tag() {
        let template = DocumentTemplate::with_partials(&[
            Partial::StringLiteral("Hello ".to_string()),
            Partial::Tag("name".to_string()),
            Partial::StringLiteral(", welcome!".to_string()),
        ]);

        let filled_document = template
            .saturate(&[TagPair {
                key: "name".to_string(),
                value: "Joe".to_string(),
            }])
            .unwrap();

        let expected_string = "Hello Joe, welcome!".to_string();

        assert_eq!(expected_string, filled_document.document());
    }

    #[test]
    #[should_panic]
    fn test_non_existent_tag() {
        let template = DocumentTemplate::with_partials(&[Partial::Tag(
            "name".to_string(),
        )]);

        template
            .saturate(&[TagPair {
                key: "Hello".to_string(),
                value: "___".to_string(),
            }])
            .unwrap();
    }

    #[test]
    fn test_multiple_tags() {
        let template = DocumentTemplate::with_partials(&[
            Partial::StringLiteral("<S1>".to_string()),
            Partial::Tag("T1".to_string()),
            Partial::StringLiteral("<S2>".to_string()),
            Partial::Tag("T2".to_string()),
            Partial::Tag("T1".to_string()),
            Partial::Tag("T3".to_string()),
        ]);

        let filled_document = template
            .saturate(&[
                TagPair {
                    key: "T1".to_string(),
                    value: "T1V".to_string(),
                },
                TagPair {
                    key: "T2".to_string(),
                    value: "T2V".to_string(),
                },
                TagPair {
                    key: "T3".to_string(),
                    value: "T3V".to_string(),
                },
            ])
            .unwrap();

        let expected_string = "<S1>T1V<S2>T2VT1VT3V".to_string();

        assert_eq!(expected_string, filled_document.document());
    }
}
