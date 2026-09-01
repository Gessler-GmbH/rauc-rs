# rauc

Async Rust bindings for the [RAUC](https://rauc.io/) D-Bus installer API,
built with [zbus](https://docs.rs/zbus).

## Compatibility

| | |
| --- | --- |
| **Platform** | Linux |
| **Rust** | 1.85+ · Edition 2024 |
| **RAUC** | D-Bus API shipped with 1.15.2 |
| **Interface** | `de.pengutronix.rauc.Installer` |
| **Connection** | System bus |

Other RAUC releases may work when they provide a compatible D-Bus API.

## Installation

```bash
cargo add rauc
```

## Usage

```rust,no_run
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

See the [API documentation](https://docs.rs/rauc) for all available methods and
types.

> [!CAUTION]
> Installing bundles and marking slots modify the target system. Only expose
> these operations to trusted callers.

## Development

Live RAUC tests are ignored by default:

```bash
# Read-only tests
cargo test --test rauc_readonly -- --ignored

# Installs the specified bundle
RAUC_TEST_BUNDLE=/path/to/update.raucb \
  cargo test --test rauc_mutating -- --ignored
```

The interface is intentionally pinned. If the upstream API changes, update it
with `./tools/update-interface.sh` and regenerate reference bindings with
`./tools/generate-interface.sh`.

## License

Licensed under the
[Apache License 2.0](https://github.com/Gessler-GmbH/rauc-rs/blob/main/LICENSE).
The committed RAUC interface XML is licensed under
[CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/).
