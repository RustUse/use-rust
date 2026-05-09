# use-rust

Composable Rust ecosystem primitives for RustUse.

`use-rust` is the thin umbrella crate for the focused RustUse crates in this workspace.

## Example

```rust
use use_rust::prelude::{expected_repository_url, next_patch, parse_version};

let version = parse_version("0.1.0").unwrap();

assert_eq!(expected_repository_url("use-release").as_str(), "https://github.com/RustUse/use-release");
assert_eq!(next_patch(&version).to_string(), "0.1.1");
```
