# Releasing

This repository uses a specialized first-wave release flow rather than the
single-crate manual publish pattern used by some other RustUse repos.

## Current release state

`use-rust` is intentionally split into two public stages:

- already-published core crates: `use-version`, `use-crate`, and `use-rust`
- follow-up publishable crates: `use-rust-cargo` and `use-rust-release`

`use-rust-release` remains dependency-ordered behind `use-rust-cargo`, so its
first dry-run and first real publish still happen only after the matching
`use-rust-cargo` version resolves from crates.io.

## Canonical release guide

Use [RELEASE.md](RELEASE.md) as the authoritative release policy for:

- first-wave publish scope
- publish readiness checks
- trusted publishing setup after the first public wave
- maintainer release checklist

For the maintainer-facing day-to-day flow, also use
`docs/maintainer-release-flow.md`.

## Current automation

The repository already includes the specialized workflows that match this
release shape:

- `publish-readiness.yml`
- `facade-publish-readiness.yml`
- `release-plz-pr.yml`
- `release-plz-release.yml`

This file exists to keep the top-level release entrypoint consistent with the
other RustUse repositories while preserving the more detailed custom guidance
in `RELEASE.md`.
