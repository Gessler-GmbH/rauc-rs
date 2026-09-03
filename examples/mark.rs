use std::env::args;

use rauc::InstallerProxy;
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = args().skip(1);
    let state = args.next().expect("missing state: good, bad, or active");
    let slot = args.next().expect("missing slot identifier");

    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let info = proxy.mark(&state, &slot).await?;
    println!("{info:#?}");

    Ok(())
}
