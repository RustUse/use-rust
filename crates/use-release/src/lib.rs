#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Release-readiness reporting primitives for RustUse crates.

use std::{
    error::Error,
    fmt,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use use_cargo::{CargoManifest, CargoManifestError, find_workspace_root, load_manifest};
use use_crate::{
    CrateMetadata, expected_docs_url, expected_repository_url, is_use_prefixed, is_valid_crate_name,
};
use use_version::{
    ReleaseLevel, Version, VersionBump, VersionError, next_major, next_minor, next_patch,
    parse_version,
};

/// A single release-readiness check.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseCheck {
    CargoTomlExists,
    ReadmeExists,
    LicenseFilesExist,
    DescriptionPresent,
    LicensePresent,
    RepositoryPresent,
    DocumentationPresent,
    HomepagePresent,
    Publishable,
    VersionValid,
    CrateNameValid,
    RustUseNaming,
}

/// The overall release status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReleaseStatus {
    Ready,
    HasIssues,
}

/// A recorded release-readiness issue.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseIssue {
    pub check: ReleaseCheck,
    pub message: String,
}

/// A local release-readiness report.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseReport {
    package_name: Option<String>,
    status: ReleaseStatus,
    issues: Vec<ReleaseIssue>,
}

impl ReleaseReport {
    /// Checks a crate directory for release-readiness issues.
    pub fn check(path: impl AsRef<Path>) -> Result<Self, ReleaseError> {
        let package_root = normalize_package_root(path.as_ref());
        let manifest_path = package_root.join("Cargo.toml");
        let readme_path = package_root.join("README.md");
        let mut issues = Vec::new();
        let mut package_name = None;

        if !manifest_path.is_file() {
            issues.push(issue(
                ReleaseCheck::CargoTomlExists,
                "Cargo.toml is missing",
            ));
        }

        if !readme_path.is_file() {
            issues.push(issue(ReleaseCheck::ReadmeExists, "README.md is missing"));
        }

        if !has_license_files(&package_root) {
            issues.push(issue(
                ReleaseCheck::LicenseFilesExist,
                "license files are missing from the package or workspace root",
            ));
        }

        if manifest_path.is_file() {
            let manifest = load_manifest(&manifest_path)?;
            package_name = manifest.package_name().map(ToOwned::to_owned);
            append_manifest_issues(&manifest, &mut issues)?;
        }

        let status = if issues.is_empty() {
            ReleaseStatus::Ready
        } else {
            ReleaseStatus::HasIssues
        };

        Ok(Self {
            package_name,
            status,
            issues,
        })
    }

    /// Returns the package name when known.
    #[must_use]
    pub fn package_name(&self) -> Option<&str> {
        self.package_name.as_deref()
    }

    /// Returns the overall release status.
    #[must_use]
    pub fn status(&self) -> ReleaseStatus {
        self.status
    }

    /// Returns `true` when no issues were recorded.
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.status == ReleaseStatus::Ready
    }

    /// Returns the recorded issues.
    #[must_use]
    pub fn issues(&self) -> &[ReleaseIssue] {
        &self.issues
    }
}

/// A simple release plan derived from a version bump.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleasePlan {
    pub package_name: String,
    pub current_version: Version,
    pub next_version: Version,
    pub level: ReleaseLevel,
}

impl ReleasePlan {
    /// Builds a release plan from a current version and bump type.
    #[must_use]
    pub fn from_bump(
        package_name: impl Into<String>,
        current_version: Version,
        bump: VersionBump,
    ) -> Self {
        let (next_version, level) = match bump {
            VersionBump::Patch => (next_patch(&current_version), ReleaseLevel::Patch),
            VersionBump::Minor => (next_minor(&current_version), ReleaseLevel::Minor),
            VersionBump::Major => (next_major(&current_version), ReleaseLevel::Major),
        };

        Self {
            package_name: package_name.into(),
            current_version,
            next_version,
            level,
        }
    }
}

/// Errors that can occur while producing a release report.
#[derive(Debug)]
pub enum ReleaseError {
    Manifest(CargoManifestError),
    Version(VersionError),
}

impl fmt::Display for ReleaseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manifest(error) => write!(formatter, "failed to inspect Cargo manifest: {error}"),
            Self::Version(error) => {
                write!(formatter, "failed to inspect version metadata: {error}")
            },
        }
    }
}

impl Error for ReleaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Manifest(error) => Some(error),
            Self::Version(error) => Some(error),
        }
    }
}

impl From<CargoManifestError> for ReleaseError {
    fn from(error: CargoManifestError) -> Self {
        Self::Manifest(error)
    }
}

impl From<VersionError> for ReleaseError {
    fn from(error: VersionError) -> Self {
        Self::Version(error)
    }
}

