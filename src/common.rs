use std::time::Duration;
use serde::{Serialize, Serializer};

// https://www.freedesktop.org/software/systemd/man/systemd.resource-control.html#CPUWeight=weight
// https://www.freedesktop.org/software/systemd/man/systemd.resource-control.html#IOWeight=weight
// TODO: provide bound checking from 1 to 10000
pub type Weight = u16;

// https://www.freedesktop.org/software/systemd/man/systemd.time.html
pub enum Timespan {
    Duration(Duration),
    Infinity
}

impl Serialize for Timespan {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = match self {
            Timespan::Duration(d) => {
                // TODO: make this use bigger units where relevant
                d.as_secs().to_string()
            },
            Timespan::Infinity => String::from("infinity")
        };
        serializer.serialize_str(&string)
    }
}

// TODO: change this to something more strictly checked?
pub type UnitReference = &'static str;

// Do a unit struct so that we can serialize it as a space separated string
pub struct UnitReferenceList (pub Vec<UnitReference>);

impl Serialize for UnitReferenceList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.join(" "))
    }
}