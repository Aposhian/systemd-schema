use std::path::Path;
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::common::{Timespan, UnitReferenceList};
use crate::condition::Condition;
use crate::resource_control::ResourceControl;

// https://www.freedesktop.org/software/systemd/man/systemd.unit.html
#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct UnitSection {
    pub description: Option<&'static str>,
    pub documentation: Option<&'static str>,
    pub wants: Option<UnitReferenceList>,
    pub requires: Option<UnitReferenceList>,
    pub requisite: Option<UnitReferenceList>,
    pub binds_to: Option<UnitReferenceList>,
    pub part_of: Option<UnitReferenceList>,
    pub upholds: Option<UnitReferenceList>,
    pub conflicts: Option<UnitReferenceList>,
    pub before: Option<UnitReferenceList>,
    pub after: Option<UnitReferenceList>,
    pub on_failure: Option<UnitReferenceList>,
    pub on_success: Option<UnitReferenceList>,
    pub propogates_reload_to: Option<UnitReferenceList>,
    pub reload_propagated_from: Option<UnitReferenceList>,
    pub propagates_stop_to: Option<UnitReferenceList>,
    pub stop_propagated_from: Option<UnitReferenceList>,
    pub joins_namespace_of: Option<UnitReferenceList>,
    pub requires_mounts_for: Option<Vec<&'static Path>>,
    pub on_failure_job_mode: Option<OnFailureJobMode>,
    pub ignore_on_isolate: Option<bool>,
    pub stop_when_unneeded: Option<bool>,
    pub refuse_manual_start: Option<bool>,
    pub refuse_manual_stop: Option<bool>,
    pub allow_isolate: Option<bool>,
    pub default_dependencies: Option<YesNo>,
    pub collect_mode: Option<CollectMode>,
    pub failure_action: Option<FailureSuccessAction>,
    pub success_action: Option<FailureSuccessAction>,
    pub failure_action_exit_status: Option<u8>,
    pub success_action_exit_status: Option<u8>,
    pub job_timeout_sec: Option<Timespan>,
    pub job_running_timeout_sec: Option<Timespan>,
    pub job_timeout_action: Option<FailureSuccessAction>,
    pub job_timeout_reboot_argument: Option<RebootArgument>,
    pub start_limit_interval_sec: Option<Timespan>,
    pub start_limit_burst: Option<u32>,
    pub start_limit_action: Option<FailureSuccessAction>,
    pub reboot_argument: Option<RebootArgument>,
    pub source_path: Option<&'static Path>,
    #[serde(flatten)]
    pub resource_control: Option<ResourceControl>,
    // TODO: add condition serialization
    // conditions: Option<Vec<Condition>>
}

type RebootArgument = &'static str;

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FailureSuccessAction {
    None,
    Reboot,
    RebootForce,
    RebootImmediate,
    Poweroff,
    PoweroffForce,
    PoweroffImmediate,
    Exit,
    ExitForce
}

impl Default for FailureSuccessAction {
    fn default() -> Self { FailureSuccessAction::None }
}

// https://www.freedesktop.org/software/systemd/man/systemd.unit.html#CollectMode=
#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CollectMode {
    Inactive,
    InactiveOrFailed
}

impl Default for CollectMode {
    fn default() -> Self { CollectMode::Inactive }
}

// Used in place of bool where "yes" or "no" is required
#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum YesNo {
    Yes,
    No
}

impl Default for YesNo {
    fn default() -> Self { YesNo::No }
}

// https://www.freedesktop.org/software/systemd/man/systemd.unit.html#OnFailureJobMode=
#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum OnFailureJobMode {
    Fail,
    Replace,
    ReplaceIrreversibly,
    Isolate,
    Flush,
    IgnoreDependencies,
    IgnoreRequirements
}

impl Default for OnFailureJobMode {
    fn default() -> Self { OnFailureJobMode::Replace }
}