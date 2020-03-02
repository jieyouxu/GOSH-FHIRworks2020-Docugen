use docugen::document;
use gosh_fhirworks2020_docugen as docugen;

#[test]
fn test_string_to_template() -> Result<(), String> {
    let raw = r#"
Dear {{ receiver_name }},

Please find attached \{{{ attachment_name }}\}!

Sincerely,

{{ sender_name }}
{{ sender_title }}
{{ hospital_name }}
"#;

    let template = docugen::document::DocumentTemplate;
}
