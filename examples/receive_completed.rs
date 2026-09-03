use futures_util::StreamExt;
use rauc::InstallerProxy;
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let mut completed = proxy.receive_completed().await?;

    if let Some(signal) = completed.next().await {
        let args = signal.args()?;
        println!("{:#?}", args.result());
    }

    Ok(())
}
