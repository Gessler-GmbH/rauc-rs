# rauc

Async Rust bindings for the [RAUC](https://rauc.io/) D-Bus installer API, built
with [`zbus`](https://docs.rs/zbus).

Requires Linux, Rust 1.85 or newer, and a running RAUC service on the system
bus.

## Usage

```rust
use rauc::InstallerProxy;
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;
    let installer = InstallerProxy::new(&connection).await?;

    println!("{}", installer.operation().await?);
    Ok(())
}
```

## Development

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

Tests that communicate with RAUC are ignored by default. Run them on a suitable
system with `cargo test -- --ignored`. Bundle tests also require
`RAUC_TEST_BUNDLE` to contain a bundle path or URL.

Update the committed RAUC D-Bus interface from the repository root with
`./tools/update-interface.sh`.
