use rauc::InstallerProxy;
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let artifacts = proxy.get_artifact_status().await?;
    println!("{artifacts:#?}");

    Ok(())
}
