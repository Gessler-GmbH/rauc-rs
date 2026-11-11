use std::collections::HashMap;

use crate::common::mock::Dbus;
use rauc::{BundleFormat, InspectBundleArgs, InstallerProxy};
use zbus::OwnedVariant;

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
    let (requested_source, options): (String, HashMap<String, OwnedVariant>) = request
        .body()
        .deserialize()
        .expect("failed to decode request");
    assert_eq!(requested_source, source);
    assert!(options.is_empty(), "default options should be omitted");

    let info = info.expect("failed to inspect bundle");
    assert_eq!(
        info.manifest_hash,
        "8e4bf5602a6c6939285adca7e611140c8608fc6589d8ba7d210496ffbdafa754",
    );
    assert_eq!(info.update.compatible, "raspberrypi5");
    assert_eq!(info.update.version.as_deref(), Some("v20200703"));
    assert_eq!(info.update.description.as_deref(), Some("RAUC Demo Bundle"));
    assert_eq!(info.update.build.as_deref(), Some("20261109160215"));
    assert_eq!(info.bundle.format, BundleFormat::Verity);
    assert_eq!(info.bundle.verity_size, Some(446_464));
    assert_eq!(
        info.bundle.verity_hash.as_deref(),
        Some("92ea6486317ea5ca78caed37999f2d49601859c87afb5c3999d5a5a566636009"),
    );
    assert_eq!(
        info.bundle.verity_salt.as_deref(),
        Some("e64dd557916a3ec8baa7389fe74f8bc2cb672c99be4c487bed4ccd7c2037f7c0"),
    );
    let hooks = info.hooks.as_ref().expect("bundle hooks missing");
    assert_eq!(hooks.filename, "hook.sh");
    assert!(hooks.hooks.is_empty());
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
        Some("b07599bae1f745420b847b528ddc37050b99884ee419b626f8e9a08a2f5c1fd4"),
    );
    assert_eq!(image.hooks, ["post-install"]);
    assert!(image.variant.is_none());
    assert!(image.adaptive.is_none());
}
