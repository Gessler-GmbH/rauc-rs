use crate::common::mock::Dbus;
use rauc::{InstallerProxy, MarkState, SlotIdentifier};

#[tokio::test]
async fn decodes_recorded_mark() {
    check_mark(SlotIdentifier::Named("rootfs.1".into()), "rootfs.1").await;
}

#[tokio::test]
async fn serializes_booted_selector() {
    check_mark(SlotIdentifier::Booted, "booted").await;
}

#[tokio::test]
async fn serializes_other_selector() {
    check_mark(SlotIdentifier::Other, "other").await;
}

async fn check_mark(identifier: SlotIdentifier, expected_slot: &str) {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");
    let client = dbus.client();
    let proxy = InstallerProxy::new(&client)
        .await
        .expect("failed to create installer proxy");

    let (reply, result) = tokio::join!(
        dbus.reply(include_str!("../records/mark.json")),
        proxy.mark(MarkState::Good, &identifier),
    );

    let request = reply.expect("failed to send recorded D-Bus reply");
    let header = request.header();
    assert_eq!(header.member().map(|name| name.as_str()), Some("Mark"));
    assert_eq!(
        header.interface().map(|name| name.as_str()),
        Some("de.pengutronix.rauc.Installer"),
    );
    assert_eq!(header.path().map(|path| path.as_str()), Some("/"));
    let (state, slot): (String, String) = request
        .body()
        .deserialize()
        .expect("failed to decode request");
    assert_eq!(state, "good");
    assert_eq!(slot, expected_slot);

    let info = result.expect("failed to mark slot");
    assert_eq!(info.slot_name, "rootfs.1");
    assert_eq!(info.message, "marked slot(s) rootfs.1 as good");
}
