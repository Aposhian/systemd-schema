use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::common::UnitReferenceList;


#[skip_serializing_none]
#[derive(Serialize, Default)]
pub struct InstallSection {
    #[serde(rename = "WantedBy")]
    pub wanted_by: Option<UnitReferenceList>,
    #[serde(rename = "RequiredBy")]
    pub required_by: Option<UnitReferenceList>,
    #[serde(rename = "Alias")]
    pub alias: Option<UnitReferenceList>,
    #[serde(rename = "Also")]
    pub also: Option<UnitReferenceList>,
    #[serde(rename = "DefaultInstance")]
    pub default_instance: Option<&'static str> // should this be a UnitReference?
}