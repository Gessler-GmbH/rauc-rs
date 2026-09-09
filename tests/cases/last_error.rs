use crate::common::mock::{Dbus, MOCK_DESTINATION};
use rauc::InstallerProxy;

#[tokio::test]
async fn decodes_recorded_last_error() {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");

    let client = dbus.client();

    // The peer-to-peer mock has no bus daemon to resolve service names.
    let proxy = InstallerProxy::builder(&client)
        .destination(MOCK_DESTINATION)
        .expect("invalid mock destination")
        .build()
        .await
        .expect("failed to create installer proxy");

    let (reply, last_error) = tokio::join!(
        dbus.reply(include_str!("../records/properties.json")),
        proxy.last_error(),
    );

    reply.expect("failed to send recorded D-Bus reply");
    let last_error = last_error.expect("failed to get last_error");

    assert_eq!(last_error, "");
}
