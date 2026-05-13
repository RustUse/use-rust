# Changelog

## Unreleased

### Added

- Added a new multi-crate `use-rust` workspace with `use-cargo`, `use-crate`, `use-version`, `use-release`, and the thin umbrella `use-rust` crate.
- Added typed Cargo manifest and workspace primitives built on `cargo_metadata`, `toml_edit`, `camino`, and `serde`.
- Added crate identity, naming, repository/documentation URL, version, and release-readiness primitives for RustUse.
- Added guarded `release-plz` configuration plus publish-readiness workflows for the first public crates.io wave.

### Changed

- Repositioned the repository from the earlier single-crate experiment into the requested RustUse workspace structure.
- Updated the root README and release documentation to describe the first-wave publish surface and the deferred crates.
- Narrowed the initial crates.io release scope to `use-version`, `use-crate`, and `use-rust`, while keeping `use-cargo` and `use-release` in-repo but unpublished.
