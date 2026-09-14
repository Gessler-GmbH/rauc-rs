## [Unreleased]

### Added

- D-Bus integration tests verifying deserialization of recorded RAUC responses and serialization of method arguments.
- `MarkState`, `SlotIdentifier`, `BundleFormat`, and `InstallationResult` enums for typed installer interactions.
- Consistent `Clone` and `Eq` implementations for public data types, with `Copy` and `Hash` for value enums.

### Changed

- Replace string arguments to `mark()` with `MarkState` and `&SlotIdentifier`, return `Operation` from `operation()`,
  and use `BundleFormat` for inspected bundle formats (breaking API changes).
- Use `InstallationResult` for the installation-completed signal, mapping zero to success and preserving nonzero
  failure codes while retaining the D-Bus integer representation (breaking API change).

### Fixed

- Decode nested variants in inspected bundle metadata with newer zvariant versions, exposing typed values directly
  without variant wrappers.
- Align field optionality with RAUC v1.15.2: accept bundles without a version and make guaranteed bundle metadata, slot
  configuration, and artifact repository fields required (breaking API change).

## [0.1.0] - 2026-09-01

### Added

- Initial asynchronous Rust bindings for the RAUC D-Bus installer API.
- APIs for inspecting and installing bundles.
- APIs for querying artifact and slot status and marking slots.
- Typed structures for bundle metadata, installation options, slot states, and progress information.
- Access to RAUC installer properties and the installation-completed signal.
- Example demonstrating how to query RAUC's `Operation` property.

[Unreleased]: https://github.com/Gessler-GmbH/rauc-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Gessler-GmbH/rauc-rs/releases/tag/v0.1.0
