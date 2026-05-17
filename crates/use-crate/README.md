# use-crate

Composable crate identity and metadata primitives for RustUse.

`use-crate` provides typed helpers for crate names, naming rules, expected RustUse URLs, and lightweight crate metadata validation.

## Example

```rust
use use_crate::{crate_name_to_module_name, expected_repository_url};

assert_eq!(crate_name_to_module_name("use-rust-release"), "use_rust_release");
assert_eq!(
    expected_repository_url("use-rust-release").as_str(),
    "https://github.com/RustUse/use-rust-release"
);
```
