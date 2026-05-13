# Release Policy

RustUse/use-rust is not published yet. The root workspace metadata keeps
`publish = false` as the default, while the current first-wave crate manifests
opt in with `publish = true` on this branch.

The initial crates.io surface for this repository is intentionally limited to:

- `use-version`
- `use-crate`
- `use-rust`

The deferred crates remain in-repo but opt out of the first public wave:

- `use-cargo` stays unpublished because crates.io normalizes hyphens and
  underscores into the same namespace and `use_cargo` already exists.
- `use-release` stays unpublished because it still depends on `use-cargo`.

## First Publish Wave

The intended first publish candidates are `use-version`, `use-crate`, and the
`use-rust` facade.

Publish `use-version` and `use-crate` first. The order between those two
focused crates is maintainer-chosen because they do not depend on one another.
Wait for crates.io index propagation, then release `use-rust`.

## Publish Surface

Before the first publish wave, confirm that the release surface:

- keeps the workspace-level default at `publish = false`
- keeps `crates/use-version/Cargo.toml`, `crates/use-crate/Cargo.toml`, and
  `crates/use-rust/Cargo.toml` at `publish = true`
- keeps `crates/use-cargo/Cargo.toml` and `crates/use-release/Cargo.toml` at
  `publish = false` until the deferred crates are intentionally reviewed again

## Versioning

- The workspace currently uses lockstep `0.x.y` versioning for the publishable
  crates.
- Before `1.0`, breaking changes should bump the minor version.
- Before `1.0`, additive compatible changes should bump the patch version.
- The facade crate should only advertise actively published crates and APIs.

## Automated Release Validation

The repository now includes a dedicated release-validation path:

- `.github/workflows/publish-readiness.yml` runs on pull requests, pushes to
  `main`, and manual dispatch.
- The workflow dry-runs `use-version` and `use-crate` first.
- `.github/workflows/facade-publish-readiness.yml` is a manual post-publication
  check that dry-runs `use-rust` only after the focused crates are live on
  crates.io.
- The facade workflow fails fast unless the focused crates already resolve from
  crates.io, so the manual gate is explicit instead of relying on a downstream
  Cargo error.

## Branch Protection Gate

Before the first public release, the canonical GitHub repository should require
`Publish Readiness / Release Readiness Checks` on `main`.

This repository can document the required check name, but it cannot enforce
branch protection from version-controlled files alone. Set the rule in the
GitHub branch protection or ruleset UI before the first crates.io publish.

## Version and Changelog Automation

The repository now includes `release-plz` configuration in `release-plz.toml`
and maintainer workflows under `.github/workflows/release-plz-*.yml`.

For the maintainer-facing merge, review, and dispatch sequence, use
`docs/maintainer-release-flow.md`.

- `Release PR Automation` opens or updates a release PR with lockstep version
  changes for every publishable crate in the workspace.
- The workspace is configured with one `version_group` so the published crates
  keep the same version.
- The root `CHANGELOG.md` remains the shared changelog and is updated through
  the `use-rust` package entry, including `use-version` and `use-crate`
  changes.
- `Release Publish Automation` can publish automatically on pushes to `main`
  after the initial manual wave is complete, crates.io trusted publishing is
  configured for every published crate, and the
  `CRATES_IO_AUTOPUBLISH_ENABLED` repository variable is set to `true`.
- The publish workflow uses GitHub OIDC, keeps manual dispatch as a fallback,
  and still checks that `use-version`, `use-crate`, and `use-rust` already
  exist on crates.io before it attempts automated publishing.

One-time post-initial-release setup:

- Configure crates.io Trusted Publishing for each published crate with
  repository owner `RustUse`, repository name `use-rust`, and workflow
  filename `release-plz-release.yml`.
- Leave the crates.io environment field empty unless you intentionally add a
  matching GitHub Actions environment to the workflow later.
- Set the repository variable `CRATES_IO_AUTOPUBLISH_ENABLED` to `true` only
  after the initial manual crates.io wave is complete.
- Do not set `CARGO_REGISTRY_TOKEN` for this workflow when using trusted
  publishing.

## Maintainer Release Checklist

Use this shorter checklist when you want the operational release path without
reading the longer maintainer guide end to end.

For normal post-initial-release releases:

1. Merge ordinary PRs with clean final commit subjects or squash titles that
   match `type: summary` or `type(scope)!: summary`.
2. Let `Release PR Automation` open or update the release PR.
3. Review the release PR for the lockstep version bump, the generated root
   `CHANGELOG.md`, and any low-signal fallback entries under `Changed`.
4. Clean up the changelog directly in the release PR branch when the generated
   wording is accurate but not maintainer-quality.
5. Merge the release PR after the required checks pass.
6. Let the push-triggered `Release Publish Automation` run on the merged
   release commit, or manually dispatch it with `post-initial-release = true`
   if you need a controlled rerun.
7. Verify the published crates, docs.rs pages, and any release tags or
   artifacts after the workflow completes.

For the initial public crates.io wave:

1. Do not use `Release Publish Automation` yet.
2. Run the full release-readiness path and publish `use-version` and
   `use-crate` before `use-rust`.
3. Treat `.github/workflows/facade-publish-readiness.yml` as the final facade
   check once the focused crates resolve from crates.io.

## Publish Readiness Checklist

1. Confirm `cargo fmt` is clean.
2. Confirm `cargo check --workspace --all-features` passes.
3. Confirm `cargo check --workspace --all-features --examples` passes.
4. Confirm `cargo test --workspace --all-features` passes.
5. Confirm `cargo test --workspace --no-default-features` passes.
6. Confirm `cargo clippy --workspace --all-targets --all-features` passes.
7. Review README examples, crate metadata, repository health files,
   `Cargo.lock`, and changelog entries.
8. Confirm `use-version`, `use-crate`, and `use-rust` are the only
   intentionally publishable crates.
9. Confirm the focused-crate dry-run path passes across the first-wave set, for
   example via `.github/workflows/publish-readiness.yml`.
10. Publish `use-version` and `use-crate`, then wait for crates.io index
    resolution.
11. Confirm branch protection on `main` requires
    `Publish Readiness / Release Readiness Checks` before the first public
    release.
12. Confirm `cargo publish --dry-run --allow-dirty -p use-rust` passes, or run
    `.github/workflows/facade-publish-readiness.yml`, once the matching
    focused-crate versions are available on crates.io.
13. Publish the first wave manually because crates.io trusted publishing cannot
    create new crates for the first release.
