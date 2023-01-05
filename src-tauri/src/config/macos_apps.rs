use serde;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MacApps {
    #[serde(rename = "SPApplicationsDataType")]
    pub sp_applications_data_type: Vec<SpApplicationsDataType>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SpApplicationsDataType {
    #[serde(rename = "_name")]
    pub name: String,

    #[serde(rename = "arch_kind")]
    arch_kind: ArchKind,

    #[serde(rename = "lastModified")]
    last_modified: String,

    #[serde(rename = "obtained_from")]
    obtained_from: ObtainedFrom,

    #[serde(rename = "path")]
    pub path: String,

    #[serde(rename = "signed_by")]
    signed_by: Option<Vec<String>>,

    #[serde(rename = "version")]
    version: Option<String>,

    #[serde(rename = "info")]
    info: Option<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ArchKind {
    #[serde(rename = "arch_arm")]
    ArchArm,

    #[serde(rename = "arch_arm_i64")]
    ArchArmI64,

    #[serde(rename = "arch_i64")]
    ArchI64,

    #[serde(rename = "arch_other")]
    ArchOther,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ObtainedFrom {
    #[serde(rename = "apple")]
    Apple,

    #[serde(rename = "identified_developer")]
    IdentifiedDeveloper,

    #[serde(rename = "mac_app_store")]
    MacAppStore,

    #[serde(rename = "unknown")]
    Unknown,
}
