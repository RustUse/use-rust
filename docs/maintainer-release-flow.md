# Maintainer Release Flow

This document describes how maintainers should run releases with the current
`release-plz` setup.

It covers two different paths:

- the initial public crates.io wave, which is still manual
- normal follow-up releases, where version bumps and changelog generation are
  automated and publishing stays maintainer-triggered

## Current model

- `Release PR Automation` opens or updates a release PR from `main`.
- `release-plz` keeps every publishable crate in the workspace in one lockstep
  version group.
- The shared root `CHANGELOG.md` is generated through the `use-rust` package
  entry and includes `use-version` and `use-crate` commits.
- `Release Publish Automation` runs automatically on pushes to `main` after the
  initial manual publish wave is complete and the repository enables the
  guarded auto-publish path.

## Deferred crates

The current first public wave intentionally excludes two in-repo crates:

- `use-cargo` is deferred because crates.io resolves the normalized package
  name to the canonical registry crate `use_cargo`, so Cargo cannot resolve the
  current local package identity `use-cargo` during dependent package
  verification.
- `use-release` is deferred because it still depends on `use-cargo`.

Do not add either deferred crate back into the publish surface without an
explicit maintainer decision and a follow-up release-model update.

## One-time post-initial-release setup

Before relying on automated publishing, finish these one-time steps:

- Configure crates.io Trusted Publishing for every published crate with
  repository owner `RustUse`, repository name `use-rust`, and workflow
  filename `release-plz-release.yml`.
- Leave the crates.io environment field empty unless you later add a matching
  GitHub Actions environment to the workflow.
- Set the repository variable `CRATES_IO_AUTOPUBLISH_ENABLED` to `true` only
  after the first manual crates.io wave is complete.
- Do not configure `CARGO_REGISTRY_TOKEN` for the release-plz publish workflow
  when using trusted publishing.

## How changelog generation works

The current parser rules map strict conventional-commit style subjects into
these changelog groups:

- `feat:` -> `Added`
- `fix:` -> `Fixed`
- `security:` -> `Security`
- `refactor:`, `perf:`, `change:` -> `Changed`
- `docs:` -> `Documentation`
- `build:`, `ci:`, `chore:`, `deps:`, `test:` -> `Tooling`
- `changelog: ignore` footer -> skipped from release notes

The intended subject shapes are:

- `type: summary`
- `type(scope): summary`
- `type!: summary`
- `type(scope)!: summary`

Breaking changes should use `!` in the subject or a `BREAKING CHANGE:` footer.

Any commit that does not match one of the explicit parser groups still lands in
`Changed`. That is intentional because it prevents real work from disappearing
from release notes, but it also means vague subjects create vague release
notes.

## Preferred commit and PR title examples

- `feat: add crate metadata parser`
- `fix: preserve explicit publish false in manifest parsing`
- `docs: clarify deferred crate publish policy`
- `refactor: simplify facade re-export surface`
- `build: add guarded release-plz workflows`
- `security: harden publish workflow gating`

## Normal post-initial-release flow

Use this flow after the first public crates.io wave already exists.

1. Merge ordinary PRs into `main` with clean conventional commit style in the
   final commit subject or squash-merge title.
2. Let `Release PR Automation` open or update the release PR.
3. Review the release PR for three things:
    - the lockstep version bump across all publishable crates
    - the generated root `CHANGELOG.md`
    - any low-signal fallback entries in `Changed`
4. If the generated changelog needs cleanup, edit the changelog directly in the
   release PR branch before merging.
5. Merge the release PR into `main`.
6. Confirm the push-triggered release-readiness checks are green on the merged
   release commit.
7. Let `Release Publish Automation` publish from the merged release commit, or
   manually dispatch it with `post-initial-release = true` if you need a
   controlled rerun.
8. Verify the published crates, docs.rs pages, and repository tag or release
   artifacts after the workflow completes.

## Initial public release exception

Do not use `Release Publish Automation` for the first public crates.io wave.

Use the manual dependency-ordered publish path instead:

1. Confirm `use-version`, `use-crate`, and `use-rust` are still the intended
  first-wave publishable crates.
2. Run the full publish-readiness checks.
3. Publish `use-version` and `use-crate`.
4. Wait for crates.io index propagation.
5. Run `cargo publish --dry-run -p use-rust` or the manual
  `Facade Publish Readiness` workflow.
6. Publish `use-rust`.

After that first wave is complete, the guarded auto-publish path can take over
for subsequent releases.

## Maintainer review checklist for every release PR

- The version bump is still lockstep across every publishable crate.
- The root changelog reads cleanly without vague fallback entries.
- Any intentionally skipped commits actually carry `changelog: ignore` for a
  good reason.
- The release still matches the current publish surface and feature model.
- The publish workflow is being used in the correct phase: manual first wave
  versus post-initial-release automation.
