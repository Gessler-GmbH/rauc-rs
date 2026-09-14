# RAUC for Rust ✨

Async Rust bindings for the [RAUC D-Bus](https://rauc.readthedocs.io/en/latest/reference.html#d-bus-api) installer API,
built with [zbus](https://docs.rs/zbus).

[Installation](#installation) · [Usage](#usage) · [Examples](#examples) · [API docs](https://docs.rs/rauc) · [Contributing](CONTRIBUTING.md)

## Compatibility

| Component  | Support                       |
| ---------- | ----------------------------- |
| Platform   | Linux                         |
| Connection | System bus                    |
| Interface  | `de.pengutronix.rauc.Installer` |
| RAUC       | [RAUC_VERSION](RAUC_VERSION)   |
| Rust       | 1.87+                         |

## Installation

```bash
cargo add rauc
```

## Usage

Connect to the system bus and query slot status:

```rust,no_run
use rauc::InstallerProxy;
use zbus::{Connection, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;
    let proxy = InstallerProxy::new(&connection).await?;

    let slots = proxy.get_slot_status().await?;
    println!("{slots:#?}");

    Ok(())
}
```

See the [API documentation](https://docs.rs/rauc) for all available methods and types.

## Examples

### Inspect and monitor

```bash
# Each read-only example is named after its RAUC operation
cargo run --example get_slot_status

# Bundle inspection takes a path or URL
cargo run --example inspect_bundle -- path/to/update.raucb

# Receives installation progress changes
cargo run --example receive_progress_changed

# Receives the installation-completed signal
cargo run --example receive_completed
```

### Modify the system

> [!CAUTION]
> Installing bundles and marking slots modify the target system.

```bash
# Marks a slot as good, bad, or active
cargo run --example mark -- good booted

# Installs a specified bundle
cargo run --example install_bundle -- path/to/update.raucb
```

## Initial interface generation

The initial bindings were generated from RAUC's official installer interface for the release in `RAUC_VERSION`:

```bash
mkdir -p src/generated

RAUC_VERSION=$(cat RAUC_VERSION)
RAUC_INTERFACE=de.pengutronix.rauc.Installer.xml

curl -L "https://raw.githubusercontent.com/rauc/rauc/${RAUC_VERSION}/src/${RAUC_INTERFACE}" -o "interfaces/${RAUC_INTERFACE}"
zbus-xmlgen file "interfaces/${RAUC_INTERFACE}" -o src/generated/installer.rs
```

> [!NOTE]
> This records the original setup, not an ongoing maintenance workflow.

## Contributing

Bug fixes, features, tests, and documentation improvements are welcome.
See the [contribution guidelines](CONTRIBUTING.md) to get started.

## License

Licensed under the [MIT License](LICENSE-MIT) or [Apache License 2.0](LICENSE-APACHE), at your option.

The committed RAUC interface XML is licensed under
[CC0-1.0](https://creativecommons.org/publicdomain/zero/1.0/).
