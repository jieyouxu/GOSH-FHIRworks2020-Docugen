use docugen::core::document::TagPair;
use docugen::core::parser::document_template;
use log::debug;

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

    let template = document_template().parse(raw.as_bytes()).unwrap();
    let tag_pairs: &[TagPair] = &vec![
        TagPair {
            key: "receiver_name".to_string(),
            value: "Moon Moon".to_string(),
        },
        TagPair {
            key: "attachment_name".to_string(),
            value: "Word.exe".to_string(),
        },
        TagPair {
            key: "sender_name".to_string(),
            value: "Shiba Inu".to_string(),
        },
        TagPair {
            key: "sender_title".to_string(),
            value: "Professuer of Bepis Studies".to_string(),
        },
        TagPair {
            key: "hospital_name".to_string(),
            value: "Dogeland Hospital".to_string(),
        },
    ];

    let filled_document = template.saturate(tag_pairs).unwrap();
    let actual_filled_content = filled_document.document();

    let expected_filled_content = r#"
Dear Moon Moon,

Please find attached {Word.exe}!

Sincerely,

Shiba Inu
Professuer of Bepis Studies
Dogeland Hospital
"#;

    debug!("expected = `{}`", expected_filled_content);
    debug!("actual = `{}`", actual_filled_content);

    assert_eq!(expected_filled_content, actual_filled_content);

    Ok(())
}
