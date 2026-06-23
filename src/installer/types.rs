use std::collections::HashMap;

use serde::Deserialize;

use zbus::zvariant::as_value;
use zbus::zvariant::{DeserializeDict, SerializeDict};
use zbus::zvariant::{OwnedValue, Type};

/// Custom type for the GetArtifactStatus method.
/// Check https://github.com/rauc/rauc/blob/master/src/artifacts.c for reference.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct ArtifactStatusInfo {
    pub name: String,
    pub description: Option<String>,
    pub path: Option<String>,

    #[zvariant(rename = "type")]
    pub r#type: Option<String>,

    pub parent_class: Option<String>,
    pub artifacts: Vec<ArtifactStatusArtifactInfo>,
}

#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct ArtifactStatusArtifactInfo {
    pub name: String,
    pub instances: Vec<ArtifactStatusInstanceInfo>,
}

#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct ArtifactStatusInstanceInfo {
    pub checksum: String,
    pub references: String,
}

#[derive(Deserialize, Type, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
#[zvariant(signature = "s")]
pub enum SlotState {
    Booted,
    Active,
    Inactive,
}

#[derive(Deserialize, Type, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
#[zvariant(signature = "s")]
pub enum SlotBootStatus {
    Good,
    Bad,
    Unknown,
}

#[derive(Deserialize, Type, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
#[zvariant(signature = "s")]
pub enum SlotInstallationStatus {
    Ok,
    Failed,
    Pending,
}

/// Custom types for the GetSlotStatus method.
pub type SlotName = String;

#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct GetSlotStatusInfo {
    pub class: Option<String>,
    pub device: Option<String>,

    #[zvariant(rename = "type")]
    pub r#type: Option<String>,

    pub bootname: Option<String>,
    pub state: Option<SlotState>,
    pub parent: Option<String>,
    pub mountpoint: Option<String>,
    pub boot_status: Option<SlotBootStatus>,
    pub status: Option<SlotInstallationStatus>,
    pub sha256: Option<String>,
    pub size: Option<u64>,

    #[zvariant(rename = "installed.timestamp")]
    pub installed_timestamp: Option<String>,

    #[zvariant(rename = "installed.count")]
    pub installed_count: Option<u32>,

    #[zvariant(rename = "installed.transaction")]
    pub installed_transaction: Option<String>,

    #[zvariant(rename = "activated.timestamp")]
    pub activated_timestamp: Option<String>,

    #[zvariant(rename = "activated.count")]
    pub activated_count: Option<u32>, // D-Bus 'u'

    #[zvariant(rename = "bundle.compatible")]
    pub bundle_compatible: Option<String>,

    #[zvariant(rename = "bundle.version")]
    pub bundle_version: Option<String>,

    #[zvariant(rename = "bundle.description")]
    pub bundle_description: Option<String>,

    #[zvariant(rename = "bundle.build")]
    pub bundle_build: Option<String>,

    #[zvariant(rename = "bundle.hash")]
    pub bundle_hash: Option<String>,
}

/// Custom types for the InspectBundle method.
#[derive(SerializeDict, DeserializeDict, Type, PartialEq, Debug, Default)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleArgs {
    pub tls_cert: Option<String>,
    pub tls_key: Option<String>,
    pub tls_ca: Option<String>,
    pub http_headers: Option<Vec<String>>,
    pub tls_no_verify: Option<bool>,
}

/// Represents a D-Bus `variant` (`v`) containing any value.
#[derive(Deserialize, Type, PartialEq, Debug)]
#[serde(bound(deserialize = "T: Deserialize<'de> + Type + 'de",))]
#[zvariant(signature = "v")]
pub struct Variant<T>(#[serde(with = "as_value")] pub T);

/// NOTE: RAUC returns each dict field wrapped in a D-Bus variant (`v`).
/// We keep this representation to match the RAUC D-Bus format.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleInfo {
    pub manifest_hash: String,
    pub update: Variant<InspectBundleUpdateInfo>,
    pub bundle: Option<Variant<InspectBundleBundleInfo>>,
    pub hooks: Option<Variant<InspectBundleHooksInfo>>,
    pub handler: Option<Variant<InspectBundleHandlerInfo>>,
    pub images: Option<Variant<Vec<InspectBundleImageInfo>>>,
    pub meta: Option<Variant<HashMap<String, HashMap<String, String>>>>,
}

#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleUpdateInfo {
    pub compatible: String,
    pub version: String,
    pub description: Option<String>,
    pub build: Option<String>,
}

#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleBundleInfo {
    pub format: Option<String>,
    pub verity_size: Option<u64>,
    pub verity_salt: Option<String>,
    pub verity_hash: Option<String>,
}

#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleHooksInfo {
    pub filename: Option<String>,
    pub hooks: Option<Vec<String>>,
}

#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleHandlerInfo {
    pub filename: Option<String>,
    pub args: Option<String>,
}

#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleImageInfo {
    pub slot_class: Option<String>,
    pub variant: Option<String>,
    pub filename: Option<String>,

    #[zvariant(rename = "type")]
    pub r#type: Option<String>,

    pub checksum: Option<String>,
    pub size: Option<u64>,
    pub hooks: Option<Vec<String>>,
    pub adaptive: Option<Vec<String>>,
}

/// Custom type for the InstallBundle method.
#[derive(SerializeDict, DeserializeDict, Type, PartialEq, Debug, Default)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InstallBundleArgs {
    pub ignore_compatible: Option<bool>,
    pub ignore_version_limit: Option<bool>,
    pub transaction_id: Option<String>,
    pub require_manifest_hash: Option<String>,
    pub tls_cert: Option<String>,
    pub tls_key: Option<String>,
    pub tls_ca: Option<String>,
    pub http_headers: Option<Vec<String>>,
    pub tls_no_verify: Option<bool>,
}

/// Custom type for the Mark method.
#[derive(OwnedValue, Type, Deserialize, Debug)]
pub struct MarkInfo {
    pub slot_name: String,
    pub message: String,
}

#[derive(OwnedValue, Type, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[zvariant(signature = "s")]
pub enum Operation {
    Idle,
    Installing,
}

/// Custom type for the Progress property.
#[derive(OwnedValue, Deserialize, Debug)]
pub struct Progress {
    pub percentage: i32,
    pub message: String,
    pub nesting_depth: i32,
}
