# rauc

Async Rust bindings for the [Rauc D-Bus]([https://rauc.io/](https://rauc.readthedocs.io/en/latest/reference.html#d-bus-api)) installer API,
built with [zbus](https://docs.rs/zbus).

## Compatibility

| Component  | Support                         |
|------------|---------------------------------|
| Platform   | Linux                           |
| Connection | System bus                      |
| Interface  | `de.pengutronix.rauc.Installer` |
| Rauc       | [_RAUC_VERSION_](RAUC_VERSION)  |
| Rust       | 1.87+                           |

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
    let proxy = InstallerProxy::new(&connection).await?;

    let reply = proxy.operation().await?;

    println!("{:?}", reply);

    Ok(())
}
```

See the [API documentation](https://docs.rs/rauc) for all available methods and
types.

## System Tests

> [!CAUTION]
> Installing bundles and marking slots modify the target system.

```bash
# Read-only tests
cargo test --test rauc_readonly -- --ignored
```

```bash
# Installs a specified bundle
RAUC_TEST_BUNDLE={path/to/update.raucb}
cargo test --test rauc_mutating -- --ignored
```

## Initial interface generation

The initial bindings were generated from RAUC's official installer interface
for the release in `RAUC_VERSION`:

```bash
mkdir -p src/generated

RAUC_VERSION=$(cat RAUC_VERSION)
RAUC_INTERFACE=de.pengutronix.rauc.Installer.xml

curl -L "https://raw.githubusercontent.com/rauc/rauc/${RAUC_VERSION}/src/${RAUC_INTERFACE}" -o "interfaces/${RAUC_INTERFACE}"
zbus-xmlgen file "interfaces/${RAUC_INTERFACE}" -o src/generated/installer.rs
```

This records the original setup, not an ongoing maintenance workflow.

## License

Licensed under the
[Apache License 2.0](https://github.com/Gessler-GmbH/rauc-rs/blob/main/LICENSE).
The committed RAUC interface XML is licensed under
[CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/).
