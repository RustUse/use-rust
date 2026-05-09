# Releasing

This workspace supports manual, safety-first publish flows. It does not publish automatically on normal pushes.

## crates.io token setup

1. Create or reuse a crates.io API token with publish access for the intended crates.
2. Add the token to the GitHub repository secrets as `CARGO_REGISTRY_TOKEN`.
3. Do not print the token in logs or local shell history.

## GitHub Actions secret

- Secret name: `CARGO_REGISTRY_TOKEN`

## Dry-run publish

Use the `Publish` workflow with:

- `crate = all` or one specific workspace crate
- `dry_run = true`

This is the default mode and is the safest way to validate publish packaging before a real release.

## Manual publish

Use the `Publish` workflow with:

- `crate = all` to publish the full workspace in dependency order, or one specific crate
- `dry_run = false`

The workflow will run formatting, linting, tests, and `cargo check` before it attempts any publish step.

## Local dry-run example

```sh
cargo publish -p use-cargo --dry-run
```

## Semver notes

- Patch bumps are for compatible fixes and small additive maintenance changes.
- Minor bumps are for additive API changes during `0.x` development.
- Major bumps are for stable-line breaking changes after `1.0.0`.
- Pre-release identifiers should remain intentional and explicit.

## Permanent version warning

Published crates.io versions are permanent. You cannot replace an already published version with new contents, so verify the crate list, metadata, and changelog inputs before any real publish.
