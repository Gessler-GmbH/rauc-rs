use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use zbus::zvariant::{DeserializeDict, DeserializeValue, SerializeDict};
use zbus::zvariant::{OwnedValue, Signature, Type};

/// Status of a configured artifact repository.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct ArtifactStatusInfo {
    /// Repository name.
    pub name: String,
    /// Human-readable repository description.
    pub description: Option<String>,
    /// Repository storage path.
    pub path: String,

    /// Repository type.
    #[zvariant(rename = "type")]
    pub r#type: String,

    /// Slot-class to which the repository belongs.
    pub parent_class: Option<String>,
    /// Artifacts stored in the repository.
    pub artifacts: Vec<ArtifactStatusArtifactInfo>,
}

/// Status of an artifact stored in a repository.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct ArtifactStatusArtifactInfo {
    /// Artifact name.
    pub name: String,
    /// Available instances of the artifact.
    pub instances: Vec<ArtifactStatusInstanceInfo>,
}

/// Status of one artifact instance.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct ArtifactStatusInstanceInfo {
    /// Checksum identifying the instance.
    pub checksum: String,
    /// Names that refer to this instance.
    pub references: Vec<String>,
}

/// Runtime state of a slot.
#[derive(Deserialize, Type, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
#[zvariant(signature = "s")]
pub enum SlotState {
    /// The system was booted from this slot.
    Booted,
    /// The slot is active but was not booted.
    Active,
    /// The slot is inactive.
    Inactive,
}

/// Bootloader status of a slot.
#[derive(Deserialize, Type, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
#[zvariant(signature = "s")]
pub enum SlotBootStatus {
    /// The slot is considered bootable.
    Good,
    /// The slot is considered unbootable.
    Bad,
    /// The boot status could not be determined.
    Unknown,
}

/// Result of the most recent installation to a slot.
#[derive(Deserialize, Type, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
#[zvariant(signature = "s")]
pub enum SlotInstallationStatus {
    /// Installation completed successfully.
    Ok,
    /// Installation failed.
    Failed,
    /// The slot is currently being updated.
    Pending,
}

/// Name identifying a RAUC slot, such as `rootfs.0`.
pub type SlotName = String;

/// Status and metadata of a RAUC slot.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct GetSlotStatusInfo {
    /// Slot class, such as `rootfs`.
    pub class: String,
    /// Backing device path.
    pub device: String,

    /// Slot type configured by RAUC.
    #[zvariant(rename = "type")]
    pub r#type: String,

    /// Name used by the bootloader.
    pub bootname: Option<String>,
    /// Current runtime state.
    pub state: Option<SlotState>,
    /// Name of the parent slot.
    pub parent: Option<String>,
    /// Current mount point.
    pub mountpoint: Option<String>,
    /// Status reported by the bootloader.
    pub boot_status: Option<SlotBootStatus>,
    /// Result of the most recent installation.
    pub status: Option<SlotInstallationStatus>,
    /// SHA-256 checksum of the last installed image.
    pub sha256: Option<String>,
    /// Size of the last installed image in bytes.
    pub size: Option<u64>,

    /// ISO 8601 timestamp when the last installation completed.
    #[zvariant(rename = "installed.timestamp")]
    pub installed_timestamp: Option<String>,

    /// Number of images written to the slot.
    #[zvariant(rename = "installed.count")]
    pub installed_count: Option<u32>,

    /// Transaction UUID of the last installation.
    #[zvariant(rename = "installed.transaction")]
    pub installed_transaction: Option<String>,

    /// ISO 8601 timestamp when the slot was last activated.
    #[zvariant(rename = "activated.timestamp")]
    pub activated_timestamp: Option<String>,

    /// Number of times the slot has been activated.
    #[zvariant(rename = "activated.count")]
    pub activated_count: Option<u32>,

    /// Compatibility identifier from the last installed bundle.
    #[zvariant(rename = "bundle.compatible")]
    pub bundle_compatible: Option<String>,

    /// Version from the last installed bundle.
    #[zvariant(rename = "bundle.version")]
    pub bundle_version: Option<String>,

    /// Description from the last installed bundle.
    #[zvariant(rename = "bundle.description")]
    pub bundle_description: Option<String>,

    /// Build identifier from the last installed bundle.
    #[zvariant(rename = "bundle.build")]
    pub bundle_build: Option<String>,

    /// Manifest hash of the last installed bundle.
    #[zvariant(rename = "bundle.hash")]
    pub bundle_hash: Option<String>,
}

/// Options for inspecting a local or remote bundle.
#[derive(SerializeDict, DeserializeDict, Type, PartialEq, Debug, Default)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleArgs {
    /// Certificate file or PKCS#11 URL used for TLS client authentication.
    pub tls_cert: Option<String>,
    /// Private-key file or PKCS#11 URL used for TLS client authentication.
    pub tls_key: Option<String>,
    /// Certificate file or PKCS#11 URL used instead of the system trust store.
    pub tls_ca: Option<String>,
    /// Additional headers sent with every HTTP request.
    pub http_headers: Option<Vec<String>>,
    /// Whether to ignore server certificate verification errors.
    pub tls_no_verify: Option<bool>,
}

