use crate::common::mock::Dbus;
use rauc::InstallerProxy;

#[tokio::test]
async fn decodes_recorded_get_primary() {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");
    let client = dbus.client();
    let proxy = InstallerProxy::new(&client)
        .await
        .expect("failed to create installer proxy");

    let (reply, result) = tokio::join!(
        dbus.reply(include_str!("../records/get-primary.json")),
        proxy.get_primary(),
    );

    let request = reply.expect("failed to send recorded D-Bus reply");
    let header = request.header();
    assert_eq!(
        header.member().map(|name| name.as_str()),
        Some("GetPrimary")
    );
    assert_eq!(
        header.interface().map(|name| name.as_str()),
        Some("de.pengutronix.rauc.Installer"),
    );
    assert_eq!(header.path().map(|path| path.as_str()), Some("/"));
    request
        .body()
        .deserialize::<()>()
        .expect("expected no arguments");
    assert_eq!(result.expect("failed to get primary"), "rootfs.1");
}
