/// A `DocumentTemplate` mimics a [mustache](https://mustache.github.io/)
/// template. A template consists of a list of `Partial`s.
#[derive(Debug, PartialEq)]
pub(crate) struct DocumentTemplate {
    pub(crate) partials: Vec<Partial>,
}

impl DocumentTemplate {
    fn new() -> Self {
        Self {
            partials: Vec::new(),
        }
    }

    fn with_partials(partials: &[Partial]) -> Self {
        Self {
            partials: partials.to_vec(),
        }
    }
}

pub(crate) type Identifier = String;

/// Each `Partial` is either a UTF-8 `StringLiteral`, or a `Tag`.
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum Partial {
    StringLiteral(String),
    Tag(Identifier),
}

/// A `FilledDocument` is generated from a `DocumentTemplate` with the required
/// `Tag`s filled in.
pub(crate) struct FilledDocument(String);

/// A `TagPair` is an association between the tag name `key` and the `value`
/// that should be used to fill its place.
#[derive(Debug, PartialEq)]
pub(crate) struct TagPair {
    pub(crate) key: String,
    pub(crate) value: String,
}

/// Cause of error when trying to fill a `DocumentTemplate`.
pub(crate) enum TemplateError {
    MissingRequiredTagValue(Identifier),
    NonExhaustiveTags(Vec<Identifier>),
}

impl DocumentTemplate {
    fn saturate(
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
    match tag_pairs.iter().find(|&t| &t.key == tag_key) {
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
    fn test_no_tags() -> Result<(), String> {
        let template = DocumentTemplate::new();
        let saturated = template.saturate(&[]);
        assert!(saturated.is_ok());
        Ok(())
    }
}
