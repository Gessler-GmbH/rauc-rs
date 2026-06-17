# rauc

Thin Rust bindings for the RAUC D-Bus API.

This crate exposes a small `zbus` based API for talking to the local RAUC
service over the system D-Bus. The first exposed interface is
`de.pengutronix.rauc.Installer` via `InstallerProxy`.

## Goals

* Provide an idiomatic Rust interface for RAUC
* Support communication with the RAUC D-Bus service
* Keep the public API thin and close to the upstream D-Bus interface
* Provide examples for common RAUC operations

## Usage

```rust
use rauc::InstallerProxy;
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;
    let installer = InstallerProxy::new(&connection).await?;

    let operation = installer.operation().await?;
    println!("{operation}");

    Ok(())
}
```

The example requires a running RAUC D-Bus service on the system bus.

## Development

Build:

```bash
cargo check -p rauc --examples
```

Run the operation example:

```bash
cargo run -p rauc --example operation
```

## Updating D-Bus Interfaces

The upstream RAUC D-Bus XML files are committed in `interfaces/`.

To refresh them from upstream RAUC:

```bash
cd update-management/rauc
./tools/update-interface.sh
```

To regenerate raw `zbus` bindings for comparison:

```bash
cd update-management/rauc
./tools/generate-interface.sh
```

Generated bindings are written to `src/generated/`, which is ignored by Git.
Promote only the reviewed/adapted API into `src/` before committing.
