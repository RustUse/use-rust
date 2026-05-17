# Changelog

## Unreleased

### Added

- Added a new multi-crate `use-rust` workspace with `use-rust-cargo`, `use-crate`, `use-version`, `use-rust-release`, and the thin umbrella `use-rust` crate.
- Added typed Cargo manifest and workspace primitives built on `cargo_metadata`, `toml_edit`, `camino`, and `serde`.
- Added crate identity, naming, repository/documentation URL, version, and release-readiness primitives for RustUse.
- Added guarded `release-plz` configuration plus publish-readiness workflows for the first public crates.io wave.

### Changed

- Repositioned the repository from the earlier single-crate experiment into the requested RustUse workspace structure.
- Updated the root README and release documentation to describe the published core crates plus the follow-up publishable `use-rust-cargo` and `use-rust-release` stage.
- Switched `use-rust-cargo` and `use-rust-release` into the manual publish surface, with `use-rust-release` remaining dependency-ordered behind `use-rust-cargo`.
