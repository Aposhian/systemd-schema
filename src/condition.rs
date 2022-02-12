use serde::{Serialize, Serializer};

// https://www.freedesktop.org/software/systemd/man/systemd.unit.html#Conditions%20and%20Asserts
pub struct Condition {
    /// "If multiple conditions are specified,
    /// the unit will be executed if all of them apply (i.e. a logical AND is applied).
    /// Condition checks can use a pipe symbol ("|") after the equals sign ("Condition…=|…"),
    /// which causes the condition to become a triggering condition.
    /// If at least one triggering condition is defined for a unit,
    /// then the unit will be started if at least one of the triggering conditions of the unit applies
    /// and all of the regular (i.e. non-triggering) conditions apply."
    pub triggering: bool,
    /// "Before the unit is started, systemd will verify that the specified conditions and asserts are true.
    /// If not, the starting of the unit will be (mostly silently) skipped (in case of conditions),
    /// or aborted with an error message (in case of asserts)."
    pub assert: bool,
    pub condition: ConditionType
}

impl Default for Condition {
    fn default() -> Self {
        Condition {
            triggering: false,
            assert: false,
            ..Default::default()
        }
    }
}


// TODO: add condition config
pub enum ConditionType {
    Architecture,
    Firmware,
    Virtualization,
    Host,
    KernelCommandLine,
    KernelVersion,
    Environment,
    Security,
    Capability,
    ACPower,
    NeedsUpdate,
    FirstBoot,
    PathExists,
    PathExistsGlob,
    PathIsDirectory,
    PathIsSymbolicLink,
    PathIsMountPoint,
    PathIsReadWrite,
    PathIsEncrypted,
    DirectoryNotEmpty,
    FileNotEmpty,
    FileIsExecutable,
    User,
    Group,
    ControlGroupController,
    Memory,
    CPUs,
    CPUFeature,
    OSRelease,
    MemoryPressure,
    CPUPressure,
    IOPressure
}

// https://www.freedesktop.org/software/systemd/man/systemd.unit.html#ConditionArchitecture=
#[derive(Serialize)]
enum Architecture {
    #[serde(rename = "x86")]
    X86,
    #[serde(rename = "x86-64")]
    X86_64,
    #[serde(rename = "ppc")]
    Ppc,
    #[serde(rename = "ppc-le")]
    PpcLe,
    #[serde(rename = "ppc64")]
    Ppc64,
    #[serde(rename = "ppc64-le")]
    Ppc64Le,
    #[serde(rename = "ia64")]
    Ia64,
    #[serde(rename = "parisc")]
    Parisc,
    #[serde(rename = "parisc64")]
    Parisc64,
    #[serde(rename = "s390")]
    S390,
    #[serde(rename = "s390x")]
    S390x,
    #[serde(rename = "sparc")]
    Sparc,
    #[serde(rename = "sparc64")]
    Sparc64,
    #[serde(rename = "mips")]
    Mips,
    #[serde(rename = "mips-le")]
    MipsLe,
    #[serde(rename = "mips64")]
    Mips64,
    #[serde(rename = "mips64-le")]
    Mips64Le,
    #[serde(rename = "alpha")]
    Alpha,
    #[serde(rename = "arm")]
    Arm,
    #[serde(rename = "arm-be")]
    ArmBe,
    #[serde(rename = "arm64")]
    Arm64,
    #[serde(rename = "arm64-be")]
    Arm64Be,
    #[serde(rename = "sh")]
    Sh,
    #[serde(rename = "sh64")]
    Sh64,
    #[serde(rename = "m68k")]
    M68k,
    #[serde(rename = "tilegx")]
    Tilegx,
    #[serde(rename = "cris")]
    Cris,
    #[serde(rename = "arc")]
    Arc,
    #[serde(rename = "arc-be")]
    ArcBe,
    #[serde(rename = "native")]
    Native
}
