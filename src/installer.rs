use zbus::{Result, proxy};

pub mod types;
use types::*;

#[proxy(
    interface = "de.pengutronix.rauc.Installer",
    default_service = "de.pengutronix.rauc",
    default_path = "/"
)]
pub trait Installer {
    /// GetArtifactStatus method
    fn get_artifact_status(&self) -> Result<Vec<ArtifactStatusInfo>>;

    /// GetPrimary method
    fn get_primary(&self) -> Result<String>;

    /// GetSlotStatus method
    fn get_slot_status(&self) -> Result<Vec<(SlotName, GetSlotStatusInfo)>>;

    /// InspectBundle method
    fn inspect_bundle(&self, source: &str, args: InspectBundleArgs) -> Result<InspectBundleInfo>;

    /// InstallBundle method
    fn install_bundle(&self, source: &str, args: InstallBundleArgs) -> Result<()>;

    /// Mark method
    fn mark(&self, state: &str, slot_identifier: &str) -> Result<MarkInfo>;

    /// Completed signal
    #[zbus(signal)]
    fn completed(&self, result: i32) -> Result<()>;

    /// BootSlot property
    #[zbus(property)]
    fn boot_slot(&self) -> Result<String>;

    /// Compatible property
    #[zbus(property)]
    fn compatible(&self) -> Result<String>;

    /// LastError property
    #[zbus(property)]
    fn last_error(&self) -> Result<String>;

    /// Operation property
    #[zbus(property)]
    fn operation(&self) -> Result<String>;

    /// Progress property
    #[zbus(property)]
    fn progress(&self) -> Result<Progress>;

    /// Variant property
    #[zbus(property)]
    fn variant(&self) -> Result<String>;
}
