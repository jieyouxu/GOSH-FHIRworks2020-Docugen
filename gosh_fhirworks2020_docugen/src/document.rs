/// A `DocumentTemplate` mimics a [mustache](https://mustache.github.io/)
/// template. A template consists of a list of `Partial`s.
#[derive(Debug, PartialEq)]
pub(crate) struct DocumentTemplate {
    pub(crate) partials: Vec<Partial>;
}

pub(crate) type Identifier = String;

/// Each `Partial` is either a UTF-8 `StringLiteral`, or a `Tag`.
#[derive(Debug, PartialEq)]
pub(crate) enum Partial {
    pub(crate) StringLiteral(String),
    pub(crate) Tag(Identifier)
}

/// A `FilledDocument` is generated from a `DocumentTemplate` with the required
/// `Tag`s filled in.
pub(crate) struct FilledDocument(String);

/// A `TagPair` is an association between the tag name `key` and the `value`
/// that should be used to fill its place.
pub(crate) struct TagPair {
    pub(crate) key: String,
    pub(crate) value: String,
}

pub(crate) enum TemplateError {
    MissingRequiredTagValue(Identifier)
}

impl DocumentTemplate {
    fn saturate(&self, tag_pairs: &[TagPair]) -> Result<FilledDocument, TemplateError> {
        let result = String::new();

        for partial in &self.partials[..] {
            match partial {
                StringLiteral(s) => result.push_str(s),
                Tag(id) => {
                    let tag_value = match tag_pairs.iter().find(|t| t.key == id) {
                        Some(v) => v,
                        None => {
                            return TemplateError(id);
                        }
                    };

                    result.push_str(tag_value);
                }
            }
        }

        Ok(result)
    }
}

