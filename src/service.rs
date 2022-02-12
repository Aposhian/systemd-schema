use crate::common::Timespan;

use serde::{Serialize, Serializer};
use serde_with::skip_serializing_none;

// TODO: use path type with args?
type Command = &'static str;

#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceSection {
    #[serde(rename = "Type")]
    pub service_type: ServiceType,
    pub exec_start: Command,
    pub exec_start_pre: Option<Vec<ExecExtra>>,
    pub exec_start_post: Option<Vec<ExecExtra>>,
    pub exec_reload: Option<Command>,
    pub exec_stop: Option<Command>,
    pub restart_sec: Option<Timespan>,
    pub restart: Option<Restart>,
    pub timeout_sec: Option<u32>
}

pub struct ExecExtra {
    fallible: bool,
    command: Command
}

impl Serialize for ExecExtra {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if self.fallible {
            let string = format!("-{}", self.command);
            serializer.serialize_str(&string)
        } else {
            serializer.serialize_str(self.command)
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum NotifyAccess {
    None,
    Main,
    All
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
// TODO: add type specific config
pub enum ServiceType {
    Simple,
    Forking,
    Oneshot,
    Dbus,
    Notify,
    Idle
}

impl Default for ServiceType {
    fn default() -> Self {
        ServiceType::Simple
    }
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Restart {
    Always,
    OnSuccess,
    OnFailure,
    OnAbnormal,
    OnAbort,
    OnWatchdog
}