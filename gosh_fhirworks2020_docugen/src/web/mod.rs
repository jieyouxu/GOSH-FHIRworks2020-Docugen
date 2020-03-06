use super::data::patient::Patient;
use log::{debug, error, info};
use reqwest;
use serde::Deserialize;

/// Patient data from the FHIR web API is returned in `Bundle`s of approximately
/// 10 `Patient`s each. The `Patient`s are themselves encapsulated by an `Entry`
/// wrapper.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Bundle {
    id: String,
    #[serde(rename = "entry")]
    entries: Vec<Entry>,
}

/// Each `Entry` encapsulates a `Patient` and provides additional metadata.
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    resource: Patient,
}

pub async fn get_patients(
    endpoint: &str,
) -> Result<Vec<Patient>, Box<dyn std::error::Error>> {
    info!("Requesting patient data from {}", endpoint);

    let response = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?
        .get(endpoint)
        .send()
        .await?
        .text()
        .await
        .expect("failed to get request body");

    println!("{}", &response[0..50]);

    let response: Vec<Bundle> = match serde_json::from_str(&response) {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to parse response!");
            error!("{:#?}", e);
            std::process::exit(1);
        }
    };

    // We need to pull `Patient` out of the various layers.
    let response = response
        .into_iter()
        .flat_map(|r| r.entries)
        .map(|e| e.resource)
        .collect();

    debug!("Response received = {:#?}", &response);

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_bundle() {
        let raw = r#"[
            {
                "id": "123",
                "entry": [
                    {
                        "resource": {
                            "name": [
                                {
                                    "given": ["A"],
                                    "family": "B"
                                }
                            ],
                            "birthDate": "1234-12-12"
                        }
                    }
                ]
            }
        ]"#;

        assert!(serde_json::from_str::<Vec<Bundle>>(raw).is_ok());
    }
}

