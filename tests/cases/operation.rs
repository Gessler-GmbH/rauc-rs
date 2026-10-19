use crate::common::mock::{Dbus, MOCK_DESTINATION};
use rauc::{InstallerProxy, Operation};

#[tokio::test]
async fn decodes_recorded_operation() {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");

    let client = dbus.client();

    // The peer-to-peer mock has no bus daemon to resolve service names.
    let proxy = InstallerProxy::builder(&client)
        .destination(MOCK_DESTINATION)
        .build()
        .await
        .expect("failed to create installer proxy");

    let (reply, operation) = tokio::join!(
        dbus.reply(include_str!("../records/properties.json")),
        proxy.operation(),
    );

    reply.expect("failed to send recorded D-Bus reply");
    let operation = operation.expect("failed to get operation");

    assert_eq!(operation, Operation::Idle);
}

#[tokio::test]
async fn decodes_operation_values() {
    use std::collections::HashMap;

    use zbus::Variant;
    use zbus::wire::{LE, serialized::Context, to_bytes};

    for (value, expected) in [
        ("idle", Operation::Idle),
        ("installing", Operation::Installing),
    ] {
        // Encode RAUC's strings independently of Operation's Serde attributes.
        let properties = HashMap::from([("Operation", Variant::from(value))]);
        let body = to_bytes(Context::new(LE, 0), &properties)
            .expect("failed to serialize operation property");
        let record = serde_json::json!({
            "signature": "a{sv}",
            "body": body.bytes(),
        })
        .to_string();

        let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");
        let client = dbus.client();
        let proxy = InstallerProxy::builder(&client)
            .destination(MOCK_DESTINATION)
            .build()
            .await
            .expect("failed to create installer proxy");

        let (reply, operation) = tokio::join!(dbus.reply(&record), proxy.operation());
        reply.expect("failed to send operation property reply");
        assert_eq!(operation.expect("failed to get operation"), expected);
    }
}
