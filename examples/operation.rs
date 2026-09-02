use rauc::InstallerProxy;
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let reply = proxy.operation().await?;

    println!("{:?}", reply);

    Ok(())
}
