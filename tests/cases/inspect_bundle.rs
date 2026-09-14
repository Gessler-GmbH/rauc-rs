use std::collections::HashMap;

use crate::common::mock::Dbus;
use rauc::{BundleFormat, InspectBundleArgs, InstallerProxy};
use zbus::zvariant::OwnedValue;

#[test]
fn decodes_bundle_formats() {
    for (name, expected) in [
        ("plain", BundleFormat::Plain),
        ("verity", BundleFormat::Verity),
        ("crypt", BundleFormat::Crypt),
    ] {
        let actual: BundleFormat = serde_json::from_value(serde_json::json!(name))
            .expect("failed to decode bundle format");
        assert_eq!(actual, expected);
    }
}

#[tokio::test]
async fn decodes_recorded_inspect_bundle() {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");
    let client = dbus.client();
    let proxy = InstallerProxy::new(&client)
        .await
        .expect("failed to create installer proxy");

    let source = "/tmp/test-bundle.raucb";
    let (reply, info) = tokio::join!(
        dbus.reply(include_str!("../records/inspect-bundle.json")),
        proxy.inspect_bundle(source, InspectBundleArgs::default()),
    );

    let request = reply.expect("failed to send recorded D-Bus reply");
    let header = request.header();
    assert_eq!(
        header.member().map(|name| name.as_str()),
        Some("InspectBundle")
    );
    assert_eq!(
        header.interface().map(|name| name.as_str()),
        Some("de.pengutronix.rauc.Installer"),
    );
    assert_eq!(header.path().map(|path| path.as_str()), Some("/"));
    let (requested_source, options): (String, HashMap<String, OwnedValue>) = request
        .body()
        .deserialize()
        .expect("failed to decode request");
    assert_eq!(requested_source, source);
    assert!(options.is_empty(), "default options should be omitted");

    let info = info.expect("failed to inspect bundle");
    assert_eq!(
        info.manifest_hash,
        "0becf71e55f5fac7a4d5cf07a8dc05f7622f70617c9d6aae47d936586e23ae5a",
    );
    assert_eq!(info.update.compatible, "raspberrypi5");
    assert_eq!(info.update.version.as_deref(), Some("v20200703"));
    assert_eq!(info.update.description.as_deref(), Some("RAUC Demo Bundle"));
    assert_eq!(info.update.build.as_deref(), Some("20260624062251"));
    assert_eq!(info.bundle.format, BundleFormat::Verity);
    assert_eq!(info.bundle.verity_size, Some(446_464));
    assert_eq!(
        info.bundle.verity_hash.as_deref(),
        Some("977963773e7a13d405b020bf5059b00bfa7a5b67f37860e262192fbbf7b873f5"),
    );
    assert_eq!(
        info.bundle.verity_salt.as_deref(),
        Some("4ceca11a5d984d90a259c7a66d0eb9e81806016423ee42143bb03173a3fdfcfa"),
    );
    assert!(info.hooks.is_none());
    assert!(info.handler.is_none());
    assert!(info.meta.is_empty());

    assert_eq!(info.images.len(), 1);
    let image = &info.images[0];
    assert_eq!(image.slot_class, "rootfs");
    assert_eq!(
        image.filename.as_deref(),
        Some("core-image-minimal-raspberrypi5.rootfs.ext4"),
    );
    assert_eq!(image.r#type.as_deref(), Some("ext4"));
    assert_eq!(image.size, Some(205_520_896));
    assert_eq!(
        image.checksum.as_deref(),
        Some("2e9ad3d9ce442f4fc84035d7a74139005178e879c7bb96c10213bd98b3fb3484"),
    );
    assert!(image.hooks.is_empty());
    assert!(image.variant.is_none());
    assert!(image.adaptive.is_none());
}
