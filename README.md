# use-rust

Composable Rust ecosystem primitives for RustUse.

`use-rust` is the Rust ecosystem primitives set for RustUse. It provides reusable building blocks for shaping Rust crate identity and semantic versions, with a first public crates.io wave focused on the publishable core crates.

This repository is not a CLI app and not a publish automation tool. It is the reusable primitives layer that can later power RustUse release tooling across `use-*` repositories.

The published core crates.io wave is intentionally smaller than the full in-repo workspace. `use-version`, `use-crate`, and the `use-rust` facade are already part of the public surface, and the renamed follow-up crates `use-rust-cargo` and `use-rust-release` now opt into publishing as a second manual stage.

## Workspace crates

| Crate         | Purpose                                             | Current crates.io status            |
| ------------- | --------------------------------------------------- | ----------------------------------- |
| `use-rust`    | Thin umbrella crate for the publishable core crates | published                           |
| `use-crate`   | Crate identity, naming, and metadata primitives     | published                           |
| `use-version` | Semver and version policy primitives                | published                           |
| `use-rust-cargo` | Cargo project and workspace primitives           | publishable follow-up crate         |
| `use-rust-release` | Release-readiness reporting primitives         | publishable after `use-rust-cargo`  |

## Installation

Install the umbrella crate when you want the common RustUse surface:

```toml
[dependencies]
use-rust = "0.1.0"
```

Or install the already-published focused crates directly:

```toml
[dependencies]
use-crate = "0.1.0"
use-version = "0.1.0"
```

## Follow-up publishable crates

`use-rust-cargo` is the renamed successor to the earlier `use-cargo` identity, avoiding the crates.io namespace collision with the canonical crate `use_cargo`. Its local `cargo publish --dry-run` path now passes.

`use-rust-release` is now publishable too, but its dry-run remains intentionally staged after `use-rust-cargo` because Cargo resolves the exact sibling dependency from crates.io during package verification.

## Basic usage

### Shape crate identity defaults

```rust
use use_rust::use_crate::{crate_name_to_module_name, expected_repository_url};

assert_eq!(crate_name_to_module_name("use-crate"), "use_crate");
assert_eq!(
	expected_repository_url("use-crate").as_str(),
	"https://github.com/RustUse/use-crate"
);
```

### Work with semantic versions

```rust
use use_rust::use_version::{next_minor, parse_version};

let version = parse_version("0.1.0").unwrap();
assert_eq!(next_minor(&version).to_string(), "0.2.0");
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
