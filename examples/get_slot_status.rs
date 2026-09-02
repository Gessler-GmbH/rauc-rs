use rauc::InstallerProxy;
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let slots = proxy.get_slot_status().await?;
    println!("{slots:#?}");

    Ok(())
}
