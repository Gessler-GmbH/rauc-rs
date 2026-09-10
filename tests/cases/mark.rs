use crate::common::mock::Dbus;
use rauc::InstallerProxy;

#[tokio::test]
async fn decodes_recorded_mark() {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");
    let client = dbus.client();
    let proxy = InstallerProxy::new(&client)
        .await
        .expect("failed to create installer proxy");

    let (reply, result) = tokio::join!(
        dbus.reply(include_str!("../records/mark.json")),
        proxy.mark("good", "rootfs.1"),
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
    assert_eq!(slot, "rootfs.1");

    let info = result.expect("failed to mark slot");
    assert_eq!(info.slot_name, "rootfs.1");
    assert_eq!(info.message, "marked slot(s) rootfs.1 as good");
}