/// Metadata read from an inspected bundle.
///
/// RAUC's variant wrappers are consumed during deserialization, exposing only
/// the typed metadata to callers.
#[derive(Deserialize, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}")]
#[serde(rename_all = "kebab-case")]
pub struct InspectBundleInfo {
    /// SHA-256 hash of the bundle manifest.
    pub manifest_hash: String,
    /// Bundle update metadata.
    #[serde(deserialize_with = "decode_variant_layers")]
    pub update: InspectBundleUpdateInfo,
    /// Bundle format and integrity metadata.
    #[serde(deserialize_with = "decode_variant_layers")]
    pub bundle: InspectBundleBundleInfo,
    /// Hooks declared by the bundle.
    #[serde(default, deserialize_with = "decode_optional_variant_layers")]
    pub hooks: Option<InspectBundleHooksInfo>,
    /// Custom handler declared by the bundle.
    #[serde(default, deserialize_with = "decode_optional_variant_layers")]
    pub handler: Option<InspectBundleHandlerInfo>,
    /// Images contained in the bundle.
    #[serde(deserialize_with = "decode_variant_layers")]
    pub images: Vec<InspectBundleImageInfo>,
    /// Custom manifest metadata grouped by section.
    #[serde(deserialize_with = "decode_variant_layers")]
    pub meta: HashMap<String, HashMap<String, String>>,
}

/// Update metadata from a bundle manifest.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleUpdateInfo {
    /// System compatibility identifier from the manifest.
    pub compatible: String,
    /// Bundle version.
    pub version: Option<String>,
    /// Human-readable bundle description.
    pub description: Option<String>,
    /// Bundle build identifier.
    pub build: Option<String>,
}

/// Bundle format and integrity metadata.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleBundleInfo {
    /// Bundle format.
    pub format: String,
    /// Size of the verity-protected payload in bytes.
    pub verity_size: Option<u64>,
    /// Salt used by the verity-protected payload.
    pub verity_salt: Option<String>,
    /// Root hash of the verity-protected payload.
    pub verity_hash: Option<String>,
}

/// Hook metadata from a bundle manifest.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleHooksInfo {
    /// Name of the hook executable.
    pub filename: String,
    /// Hooks enabled for the bundle.
    pub hooks: Vec<String>,
}

/// Custom handler metadata from a bundle manifest.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleHandlerInfo {
    /// Name of the custom handler executable.
    pub filename: String,
    /// Arguments passed to the custom handler.
    pub args: Option<String>,
}

/// Metadata for an image contained in a bundle.
#[derive(DeserializeDict, Type, PartialEq, Debug)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InspectBundleImageInfo {
    /// Target slot class.
    pub slot_class: String,
    /// Target system variant.
    pub variant: Option<String>,
    /// Image filename within the bundle.
    pub filename: Option<String>,

    /// Image type.
    #[zvariant(rename = "type")]
    pub r#type: Option<String>,

    /// SHA-256 checksum of the original image.
    pub checksum: Option<String>,
    /// Image size in bytes.
    pub size: Option<u64>,
    /// Hooks enabled for the image.
    pub hooks: Vec<String>,
    /// Adaptive update methods enabled for the image.
    pub adaptive: Option<Vec<String>>,
}

/// Options for installing a local or remote bundle.
#[derive(SerializeDict, DeserializeDict, Type, PartialEq, Debug, Default)]
#[zvariant(signature = "a{sv}", rename_all = "kebab-case")]
pub struct InstallBundleArgs {
    /// Whether to ignore a mismatch with the system compatibility identifier.
    pub ignore_compatible: Option<bool>,
    /// Whether to ignore the configured minimum bundle version.
    pub ignore_version_limit: Option<bool>,
    /// UUID identifying the installation transaction.
    pub transaction_id: Option<String>,
    /// Expected manifest hash; installation stops if it does not match.
    pub require_manifest_hash: Option<String>,
    /// Certificate file or PKCS#11 URL used for TLS client authentication.
    pub tls_cert: Option<String>,
    /// Private-key file or PKCS#11 URL used for TLS client authentication.
    pub tls_key: Option<String>,
    /// Certificate file or PKCS#11 URL used instead of the system trust store.
    pub tls_ca: Option<String>,
    /// Additional headers sent with every HTTP request.
    pub http_headers: Option<Vec<String>>,
    /// Whether to ignore server certificate verification errors.
    pub tls_no_verify: Option<bool>,
}

/// Result of marking a slot as good, bad, or active.
#[derive(OwnedValue, Type, Deserialize, Debug)]
pub struct MarkInfo {
    /// Name of the slot that was marked.
    pub slot_name: String,
    /// Human-readable description of the completed action.
    pub message: String,
}

/// Operation currently performed by RAUC.
#[derive(OwnedValue, Type, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[zvariant(signature = "s")]
pub enum Operation {
    /// RAUC is not performing an operation.
    Idle,
    /// RAUC is installing a bundle.
    Installing,
}

/// Progress of the current installation operation.
#[derive(OwnedValue, Deserialize, Debug)]
pub struct Progress {
    /// Completion percentage from 0 to 100.
    pub percentage: i32,
    /// Human-readable description of the current step.
    pub message: String,
    /// Nesting level of the current step.
    pub nesting_depth: i32,
}

/// One explicit D-Bus variant layer containing `T`.
///
/// Consumes the outer signature and payload explicitly, leaving the inner
/// variant to `DeserializeValue` for unwrapping and signature validation.
#[derive(Deserialize)]
struct Variant<T> {
    #[allow(dead_code)]
    signature: Signature,
    value: T,
}

// Keep RAUC's two variant layers out of the public metadata API.
fn decode_variant_layers<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Type + 'de,
{
    let wrapped = Variant::<DeserializeValue<'de, T>>::deserialize(deserializer)?;
    Ok(wrapped.value.0)
}

// Decode both variant layers into Some(T). Missing keys are handled as None.
fn decode_optional_variant_layers<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Type + 'de,
{
    decode_variant_layers(deserializer).map(Some)
}
