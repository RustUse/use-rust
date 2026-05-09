# use-version

Composable version and semver primitives for RustUse.

`use-version` provides a small typed layer over semantic versions for release planning and policy checks.

## Example

```rust
use use_version::{next_minor, parse_version};

let version = parse_version("0.1.0").unwrap();

assert_eq!(next_minor(&version).to_string(), "0.2.0");
```
