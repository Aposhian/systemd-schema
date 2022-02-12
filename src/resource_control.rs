use crate::common::{Timespan, Weight};
use std::net::IpAddr;
use std::path::Path;
use bytesize::ByteSize;
use serde::{Serialize, Serializer};
use serde_with::skip_serializing_none;

// https://www.freedesktop.org/software/systemd/man/systemd.resource-control.html
#[skip_serializing_none]
#[derive(Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceControl {
    #[serde(rename = "CPUAccounting")]
    cpu_accounting: Option<bool>,
    #[serde(rename = "CPUWeight")]
    cpu_weight: Option<Weight>,
    #[serde(rename = "StartupCPUWeight")]
    startup_cpu_weight: Option<Weight>,
    #[serde(rename = "CPUQuota")]
    cpu_quota: Option<Percentage>,
    #[serde(rename = "CPUQuotaPeriodSec")]
    cpu_quota_period_sec: Option<Timespan>,
    #[serde(rename = "AllowedCPUs")]
    allowed_cpus: Option<Vec<AllowedIndices>>,
    #[serde(rename = "StartupAllowedCPUs")]
    startup_allowed_cpus: Option<Vec<AllowedIndices>>,
    allowed_memory_nodes: Option<Vec<AllowedIndices>>,
    startup_allowed_memory_nodes: Option<Vec<AllowedIndices>>,
    memory_accounting: Option<bool>,
    memory_min: Option<ByteSize>,
    memory_low: Option<ByteSize>,
    memory_max: Option<ByteSize>,
    memory_high: Option<ByteSize>,
    memory_swap_max: Option<ByteSize>,
    tasks_accounting: Option<bool>,
    tasks_max: Option<TasksMax>,
    #[serde(rename = "IOAccounting")]
    io_accounting: Option<bool>,
    #[serde(rename = "IOWeight")]
    io_weight: Option<Weight>,
    #[serde(rename = "StartupIOWeight")]
    startup_io_weight: Option<Weight>,
    // #[serde(rename = "IODeviceWeight")]
    // io_device_weight: Option<Vec<DeviceWeight>>,
    // #[serde(rename = "IOReadBandwidthMax")]
    // io_read_bandwidth_max: Option<Vec<DeviceWeight>>,
    // #[serde(rename = "IOWriteBandwidthMax")]
    // io_write_bandwidth_max: Option<Vec<DeviceWeight>>,
    // #[serde(rename = "IOReadIOPSMax")]
    // io_read_iops_max: Option<Vec<DeviceIOPS>>,
    // #[serde(rename = "IOWriteIOPSMax")]
    // io_write_iops_max: Option<Vec<DeviceIOPS>>,
    // #[serde(rename = "IODeviceLatencyTargetSec")]
    // io_device_latency_target_sec: Option<Vec<LatencyTarget>>,
    // #[serde(rename = "IPAccounting")]
    // ip_accounting: Option<bool>,
    // #[serde(rename = "IPAddressAllow")]
    // ip_address_allow: Option<Vec<AddressFilter>>,
    // #[serde(rename = "IPAddressDeny")]
    // ip_address_deny: Option<Vec<AddressFilter>>,
    // TODO: add all the other ones
}

type Device = &'static Path;

struct DeviceWeight {
    device: Device,
    weight: Weight
}

struct DeviceIOPS {
    device: Device,
    // TODO: use a type that makes it easy to use K, M, G, T, etc.
    iops: &'static str
}

struct LatencyTarget {
    device: Device,
    target: Timespan
}

struct AddressFilter {
    address: IpAddr,
    prefix_length: u8
}

#[derive(Serialize)]
#[serde(untagged)]
enum TasksMax {
    Number(u32),
    #[serde(rename = "infinity")]
    Infinity
}

// https://www.freedesktop.org/software/systemd/man/systemd.resource-control.html#AllowedCPUs=
enum AllowedIndices {
    Index(u8),
    Range(std::ops::RangeInclusive<u8>)
}

impl Serialize for AllowedIndices {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let string = match self {
            AllowedIndices::Index(i) => i.to_string(),
            AllowedIndices::Range(r) => format!("{}-{}", r.start(), r.end())
        };
        serializer.serialize_str(&string)
    }
}

// TODO: use this crate? https://docs.rs/percentage/latest/percentage/
type Percentage = f32;

