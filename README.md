# use-rust

Composable Rust ecosystem primitives for RustUse.

`use-rust` is the Rust ecosystem primitives set for RustUse. It provides reusable building blocks for inspecting, validating, and shaping Rust crates, Cargo workspaces, versions, metadata, and release readiness.

This repository is not a CLI app and not a publish automation tool. It is the reusable primitives layer that can later power RustUse release tooling across `use-*` repositories.

## Workspace crates

| Crate         | Purpose                                                |
| ------------- | ------------------------------------------------------ |
| `use-rust`    | Thin umbrella crate that re-exports the focused crates |
| `use-cargo`   | Cargo project and workspace primitives                 |
| `use-crate`   | Crate identity, naming, and metadata primitives        |
| `use-version` | Semver and version policy primitives                   |
| `use-release` | Release-readiness reporting primitives                 |

## Installation

Install the umbrella crate when you want the common RustUse surface:

```toml
[dependencies]
use-rust = "0.1.0"
```

Or install a focused crate directly:

```toml
[dependencies]
use-cargo = "0.1.0"
use-release = "0.1.0"
```

## Basic usage

### Inspect a nearby Cargo manifest

```rust,no_run
use use_rust::use_cargo::find_manifest;

let manifest = find_manifest(".")?;
println!("{}", manifest);
# Ok::<(), Box<dyn std::error::Error>>(())
```

### Shape crate identity defaults

```rust
use use_rust::use_crate::{crate_name_to_module_name, expected_repository_url};

assert_eq!(crate_name_to_module_name("use-release"), "use_release");
assert_eq!(
	expected_repository_url("use-release").as_str(),
	"https://github.com/RustUse/use-release"
);
```

### Work with semantic versions

```rust
use use_rust::use_version::{next_minor, parse_version};

let version = parse_version("0.1.0").unwrap();
assert_eq!(next_minor(&version).to_string(), "0.2.0");
```

## Release-readiness example

```rust,no_run
use use_rust::use_release::ReleaseReport;

let report = ReleaseReport::check("crates/use-rust")?;

if report.is_ready() {
	println!("ready to publish");
} else {
	for issue in report.issues() {
		println!("{:?}: {}", issue.check, issue.message);
	}
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
