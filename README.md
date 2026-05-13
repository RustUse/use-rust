# use-rust

Composable Rust ecosystem primitives for RustUse.

`use-rust` is the Rust ecosystem primitives set for RustUse. It provides reusable building blocks for shaping Rust crate identity and semantic versions, with a first public crates.io wave focused on the publishable core crates.

This repository is not a CLI app and not a publish automation tool. It is the reusable primitives layer that can later power RustUse release tooling across `use-*` repositories.

The current first public crates.io wave is intentionally smaller than the full in-repo workspace. It publishes `use-version`, `use-crate`, and the `use-rust` facade first, while keeping the deferred `use-cargo` and `use-release` crates in the repository for follow-up work.

## Workspace crates

| Crate         | Purpose                                             | First-wave crates.io status |
| ------------- | --------------------------------------------------- | --------------------------- |
| `use-rust`    | Thin umbrella crate for the publishable core crates | publish                     |
| `use-crate`   | Crate identity, naming, and metadata primitives     | publish                     |
| `use-version` | Semver and version policy primitives                | publish                     |
| `use-cargo`   | Cargo project and workspace primitives              | defer                       |
| `use-release` | Release-readiness reporting primitives              | defer                       |

## Installation

Install the umbrella crate when you want the common RustUse surface:

```toml
[dependencies]
use-rust = "0.1.0"
```

Or install the focused first-wave crates directly:

```toml
[dependencies]
use-crate = "0.1.0"
use-version = "0.1.0"
```

## Deferred crates

`use-cargo` is currently deferred because crates.io normalizes hyphens and underscores into the same namespace and `use_cargo` already exists.

`use-release` is currently deferred because it still depends on `use-cargo`.

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
