use futures_util::StreamExt;
use rauc::InstallerProxy;
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let mut progress_changed = proxy.receive_progress_changed().await;

    while let Some(change) = progress_changed.next().await {
        let progress = change.get().await?;
        println!("{:#?}", progress);
    }

    Ok(())
}
