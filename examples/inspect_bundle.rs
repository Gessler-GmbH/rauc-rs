use std::env::args;

use rauc::{InspectBundleArgs, InstallerProxy};
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let bundle = args().nth(1).expect("missing bundle path or URL");

    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let info = proxy
        .inspect_bundle(&bundle, InspectBundleArgs::default())
        .await?;
    println!("{info:#?}");

    Ok(())
}
