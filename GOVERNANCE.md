# Governance

RustUse/use-rust uses a single-canonical-repository model: many public entry
points are possible, but release authority stays with the canonical GitHub
repository.

## Roles

### Maintainers

Maintainers are the people or teams recorded in [MAINTAINERS.md](MAINTAINERS.md)
or granted equivalent authority on the canonical repository.

Maintainers are responsible for:

- triaging issues, pull requests, and discussions
- approving or rejecting changes to the public API and release process
- cutting releases and publishing crates
- handling moderation, security triage, and cross-forge provenance decisions

### Contributors

Contributors may propose code, documentation, tests, release notes, issue
reports, or design feedback through the canonical repository or a documented
public mirror.

## Decision making

- Routine bug fixes, docs fixes, and narrowly scoped maintenance changes may be
  merged with maintainer review once CI is green.
- Public API expansions, breaking changes, release-policy changes, and major
  dependency changes require explicit maintainer approval.
- Security fixes may be developed privately first and disclosed after a fix or
  mitigation is ready.
- When maintainers disagree, the release authority on the canonical repository
  makes the final decision and records the rationale in the pull request,
  changelog, or release notes.

## Releases

- Release tags, GitHub releases, and crates.io publishes are created only from
  the canonical GitHub repository.
- Mirror CI is validation only and is not release authority.
- Release checklists and publish order are documented in [RELEASE.md](RELEASE.md).

## Mirrors and provenance

- Contributions may originate from mirrors.
- Accepted mirrored work should preserve authorship and reference the original
  issue, merge request, or patch URL when it is ported into GitHub.
- Mirrors must not receive publish credentials.

## Changing governance

Changes to this document require a pull request approved by a maintainer with
canonical release authority.