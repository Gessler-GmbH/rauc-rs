use crate::common::mock::{Dbus, MOCK_DESTINATION};
use rauc::{InstallerProxy, Operation};

#[tokio::test]
async fn decodes_recorded_operation() {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");

    let client = dbus.client();

    // The peer-to-peer mock has no bus daemon to resolve service names.
    let proxy = InstallerProxy::builder(&client)
        .destination(MOCK_DESTINATION)
        .expect("invalid mock destination")
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
