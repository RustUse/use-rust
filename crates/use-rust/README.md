# use-rust

Composable Rust ecosystem primitives for RustUse.

`use-rust` is the thin umbrella crate for the publishable RustUse core crates in this workspace.

It currently re-exports:

- `use-crate` for crate naming and metadata helpers
- `use-version` for semantic version parsing and version bump helpers

## Example

```rust
use use_rust::prelude::{expected_repository_url, next_patch, parse_version};

let version = parse_version("0.1.0").unwrap();

assert_eq!(expected_repository_url("use-crate").as_str(), "https://github.com/RustUse/use-crate");
assert_eq!(next_patch(&version).to_string(), "0.1.1");
```
