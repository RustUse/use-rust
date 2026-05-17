# use-rust-release

Composable release-readiness primitives for RustUse.

`use-rust-release` reports local release issues for crates without performing network calls or running `cargo publish`.

## Example

```rust,no_run
use use_rust_release::ReleaseReport;

let report = ReleaseReport::check(".")?;
assert!(report.is_ready() || !report.issues().is_empty());
# Ok::<(), Box<dyn std::error::Error>>(())
```
