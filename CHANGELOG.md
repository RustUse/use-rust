# Changelog

## Unreleased

### Added

- Added a new multi-crate `use-rust` workspace with `use-cargo`, `use-crate`, `use-version`, `use-release`, and the thin umbrella `use-rust` crate.
- Added typed Cargo manifest and workspace primitives built on `cargo_metadata`, `toml_edit`, `camino`, and `serde`.
- Added crate identity, naming, repository/documentation URL, version, and release-readiness primitives for RustUse.
- Added workspace CI, a manual publish workflow with dry-run defaults, and Dependabot configuration.

### Changed

- Repositioned the repository from the earlier single-crate experiment into the requested RustUse workspace structure.
- Updated the root README and release documentation to describe the workspace crates and manual publish flow.
