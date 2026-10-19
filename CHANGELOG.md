# Changelog

## [Unreleased]

### Changed

- Upgrade zbus to 6.0.0. Consumers passing a `zbus::Connection` to
  `InstallerProxy` must also use zbus 6.
- Raise the minimum supported Rust version from 1.87 to 1.89.

## [0.1.0] - 2026-09-01

### Added

- Async Rust bindings for the RAUC D-Bus installer API.
- Bundle inspection and installation with typed options and metadata.
- Slot and artifact status queries and slot marking.
- Installer properties, progress updates, and completion signals.
- Usage examples and D-Bus integration tests.

[0.1.0]: https://github.com/Gessler-GmbH/rauc-rs/releases/tag/v0.1.0
