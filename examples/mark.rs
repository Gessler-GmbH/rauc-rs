use std::env::args;

use rauc::{InstallerProxy, MarkState, SlotIdentifier};
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = args().skip(1);
    let state = args
        .next()
        .expect(r#"missing state, expected: "good", "bad", or "active""#);
    let state = match state.as_str() {
        "good" => MarkState::Good,
        "bad" => MarkState::Bad,
        "active" => MarkState::Active,
        _ => panic!(r#"invalid state, expected: "good", "bad", or "active""#),
    };
    let slot = args
        .next()
        .expect(r#"missing slot identifier, expected: "booted", "other" or  slot-name"#);
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
