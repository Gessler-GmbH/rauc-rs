use crate::common::mock::Dbus;
use rauc::{InstallerProxy, SlotBootStatus, SlotInstallationStatus, SlotState};

#[tokio::test]
async fn decodes_recorded_get_slot_status() {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");

    let client = dbus.client();

    let proxy = InstallerProxy::new(&client)
        .await
        .expect("failed to create installer proxy");

    let (reply, status) = tokio::join!(
        dbus.reply(include_str!("../records/get-slot-status.json")),
        proxy.get_slot_status(),
    );

    reply.expect("failed to send recorded D-Bus reply");
    let status = status.expect("failed to get slot status");

    assert_eq!(status.len(), 2);

    let inactive = &status
        .iter()
        .find(|(name, _)| name == "rootfs.1")
        .expect("missing rootfs.1")
        .1;
    assert_eq!(inactive.class, "rootfs");
    assert_eq!(inactive.device, "/dev/mmcblk0p3");
    assert_eq!(inactive.r#type, "ext4");
    assert_eq!(inactive.bootname.as_deref(), Some("B"));
    assert_eq!(inactive.state, Some(SlotState::Inactive));
    assert_eq!(inactive.boot_status, Some(SlotBootStatus::Good));
    assert_eq!(inactive.status, Some(SlotInstallationStatus::Ok));
    assert_eq!(inactive.size, Some(205_520_896));
    assert_eq!(inactive.installed_count, Some(6));
    assert_eq!(inactive.bundle_compatible.as_deref(), Some("raspberrypi5"));

    let booted = &status
        .iter()
        .find(|(name, _)| name == "rootfs.0")
        .expect("missing rootfs.0")
        .1;
    assert_eq!(booted.class, "rootfs");
    assert_eq!(booted.device, "/dev/mmcblk0p2");
    assert_eq!(booted.r#type, "ext4");
    assert_eq!(booted.bootname.as_deref(), Some("A"));
    assert_eq!(booted.state, Some(SlotState::Booted));
    assert_eq!(booted.boot_status, Some(SlotBootStatus::Good));
    assert_eq!(booted.mountpoint.as_deref(), Some("/"));
    assert_eq!(booted.activated_count, Some(1));
    assert_eq!(booted.status, None);
    assert_eq!(booted.size, None);
}
