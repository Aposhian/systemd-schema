use crate::unit::UnitSection;
use crate::service::ServiceSection;
use crate::install::InstallSection;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Service {
    pub unit: UnitSection,
    pub service: ServiceSection,
    pub install: InstallSection
}