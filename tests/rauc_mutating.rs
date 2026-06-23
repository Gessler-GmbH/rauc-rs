use std::env::var;

use std::time::Duration;

use tokio::time::sleep;

use zbus::{Connection, Result};

use rauc::{InstallBundleArgs, InstallerProxy};

#[tokio::test]
#[ignore = "installs a RAUC bundle and requires RAUC_TEST_BUNDLE"]
async fn install_bundle() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let bundle =
        var("RAUC_TEST_BUNDLE").expect("set RAUC_TEST_BUNDLE to a valid bundle path or URL");

    proxy
        .install_bundle(&bundle, InstallBundleArgs::default())
        .await?;

    loop {
        let operation = proxy.operation().await?;
        let progress = proxy.progress().await?;

        print!("\r{progress:#?}");

        if operation == "idle" {
            break;
        }

        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
}
