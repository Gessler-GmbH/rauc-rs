use zbus::{Result, proxy};

pub mod types;
use types::*;

#[proxy(
    interface = "de.pengutronix.rauc.Installer",
    default_service = "de.pengutronix.rauc",
    default_path = "/"
)]
pub trait Installer {
    /// Returns status information for all configured artifacts and repositories.
    fn get_artifact_status(&self) -> Result<Vec<ArtifactStatusInfo>>;

    /// Returns the primary boot slot reported by the bootloader.
    fn get_primary(&self) -> Result<String>;

    /// Returns the status of every system slot.
    fn get_slot_status(&self) -> Result<Vec<(SlotName, GetSlotStatusInfo)>>;

    /// Inspects a bundle at the given path or URL without installing it.
    fn inspect_bundle(&self, source: &str, args: InspectBundleArgs) -> Result<InspectBundleInfo>;

    /// Starts installing a bundle from the given path or URL.
    fn install_bundle(&self, source: &str, args: InstallBundleArgs) -> Result<()>;

    /// Marks a slot as good, bad, or active and returns the affected slot.
    fn mark(&self, state: &str, slot_identifier: &str) -> Result<MarkInfo>;

    /// Signals that an installation finished, with zero indicating success.
    #[zbus(signal)]
    fn completed(&self, result: i32) -> Result<()>;

    /// Returns the slot from which the system was booted.
    #[zbus(property)]
    fn boot_slot(&self) -> Result<String>;

    /// Returns the system compatibility identifier.
    #[zbus(property)]
    fn compatible(&self) -> Result<String>;

    /// Returns a message describing the most recent error.
    #[zbus(property)]
    fn last_error(&self) -> Result<String>;

    /// Returns the operation RAUC is currently performing.
    #[zbus(property)]
    fn operation(&self) -> Result<String>;

    /// Returns the current installation percentage, message, and nesting depth.
    #[zbus(property)]
    fn progress(&self) -> Result<Progress>;

    /// Returns the system variant identifier.
    #[zbus(property)]
    fn variant(&self) -> Result<String>;
}
