use crate::common::mock::{Dbus, MOCK_DESTINATION};
use rauc::InstallerProxy;

#[tokio::test]
async fn decodes_recorded_variant() {
    let mut dbus = Dbus::new().await.expect("failed to create D-Bus mock");

    let client = dbus.client();

    // The peer-to-peer mock has no bus daemon to resolve service names.
    let proxy = InstallerProxy::builder(&client)
        .destination(MOCK_DESTINATION)
        .expect("invalid mock destination")
        .build()
        .await
        .expect("failed to create installer proxy");

    let (reply, variant) = tokio::join!(
        dbus.reply(include_str!("../records/properties.json")),
        proxy.variant(),
    );

    reply.expect("failed to send recorded D-Bus reply");
    let variant = variant.expect("failed to get variant");

    assert_eq!(variant, "");
}
