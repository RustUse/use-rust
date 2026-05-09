# Contributing

RustUse/use-rust is intentionally small at the API level even though it is now a multi-crate workspace. Contributions should keep the public Rust primitives explicit, typed, and pragmatic.

## Development flow

1. Make the smallest useful change.
2. Add or update tests for public behavior changes.
3. Prefer direct helpers over broad framework abstractions.
4. Keep public docs aligned with the actual local Cargo, crate, version, and release-reporting behavior.

## Local validation

```sh
cargo fmt --all --check
cargo check --workspace --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
```

## Scope guidance

- `use-rust` is the RustUse Rust ecosystem primitives set, not a CLI and not a general DevOps automation layer.
- Prefer reusable typed primitives over shell-oriented workflows.
- Avoid network calls and avoid real publish operations in library code.
- Keep release checks reporting-oriented and easy to test with temporary fixture directories.
