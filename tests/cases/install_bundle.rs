use crate::common::mock::Dbus;
use std::collections::HashMap;

use rauc::{InstallBundleArgs, InstallerProxy};
use zbus::zvariant::OwnedValue;

#[tokio::test]
async fn decodes_recorded_install_bundle() {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");
    let client = dbus.client();
    let proxy = InstallerProxy::new(&client)
        .await
        .expect("failed to create installer proxy");

    let (reply, result) = tokio::join!(
        dbus.reply(include_str!("../records/install-bundle.json")),
        proxy.install_bundle("/tmp/test-bundle.raucb", InstallBundleArgs::default()),
    );

    let request = reply.expect("failed to send recorded D-Bus reply");
    let header = request.header();
    assert_eq!(
        header.member().map(|name| name.as_str()),
        Some("InstallBundle")
    );
    assert_eq!(
        header.interface().map(|name| name.as_str()),
        Some("de.pengutronix.rauc.Installer"),
    );
    assert_eq!(header.path().map(|path| path.as_str()), Some("/"));
    let (source, options): (String, HashMap<String, OwnedValue>) = request
        .body()
        .deserialize()
        .expect("failed to decode request");
    assert_eq!(source, "/tmp/test-bundle.raucb");
    assert!(options.is_empty(), "default options should be omitted");

    result.expect("failed to install bundle");
}
