use std::env::args;

use rauc::{InstallerProxy, MarkState, SlotIdentifier};
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = args().skip(1);
    let state = args.next().expect("missing state: good, bad, or active");
    let state = match state.as_str() {
        "good" => MarkState::Good,
        "bad" => MarkState::Bad,
        "active" => MarkState::Active,
        _ => panic!("invalid state: expected good, bad, or active"),
    };
    let slot = args.next().expect("missing slot identifier");
    let slot = match slot.as_str() {
        "booted" => SlotIdentifier::Booted,
        "other" => SlotIdentifier::Other,
        _ => SlotIdentifier::Named(slot),
    };

    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let info = proxy.mark(state, &slot).await?;
    println!("{info:#?}");

    Ok(())
}
