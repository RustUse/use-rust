# use-cargo

Composable Cargo project and workspace primitives for RustUse.

`use-cargo` provides typed helpers for finding manifests, reading Cargo metadata, and inspecting local Cargo workspaces.

## Example

```rust,no_run
use use_cargo::find_manifest;

let manifest = find_manifest(".")?;
println!("{}", manifest);
# Ok::<(), Box<dyn std::error::Error>>(())
```