fn append_manifest_issues(
    manifest: &CargoManifest,
    issues: &mut Vec<ReleaseIssue>,
) -> Result<(), ReleaseError> {
    if manifest.description().is_none() {
        issues.push(issue(
            ReleaseCheck::DescriptionPresent,
            "package.description is missing",
        ));
    }

    if manifest.license().is_none() {
        issues.push(issue(
            ReleaseCheck::LicensePresent,
            "package.license is missing",
        ));
    }

    if manifest.repository().is_none() {
        issues.push(issue(
            ReleaseCheck::RepositoryPresent,
            "package.repository is missing",
        ));
    }

    if manifest.documentation().is_none() {
        issues.push(issue(
            ReleaseCheck::DocumentationPresent,
            "package.documentation is missing",
        ));
    }

    if manifest.homepage().is_none() {
        issues.push(issue(
            ReleaseCheck::HomepagePresent,
            "package.homepage is missing",
        ));
    }

    if !manifest.is_publishable() {
        issues.push(issue(
            ReleaseCheck::Publishable,
            "package is not publishable under current Cargo metadata",
        ));
    }

    match manifest.package_version() {
        Some(version) => {
            if parse_version(version).is_err() {
                issues.push(issue(
                    ReleaseCheck::VersionValid,
                    "package.version is not valid semantic versioning",
                ));
            }
        },
        None => issues.push(issue(
            ReleaseCheck::VersionValid,
            "package.version is missing",
        )),
    }

    match manifest.package_name() {
        Some(name) => {
            if !is_valid_crate_name(name) {
                issues.push(issue(
                    ReleaseCheck::CrateNameValid,
                    "package.name is not a valid crate name",
                ));
            }

            if !is_use_prefixed(name) {
                issues.push(issue(
                    ReleaseCheck::RustUseNaming,
                    "package.name does not follow the RustUse use-* naming convention",
                ));
            }

            if let Some(repository) = manifest.repository() {
                let expected = expected_repository_url(name);
                if repository != expected.as_str() {
                    issues.push(issue(
                        ReleaseCheck::RustUseNaming,
                        &format!("package.repository should be {}", expected.as_str()),
                    ));
                }
            }

            if let Some(documentation) = manifest.documentation() {
                let expected = expected_docs_url(name);
                if documentation != expected.as_str() {
                    issues.push(issue(
                        ReleaseCheck::RustUseNaming,
                        &format!("package.documentation should be {}", expected.as_str()),
                    ));
                }
            }

            if let Some(homepage) = manifest.homepage() {
                if homepage != "https://rustuse.org" {
                    issues.push(issue(
                        ReleaseCheck::RustUseNaming,
                        "package.homepage should be https://rustuse.org",
                    ));
                }
            }

            if let Some(metadata) =
                CrateMetadata::from_manifest_path(manifest.path().as_path().as_std_path())
            {
                for message in use_crate::validate_crate_metadata(&metadata) {
                    issues.push(issue(ReleaseCheck::RustUseNaming, &message));
                }
            }
        },
        None => issues.push(issue(
            ReleaseCheck::CrateNameValid,
            "package.name is missing",
        )),
    }

    Ok(())
}

fn issue(check: ReleaseCheck, message: &str) -> ReleaseIssue {
    ReleaseIssue {
        check,
        message: message.to_string(),
    }
}

fn normalize_package_root(path: &Path) -> PathBuf {
    if path.is_dir() {
        return path.to_path_buf();
    }

    path.parent()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

fn has_license_files(package_root: &Path) -> bool {
    has_license_files_in(package_root)
        || find_workspace_root(package_root)
            .map(|root| has_license_files_in(root.as_path().as_std_path()))
            .unwrap_or(false)
}

fn has_license_files_in(root: &Path) -> bool {
    ["LICENSE", "LICENSE.md", "LICENSE-MIT", "LICENSE-APACHE"]
        .iter()
        .any(|name| root.join(name).is_file())
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        process,
        time::{SystemTime, UNIX_EPOCH},
    };

    use use_version::{VersionBump, parse_version};

    use super::{ReleasePlan, ReleaseReport, ReleaseStatus};

    #[test]
    fn reports_ready_workspace_member() {
        let temp_dir = TestDir::new("release-ready");
        write_file(&temp_dir.path().join("LICENSE-MIT"), "MIT\n");
        write_file(&temp_dir.path().join("LICENSE-APACHE"), "Apache\n");
        write_file(
            &temp_dir.path().join("Cargo.toml"),
            r#"[workspace]
members = ["crates/use-demo"]
"#,
        );
        write_file(
            &temp_dir
                .path()
                .join("crates")
                .join("use-demo")
                .join("Cargo.toml"),
            r#"[package]
name = "use-demo"
version = "0.1.0"
edition = "2021"
description = "demo"
license = "MIT OR Apache-2.0"
repository = "https://github.com/RustUse/use-demo"
documentation = "https://docs.rs/use-demo"
homepage = "https://rustuse.org"
readme = "README.md"
"#,
        );
        write_file(
            &temp_dir
                .path()
                .join("crates")
                .join("use-demo")
                .join("README.md"),
            "# use-demo\n",
        );
        write_file(
            &temp_dir
                .path()
                .join("crates")
                .join("use-demo")
                .join("src")
                .join("lib.rs"),
            "pub fn sample() {}\n",
        );

        let report = ReleaseReport::check(temp_dir.path().join("crates").join("use-demo"))
            .expect("release report should build");

        assert!(report.is_ready());
        assert_eq!(report.status(), ReleaseStatus::Ready);
        assert!(report.issues().is_empty());
    }

    #[test]
    fn reports_release_issues_for_missing_metadata() {
        let temp_dir = TestDir::new("release-issues");
        write_file(
            &temp_dir.path().join("Cargo.toml"),
            r#"[package]
name = "Bad crate"
version = "banana"
edition = "2021"
"#,
        );

        let report = ReleaseReport::check(temp_dir.path()).expect("release report should build");

        assert!(!report.is_ready());
        assert!(report.issues().len() >= 6);
    }

    #[test]
    fn creates_release_plans() {
        let current = parse_version("0.1.0").expect("version should parse");
        let plan = ReleasePlan::from_bump("use-demo", current, VersionBump::Minor);

        assert_eq!(plan.package_name, "use-demo");
        assert_eq!(plan.next_version.to_string(), "0.2.0");
    }

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new(label: &str) -> Self {
            let mut path = std::env::temp_dir();
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after UNIX_EPOCH")
                .as_nanos();
            path.push(format!("use-release-{label}-{}-{nanos}", process::id()));
            fs::create_dir_all(&path).expect("temporary directory should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn write_file(path: &Path, contents: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("parent directories should be created");
        }

        fs::write(path, contents).expect("file should be written");
    }
}
