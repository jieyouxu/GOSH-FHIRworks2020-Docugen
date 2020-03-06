use serde::Deserialize;
use crate::data::patient::Patient;

/// Each `Bundle` contains multiple `Patient`s.
#[derive(Debug, PartialEq, Deserialize)]
pub struct Bundle {
    
}

