use serde::de::{Deserialize, Deserializer};
use std::fmt;
use std::str::FromStr;

/// Each `FHIRDate` is either:
///
/// 1. A full date: `YYYY-MM-DD`; or
/// 2. A partial date with only the year: `YYYY`; or
/// 3. A partial date with both the year and the month: `YYYY-MM`.
///
/// # Reference
///
/// - [date](https://www.hl7.org/fhir/datatypes.html#date)
#[derive(Debug, PartialEq, Clone)]
pub struct FHIRDate {
    year: u32,
    month: Option<u32>,
    day: Option<u32>,
}

/// We try to parser a `&str` into a `FHIRDate`.
pub fn deserialize_fhirdate(s: &str) -> Result<FHIRDate, String> {
    let s = s.trim();

    // Case: year-only; when `s` contains only unsigned integer.
    if let Ok(year) = s.parse::<u32>() {
        return Ok(FHIRDate {
            year,
            month: None,
            day: None,
        });
    }

    // Case: delimiter `-` exists.
    // We first `split` the date string into parts.
    let parts = s.split('-').collect::<Vec<&str>>();

    // Then we try to parse each part as a `u32` as required. If any part fails
    // to parse as `u32`, the date is invalid.
    let parts: Vec<Result<u32, _>> =
        parts.iter().map(|s| s.parse::<u32>()).collect();

    // We require that all date parts are valid `u32`s.
    if !parts.iter().all(|r| r.is_ok()) {
        return Err("invalid date".to_string());
    }

    // Safe to `unwrap` because of the previous check.
    let parts = parts.into_iter().map(|s| s.unwrap()).collect::<Vec<u32>>();

    match &parts[..] {
        [year, month] => Ok(FHIRDate {
            year: *year,
            month: Some(*month),
            day: None,
        }),
        [year, month, day] => Ok(FHIRDate {
            year: *year,
            month: Some(*month),
            day: Some(*day),
        }),
        _ => Err("invalid date".to_string()),
    }
}

impl FromStr for FHIRDate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        deserialize_fhirdate(s)
    }
}

impl<'de> Deserialize<'de> for FHIRDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl fmt::Display for FHIRDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FHIRDate {
                year,
                month: None,
                day: None,
            } => write!(f, "{:0>4}", year),
            FHIRDate {
                year,
                month: Some(month),
                day: None,
            } => write!(f, "{:0>4}-{:0>2}", year, month),
            FHIRDate {
                year,
                month: Some(month),
                day: Some(day),
            } => write!(f, "{:0>4}-{:0>2}-{:0>2}", year, month, day),
            _ => Err(fmt::Error),
        }
    }
}

pub fn serialize_fhirdate(date: &FHIRDate) -> String {
    format!("{}", date)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_year() {
        let raw = "2019";
        let expected = FHIRDate {
            year: 2019,
            month: None,
            day: None,
        };
        assert_eq!(expected, deserialize_fhirdate(raw).unwrap());
    }

    #[test]
    fn test_year_month() {
        let raw = "2019-01";
        let expected = FHIRDate {
            year: 2019,
            month: Some(1),
            day: None,
        };
        assert_eq!(expected, deserialize_fhirdate(raw).unwrap());
    }

    #[test]
    fn test_full_date() {
        let raw = "2019-01-23";
        let expected = FHIRDate {
            year: 2019,
            month: Some(1),
            day: Some(23),
        };
        assert_eq!(expected, deserialize_fhirdate(raw).unwrap());
    }

    #[test]
    #[should_panic]
    fn test_invalid_date() {
        let raw = "2019-";
        deserialize_fhirdate(raw).unwrap();
    }

    #[test]
    fn test_serialize_year() {
        let s = serialize_fhirdate(&FHIRDate {
            year: 1,
            month: None,
            day: None,
        });

        assert_eq!("0001", &s);
    }

    #[test]
    fn test_serialize_year_month() {
        let s = serialize_fhirdate(&FHIRDate {
            year: 1,
            month: Some(1),
            day: None,
        });

        assert_eq!("0001-01", &s);
    }

    #[test]
    fn test_serialize_full_date() {
        let s = serialize_fhirdate(&FHIRDate {
            year: 1,
            month: Some(1),
            day: Some(1),
        });

        assert_eq!("0001-01-01", &s);
    }
}
