use super::fhir_date::FHIRDate;
use serde::Deserialize;

/// Each `Patient` is a resource as described in FHIR v4.0.1's `Patient` JSON
/// template. This `Patient` definition is only a subset of that in the official
/// FHIR.
///
/// # Reference
///
/// - [FHIR | Patient](https://www.hl7.org/fhir/patient.html#resource)
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct Patient {
    #[serde(rename = "name")]
    names: Vec<HumanName>,
    #[serde(rename = "camelCase")]
    birth_date: FHIRDate,
}

/// Each `Patient` has one or more `HumanName`s. A `HumanName` contains more
/// attributes in the official specification so this definition is a subset of
/// the official definition.
///
/// # Reference
///
/// - [Human Name](https://www.hl7.org/fhir/datatypes.html#HumanName).
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub struct HumanName {
    family: String,
    given: String,
}
