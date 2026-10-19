use crate::common::mock::Dbus;
use rauc::InstallerProxy;

#[tokio::test]
async fn decodes_recorded_get_artifact_status() {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");
    let client = dbus.client();
    let proxy = InstallerProxy::new(&client)
        .await
        .expect("failed to create installer proxy");

    let (reply, result) = tokio::join!(
        dbus.reply(include_str!("../records/get-artifact-status.json")),
        proxy.get_artifact_status(),
    );

    let request = reply.expect("failed to send recorded D-Bus reply");
    let header = request.header();
    assert_eq!(
        header.member().map(|name| name.as_str()),
        Some("GetArtifactStatus")
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
    assert!(result.expect("failed to get artifact status").is_empty());
}

#[tokio::test]
async fn decodes_populated_get_artifact_status() {
    use std::collections::HashMap;

    use rauc::{ArtifactStatusArtifactInfo, ArtifactStatusInfo, ArtifactStatusInstanceInfo};
    use zbus::Variant;
    use zbus::wire::{LE, serialized::Context, to_bytes};

    // Build the wire dictionaries independently of the production structs so
    // incorrect field names or nested signatures cannot pass via a round trip.
    let instance = HashMap::from([
        ("checksum", Variant::from("0123456789abcdef")),
        ("references", Variant::from(vec!["rootfs.0", "rootfs.1"])),
    ]);
    let artifact = HashMap::from([
        ("name", Variant::from("application")),
        ("instances", Variant::from(vec![instance])),
    ]);
    let repository = HashMap::from([
        ("name", Variant::from("apps")),
        ("description", Variant::from("Application repository")),
        ("path", Variant::from("/var/lib/rauc/apps")),
        ("type", Variant::from("files")),
        ("parent-class", Variant::from("rootfs")),
        ("artifacts", Variant::from(vec![artifact])),
    ]);
    let body = to_bytes(Context::new(LE, 0), &vec![repository])
        .expect("failed to serialize artifact status");
    let record = serde_json::json!({
        "signature": "aa{sv}",
        "body": body.bytes(),
    })
    .to_string();

    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");
    let client = dbus.client();
    let proxy = InstallerProxy::new(&client)
        .await
        .expect("failed to create installer proxy");

    let (reply, result) = tokio::join!(dbus.reply(&record), proxy.get_artifact_status());
    reply.expect("failed to send populated D-Bus reply");

    assert_eq!(
        result.expect("failed to get artifact status"),
        vec![ArtifactStatusInfo {
            name: "apps".into(),
            description: Some("Application repository".into()),
            path: "/var/lib/rauc/apps".into(),
            r#type: "files".into(),
            parent_class: Some("rootfs".into()),
            artifacts: vec![ArtifactStatusArtifactInfo {
                name: "application".into(),
                instances: vec![ArtifactStatusInstanceInfo {
                    checksum: "0123456789abcdef".into(),
                    references: vec!["rootfs.0".into(), "rootfs.1".into()],
                }],
            }],
        }],
    );
}
