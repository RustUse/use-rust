#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Crate identity, naming, and publishability primitives.

use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use toml_edit::{DocumentMut, Item};

/// A validated crate name.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CrateName(String);

impl CrateName {
    /// Creates a validated crate name after applying RustUse normalization.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CrateNameError> {
        let normalized = normalize_crate_name(value.as_ref());

        if is_valid_crate_name(&normalized) {
            Ok(Self(normalized))
        } else {
            Err(CrateNameError(value.as_ref().to_string()))
        }
    }

    /// Returns the crate name.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CrateName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A broad crate kind classification.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CrateKind {
    Library,
    Binary,
    Mixed,
    Unknown,
}

/// The publish status inferred from manifest metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PublishStatus {
    Publishable,
    Unpublishable,
}

/// A repository URL.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RepositoryUrl(String);

impl RepositoryUrl {
    /// Creates a repository URL wrapper.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the underlying URL.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RepositoryUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A documentation URL.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DocumentationUrl(String);

impl DocumentationUrl {
    /// Creates a documentation URL wrapper.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the underlying URL.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DocumentationUrl {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Lightweight crate metadata for RustUse validation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrateMetadata {
    pub name: CrateName,
    pub kind: CrateKind,
    pub description: Option<String>,
    pub license: Option<String>,
    pub repository: Option<RepositoryUrl>,
    pub documentation: Option<DocumentationUrl>,
    pub homepage: Option<String>,
    pub publish_status: PublishStatus,
}

impl CrateMetadata {
    /// Builds crate metadata from a Cargo.toml manifest path or package root.
    #[must_use]
    pub fn from_manifest_path(path: impl AsRef<Path>) -> Option<Self> {
        let manifest_path = resolve_manifest_path(path.as_ref());
        let contents = fs::read_to_string(&manifest_path).ok()?;
        let document = contents.parse::<DocumentMut>().ok()?;

        Self::from_manifest_document(&manifest_path, &document)
    }

    fn from_manifest_document(manifest_path: &Path, document: &DocumentMut) -> Option<Self> {
        let name = CrateName::new(package_str(document, "name")?).ok()?;
        let crate_root = manifest_path.parent()?;
        let has_lib = crate_root.join("src/lib.rs").exists();
        let has_main = crate_root.join("src/main.rs").exists();

        let kind = match (has_lib, has_main) {
            (true, true) => CrateKind::Mixed,
            (true, false) => CrateKind::Library,
            (false, true) => CrateKind::Binary,
            (false, false) => CrateKind::Unknown,
        };

        Some(Self {
            name,
            kind,
            description: package_str(document, "description").map(ToOwned::to_owned),
            license: package_str(document, "license").map(ToOwned::to_owned),
            repository: package_str(document, "repository").map(RepositoryUrl::new),
            documentation: package_str(document, "documentation").map(DocumentationUrl::new),
            homepage: package_str(document, "homepage").map(ToOwned::to_owned),
            publish_status: if manifest_is_publishable(document) {
                PublishStatus::Publishable
            } else {
                PublishStatus::Unpublishable
            },
        })
    }
}

/// An error returned when a crate name fails validation.
#[derive(Debug)]
pub struct CrateNameError(String);

impl fmt::Display for CrateNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "invalid crate name: {}", self.0)
    }
}

impl Error for CrateNameError {}

fn resolve_manifest_path(path: &Path) -> PathBuf {
    if path.is_dir() {
        path.join("Cargo.toml")
    } else {
        path.to_path_buf()
    }
}

fn package_item<'a>(document: &'a DocumentMut, field: &str) -> Option<&'a Item> {
    document
        .get("package")
        .and_then(Item::as_table_like)
        .and_then(|package| package.get(field))
}

fn package_str<'a>(document: &'a DocumentMut, field: &str) -> Option<&'a str> {
    package_item(document, field)
        .and_then(Item::as_value)
        .and_then(|value| value.as_str())
}

fn manifest_is_publishable(document: &DocumentMut) -> bool {
    match package_item(document, "publish") {
        None => true,
        Some(item) => item
            .as_value()
            .and_then(|value| value.as_bool())
            .or_else(|| {
                item.as_value()
                    .and_then(|value| value.as_array())
                    .map(|items| !items.is_empty())
            })
            .unwrap_or(true),
    }
}

/// Returns `true` when a value is a valid crate name under RustUse defaults.
#[must_use]
pub fn is_valid_crate_name(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }

    let bytes = value.as_bytes();
    let first = bytes[0];
    let last = bytes[bytes.len() - 1];

    if matches!(first, b'-' | b'_') || matches!(last, b'-' | b'_') {
        return false;
    }

    value.chars().all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, '-' | '_')
    })
}

/// Returns `true` when a crate name uses the RustUse `use-*` prefix.
#[must_use]
pub fn is_use_prefixed(value: &str) -> bool {
    value.starts_with("use-")
}

/// Converts a crate name like `use-rust-release` into a Rust module name.
#[must_use]
pub fn crate_name_to_module_name(value: &str) -> String {
    value.replace('-', "_")
}

/// Converts a Rust module name like `use_rust_release` into a crate name.
#[must_use]
pub fn module_name_to_crate_name(value: &str) -> String {
    value.replace('_', "-")
}

