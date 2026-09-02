use std::env::args;

use rauc::{InstallBundleArgs, InstallerProxy};
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let bundle = args().nth(1).expect("missing bundle path or URL");

    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    proxy
        .install_bundle(&bundle, InstallBundleArgs::default())
        .await?;

    Ok(())
}
