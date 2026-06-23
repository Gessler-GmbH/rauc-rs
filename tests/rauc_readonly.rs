use std::env::var;
use zbus::{Connection, Result};

use rauc::InspectBundleArgs;
use rauc::InstallerProxy;

#[tokio::test]
#[ignore = "requires RAUC running on the system bus"]
async fn get_artifact_status() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let artifacts = proxy.get_artifact_status().await?;

    println!("{artifacts:#?}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires RAUC running on the system bus"]
async fn get_primary() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let primary = proxy.get_primary().await?;

    assert!(!primary.is_empty());

    println!("{primary:#?}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires RAUC running on the system bus"]
async fn get_slot_status() -> zbus::Result<()> {
    let connection = zbus::Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let slots = proxy.get_slot_status().await?;

    assert!(!slots.is_empty());

    println!("{slots:#?}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires RAUC running on the system bus and RAUC_TEST_BUNDLE"]
async fn inspect_bundle() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let bundle =
        var("RAUC_TEST_BUNDLE").expect("set RAUC_TEST_BUNDLE to a valid URL or .raucb bundle path");

    let info = proxy
        .inspect_bundle(&bundle, InspectBundleArgs::default())
        .await?;

    println!("{info:#?}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires RAUC running on the system bus"]
async fn boot_slot() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let boot_slot = proxy.boot_slot().await?;

    println!("{boot_slot:#?}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires RAUC running on the system bus"]
async fn compatible() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let compatible = proxy.compatible().await?;

    println!("{compatible:#?}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires RAUC running on the system bus"]
async fn last_error() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let last_error = proxy.last_error().await?;

    println!("{last_error:#?}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires RAUC running on the system bus"]
async fn operation() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let operation = proxy.operation().await?;

    println!("{operation:#?}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires RAUC running on the system bus"]
async fn progress() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let progress = proxy.progress().await?;

    println!("{progress:#?}");

    Ok(())
}

#[tokio::test]
#[ignore = "requires RAUC running on the system bus"]
async fn variant() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let variant = proxy.variant().await?;

    println!("{variant:#?}");

    Ok(())
}
