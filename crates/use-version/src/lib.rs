#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Composable semantic-version primitives for RustUse.

use std::{cmp::Ordering, error::Error, fmt};

use semver::{BuildMetadata, Prerelease};
use serde::{Deserialize, Serialize};

/// A typed semantic version value.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Version(pub semver::Version);

impl Version {
    /// Returns the wrapped semantic version.
    #[must_use]
    pub fn as_semver(&self) -> &semver::Version {
        &self.0
    }
}

impl fmt::Display for Version {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// The kind of version bump to apply.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionBump {
    Patch,
    Minor,
    Major,
}

/// A simple version policy marker for RustUse release flows.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum VersionPolicy {
    StrictSemver,
    RustUseDefault,
}

/// The release level represented by a version transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseLevel {
    Patch,
    Minor,
    Major,
    PreRelease,
}

/// Errors that can occur while parsing semantic versions.
#[derive(Debug)]
pub struct VersionError(semver::Error);

impl fmt::Display for VersionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "failed to parse semantic version: {}", self.0)
    }
}

impl Error for VersionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}

impl From<semver::Error> for VersionError {
    fn from(error: semver::Error) -> Self {
        Self(error)
    }
}

/// Parses a semantic version string.
pub fn parse_version(value: &str) -> Result<Version, VersionError> {
    Ok(Version(semver::Version::parse(value)?))
}

/// Returns `true` when a version contains prerelease metadata.
#[must_use]
pub fn is_prerelease(version: &Version) -> bool {
    !version.0.pre.is_empty()
}

/// Returns the next patch version.
#[must_use]
pub fn next_patch(version: &Version) -> Version {
    let mut next = version.0.clone();
    next.patch += 1;
    next.pre = Prerelease::EMPTY;
    next.build = BuildMetadata::EMPTY;
    Version(next)
}

/// Returns the next minor version.
#[must_use]
pub fn next_minor(version: &Version) -> Version {
    let mut next = version.0.clone();
    next.minor += 1;
    next.patch = 0;
    next.pre = Prerelease::EMPTY;
    next.build = BuildMetadata::EMPTY;
    Version(next)
}

/// Returns the next major version.
#[must_use]
pub fn next_major(version: &Version) -> Version {
    let mut next = version.0.clone();
    next.major += 1;
    next.minor = 0;
    next.patch = 0;
    next.pre = Prerelease::EMPTY;
    next.build = BuildMetadata::EMPTY;
    Version(next)
}

/// Compares two semantic versions.
#[must_use]
pub fn compare_versions(left: &Version, right: &Version) -> Ordering {
    left.cmp(right)
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::{
        compare_versions, is_prerelease, next_major, next_minor, next_patch, parse_version,
    };

    #[test]
    fn parses_versions() {
        let version = parse_version("1.2.3").expect("version should parse");

        assert_eq!(version.to_string(), "1.2.3");
        assert_eq!(version.as_semver().major, 1);
        assert!(parse_version("not-a-version").is_err());
    }

    #[test]
    fn detects_prereleases() {
        let stable = parse_version("1.2.3").expect("stable version should parse");
        let prerelease = parse_version("1.2.3-alpha.1").expect("prerelease should parse");

        assert!(!is_prerelease(&stable));
        assert!(is_prerelease(&prerelease));
    }

    #[test]
    fn bumps_versions() {
        let version = parse_version("0.1.2-alpha.1+build.7").expect("version should parse");

        assert_eq!(next_patch(&version).to_string(), "0.1.3");
        assert_eq!(next_minor(&version).to_string(), "0.2.0");
        assert_eq!(next_major(&version).to_string(), "1.0.0");
    }

    #[test]
    fn compares_versions() {
        let earlier = parse_version("0.1.0").expect("version should parse");
        let later = parse_version("0.2.0").expect("version should parse");

        assert_eq!(compare_versions(&earlier, &later), Ordering::Less);
        assert_eq!(compare_versions(&later, &earlier), Ordering::Greater);
        assert_eq!(compare_versions(&earlier, &earlier), Ordering::Equal);
    }
}