/// Normalizes a crate name by trimming, ASCII-lowercasing, replacing spaces or
/// underscores with hyphens, collapsing repeated hyphens, and trimming edges.
#[must_use]
pub fn normalize_crate_name(value: &str) -> String {
    let mut normalized = String::with_capacity(value.len());
    let mut last_was_hyphen = false;

    for character in value.trim().chars() {
        let mapped = match character {
            ' ' | '_' => '-',
            _ => character.to_ascii_lowercase(),
        };

        if mapped == '-' {
            if normalized.is_empty() || last_was_hyphen {
                continue;
            }

            last_was_hyphen = true;
            normalized.push(mapped);
            continue;
        }

        last_was_hyphen = false;
        normalized.push(mapped);
    }

    while normalized.ends_with('-') {
        normalized.pop();
    }

    normalized
}

/// Returns the expected RustUse GitHub repository URL for a repository name.
#[must_use]
pub fn expected_repository_url(repo_name: &str) -> RepositoryUrl {
    RepositoryUrl::new(format!(
        "https://github.com/RustUse/{}",
        normalize_crate_name(repo_name)
    ))
}

/// Returns the expected docs.rs URL for a crate name.
#[must_use]
pub fn expected_docs_url(crate_name: &str) -> DocumentationUrl {
    DocumentationUrl::new(format!(
        "https://docs.rs/{}",
        normalize_crate_name(crate_name)
    ))
}

/// Returns `true` when crate metadata is publishable and follows RustUse defaults.
#[must_use]
pub fn is_publishable(metadata: &CrateMetadata) -> bool {
    metadata.publish_status == PublishStatus::Publishable
        && validate_crate_metadata(metadata).is_empty()
}

/// Validates crate metadata against RustUse naming and URL defaults.
#[must_use]
pub fn validate_crate_metadata(metadata: &CrateMetadata) -> Vec<String> {
    let mut issues = Vec::new();
    let crate_name = metadata.name.as_str();

    if !is_valid_crate_name(crate_name) {
        issues.push(String::from("crate name is not a valid package name"));
    }

    if !is_use_prefixed(crate_name) {
        issues.push(String::from(
            "crate name does not follow the RustUse use-* naming convention",
        ));
    }

    if let Some(repository) = &metadata.repository {
        let expected = expected_repository_url(crate_name);
        if repository != &expected {
            issues.push(format!("repository URL should be {}", expected.as_str()));
        }
    }

    if let Some(documentation) = &metadata.documentation {
        let expected = expected_docs_url(crate_name);
        if documentation != &expected {
            issues.push(format!("documentation URL should be {}", expected.as_str()));
        }
    }

    if let Some(homepage) = &metadata.homepage {
        if homepage != "https://rustuse.org" {
            issues.push(String::from(
                "homepage should be https://rustuse.org for RustUse crates",
            ));
        }
    }

    issues
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        process,
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::{
        CrateMetadata, CrateName, crate_name_to_module_name, expected_docs_url,
        expected_repository_url, is_publishable, is_use_prefixed, is_valid_crate_name,
        module_name_to_crate_name, normalize_crate_name, validate_crate_metadata,
    };

    #[test]
    fn validates_crate_names_and_prefixes() {
        assert!(is_valid_crate_name("use-rust-release"));
        assert!(is_valid_crate_name("use_rust_release"));
        assert!(!is_valid_crate_name("Use-Rust-Release"));
        assert!(!is_valid_crate_name("use release"));
        assert!(is_use_prefixed("use-rust-release"));
        assert!(!is_use_prefixed("release-tools"));
    }

    #[test]
    fn converts_and_normalizes_names() {
        assert_eq!(
            crate_name_to_module_name("use-rust-release"),
            "use_rust_release"
        );
        assert_eq!(
            module_name_to_crate_name("use_rust_release"),
            "use-rust-release"
        );
        assert_eq!(
            normalize_crate_name(" Use Rust Release_tools "),
            "use-rust-release-tools"
        );
    }

    #[test]
    fn builds_expected_urls() {
        assert_eq!(
            expected_repository_url("use-rust-release").as_str(),
            "https://github.com/RustUse/use-rust-release"
        );
        assert_eq!(
            expected_docs_url("use-rust-release").as_str(),
            "https://docs.rs/use-rust-release"
        );
    }

    #[test]
    fn validates_metadata_defaults() {
        let metadata = CrateMetadata {
            name: CrateName::new("use-rust-release").expect("crate name should validate"),
            kind: super::CrateKind::Library,
            description: Some(String::from("release checks")),
            license: Some(String::from("MIT OR Apache-2.0")),
            repository: Some(expected_repository_url("use-rust-release")),
            documentation: Some(expected_docs_url("use-rust-release")),
            homepage: Some(String::from("https://rustuse.org")),
            publish_status: super::PublishStatus::Publishable,
        };

        assert!(validate_crate_metadata(&metadata).is_empty());
        assert!(is_publishable(&metadata));
    }

    #[test]
    fn builds_metadata_from_manifest_path() {
        let temp_dir = TestDir::new("crate-manifest");
        write_file(
            &temp_dir.path().join("Cargo.toml"),
            r#"[package]
    name = "use-rust-release"
version = "0.0.1"
edition = "2024"
description = "release checks"
license = "MIT OR Apache-2.0"
    repository = "https://github.com/RustUse/use-rust-release"
    documentation = "https://docs.rs/use-rust-release"
homepage = "https://rustuse.org"
"#,
        );
        write_file(
            &temp_dir.path().join("src").join("lib.rs"),
            "pub fn sample() {}\n",
        );

        let metadata =
            CrateMetadata::from_manifest_path(temp_dir.path()).expect("metadata should load");

        assert_eq!(metadata.name.as_str(), "use-rust-release");
        assert_eq!(metadata.kind, super::CrateKind::Library);
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
            path.push(format!("use-crate-{label}-{}-{nanos}", process::id()));
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
