#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Composable Cargo project and workspace primitives.

use std::{
    error::Error,
    fmt, fs,
    path::{Path, PathBuf},
};

use camino::{Utf8Path, Utf8PathBuf};
use cargo_metadata::MetadataCommand;
use serde::{Deserialize, Serialize};
use toml_edit::{DocumentMut, Item};

/// A UTF-8 `Cargo.toml` path.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ManifestPath(Utf8PathBuf);

impl ManifestPath {
    /// Returns the manifest path.
    #[must_use]
    pub fn as_path(&self) -> &Utf8Path {
        &self.0
    }
}

impl fmt::Display for ManifestPath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

/// A UTF-8 Cargo workspace root path.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceRoot(Utf8PathBuf);

impl WorkspaceRoot {
    /// Returns the workspace root path.
    #[must_use]
    pub fn as_path(&self) -> &Utf8Path {
        &self.0
    }
}

impl fmt::Display for WorkspaceRoot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.0.as_str())
    }
}

/// Known Cargo editions plus a fallback for unknown future values.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CargoEdition {
    E2015,
    E2018,
    E2021,
    E2024,
    Other(String),
}

impl CargoEdition {
    /// Parses a Cargo edition string.
    #[must_use]
    pub fn parse(value: &str) -> Self {
        match value {
            "2015" => Self::E2015,
            "2018" => Self::E2018,
            "2021" => Self::E2021,
            "2024" => Self::E2024,
            other => Self::Other(other.to_string()),
        }
    }

    /// Returns the edition as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::E2015 => "2015",
            Self::E2018 => "2018",
            Self::E2021 => "2021",
            Self::E2024 => "2024",
            Self::Other(value) => value,
        }
    }
}

impl fmt::Display for CargoEdition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A lightweight dependency description from `Cargo.toml`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CargoDependency {
    pub name: String,
    pub requirement: Option<String>,
    pub optional: bool,
}

/// A lightweight Cargo feature definition.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CargoFeature {
    pub name: String,
    pub members: Vec<String>,
}

/// A lightweight workspace package description.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CargoPackage {
    pub name: String,
    pub version: Option<String>,
    pub manifest_path: ManifestPath,
    pub publishable: bool,
}

/// A parsed Cargo manifest.
#[derive(Clone, Debug)]
pub struct CargoManifest {
    path: ManifestPath,
    document: DocumentMut,
}

impl CargoManifest {
    /// Reads and parses a `Cargo.toml` manifest.
    pub fn read(path: impl AsRef<Path>) -> Result<Self, CargoManifestError> {
        let manifest_path = resolve_manifest_path(path.as_ref())?;
        let contents = fs::read_to_string(manifest_path.as_path().as_std_path())?;
        let document = contents.parse::<DocumentMut>()?;

        Ok(Self {
            path: manifest_path,
            document,
        })
    }

    /// Returns the parsed manifest path.
    #[must_use]
    pub fn path(&self) -> &ManifestPath {
        &self.path
    }

    /// Returns `package.name` when present.
    #[must_use]
    pub fn package_name(&self) -> Option<&str> {
        self.package_str("name")
    }

    /// Returns `package.version` when present.
    #[must_use]
    pub fn package_version(&self) -> Option<&str> {
        self.package_str("version")
    }

    /// Returns `package.edition` when present.
    #[must_use]
    pub fn edition(&self) -> Option<CargoEdition> {
        self.package_str("edition").map(CargoEdition::parse)
    }

    /// Returns `package.repository` when present.
    #[must_use]
    pub fn repository(&self) -> Option<&str> {
        self.package_str("repository")
    }

    /// Returns `package.documentation` when present.
    #[must_use]
    pub fn documentation(&self) -> Option<&str> {
        self.package_str("documentation")
    }

    /// Returns `package.homepage` when present.
    #[must_use]
    pub fn homepage(&self) -> Option<&str> {
        self.package_str("homepage")
    }

    /// Returns `package.description` when present.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.package_str("description")
    }

    /// Returns `package.license` when present.
    #[must_use]
    pub fn license(&self) -> Option<&str> {
        self.package_str("license")
    }

    /// Returns `package.readme` when present.
    #[must_use]
    pub fn readme(&self) -> Option<&str> {
        self.package_str("readme")
    }

    /// Returns `true` when the manifest contains a `[workspace]` table.
    #[must_use]
    pub fn is_workspace(&self) -> bool {
        self.document.get("workspace").is_some()
    }

    /// Returns the workspace members declared in `[workspace].members`.
    #[must_use]
    pub fn workspace_members(&self) -> Vec<String> {
        self.document
            .get("workspace")
            .and_then(Item::as_table_like)
            .and_then(|workspace| workspace.get("members"))
            .and_then(Item::as_value)
            .and_then(|value| value.as_array())
            .map(|members| {
                members
                    .iter()
                    .filter_map(|value| value.as_str().map(ToOwned::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns dependency entries from `[dependencies]`.
    #[must_use]
    pub fn dependencies(&self) -> Vec<CargoDependency> {
        self.document
            .get("dependencies")
            .and_then(Item::as_table_like)
            .map(|dependencies| {
                dependencies
                    .iter()
                    .map(|(name, item)| CargoDependency {
                        name: name.to_string(),
                        requirement: dependency_requirement(item),
                        optional: dependency_optional(item),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns feature entries from `[features]`.
    #[must_use]
    pub fn features(&self) -> Vec<CargoFeature> {
        self.document
            .get("features")
            .and_then(Item::as_table_like)
            .map(|features| {
                features
                    .iter()
                    .map(|(name, item)| CargoFeature {
                        name: name.to_string(),
                        members: item
                            .as_value()
                            .and_then(|value| value.as_array())
                            .map(|values| {
                                values
                                    .iter()
                                    .filter_map(|value| value.as_str().map(ToOwned::to_owned))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Returns `true` when the package should be considered publishable.
    #[must_use]
    pub fn is_publishable(&self) -> bool {
        match self.package_item("publish") {
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

    fn package_item(&self, field: &str) -> Option<&Item> {
        self.document
            .get("package")
            .and_then(Item::as_table_like)
            .and_then(|package| package.get(field))
    }

    fn package_str(&self, field: &str) -> Option<&str> {
        self.package_item(field)
            .and_then(Item::as_value)
            .and_then(|value| value.as_str())
    }
}

/// A discovered Cargo workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CargoWorkspace {
    root: WorkspaceRoot,
    members: Vec<CargoPackage>,
}

impl CargoWorkspace {
    /// Discovers a workspace from a starting path.
    pub fn discover(start: impl AsRef<Path>) -> Result<Self, CargoWorkspaceError> {
        let root = find_workspace_root(start)?;
        let metadata = MetadataCommand::new()
            .current_dir(root.as_path().as_std_path())
            .no_deps()
            .exec()?;

        let mut members = metadata
            .packages
            .iter()
            .filter(|package| metadata.workspace_members.contains(&package.id))
            .map(CargoPackage::from_metadata_package)
            .collect::<Result<Vec<_>, _>>()?;

        members.sort_by(|left, right| left.name.cmp(&right.name));

        Ok(Self { root, members })
    }

    /// Returns the workspace root.
    #[must_use]
    pub fn root(&self) -> &WorkspaceRoot {
        &self.root
    }

    /// Returns the discovered workspace members.
    #[must_use]
    pub fn members(&self) -> &[CargoPackage] {
        &self.members
    }
}

impl CargoPackage {
    fn from_metadata_package(
        package: &cargo_metadata::Package,
    ) -> Result<Self, CargoWorkspaceError> {
        let manifest = CargoManifest::read(package.manifest_path.as_std_path())?;

        Ok(Self {
            name: package.name.clone(),
            version: Some(package.version.to_string()),
            manifest_path: ManifestPath(package.manifest_path.clone()),
            publishable: manifest.is_publishable(),
        })
    }
}

/// Errors that can occur while working with `Cargo.toml` manifests.
#[derive(Debug)]
pub enum CargoManifestError {
    Io(std::io::Error),
    NotFound(PathBuf),
    NonUtf8Path(PathBuf),
    ParseToml(toml_edit::TomlError),
}

impl fmt::Display for CargoManifestError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "failed to read Cargo manifest: {error}"),
            Self::NotFound(path) => write!(formatter, "no Cargo.toml found at {}", path.display()),
            Self::NonUtf8Path(path) => {
                write!(formatter, "path is not valid UTF-8: {}", path.display())
            },
            Self::ParseToml(error) => write!(formatter, "failed to parse Cargo manifest: {error}"),
        }
    }
}

impl Error for CargoManifestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::ParseToml(error) => Some(error),
            Self::NotFound(_) | Self::NonUtf8Path(_) => None,
        }
    }
}

impl From<std::io::Error> for CargoManifestError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<toml_edit::TomlError> for CargoManifestError {
    fn from(error: toml_edit::TomlError) -> Self {
        Self::ParseToml(error)
    }
}

/// Errors that can occur while discovering a Cargo workspace.
#[derive(Debug)]
pub enum CargoWorkspaceError {
    Manifest(CargoManifestError),
    Metadata(cargo_metadata::Error),
    NonUtf8Path(PathBuf),
    NotFound(PathBuf),
}

impl fmt::Display for CargoWorkspaceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Manifest(error) => write!(formatter, "failed to inspect manifest: {error}"),
            Self::Metadata(error) => write!(formatter, "failed to query cargo metadata: {error}"),
            Self::NonUtf8Path(path) => {
                write!(formatter, "path is not valid UTF-8: {}", path.display())
            },
            Self::NotFound(path) => {
                write!(formatter, "no workspace root found from {}", path.display())
            },
        }
    }
}

impl Error for CargoWorkspaceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Manifest(error) => Some(error),
            Self::Metadata(error) => Some(error),
            Self::NonUtf8Path(_) | Self::NotFound(_) => None,
        }
    }
}

impl From<CargoManifestError> for CargoWorkspaceError {
    fn from(error: CargoManifestError) -> Self {
        match error {
            CargoManifestError::NotFound(path) => Self::NotFound(path),
            other => Self::Manifest(other),
        }
    }
}

impl From<cargo_metadata::Error> for CargoWorkspaceError {
    fn from(error: cargo_metadata::Error) -> Self {
        Self::Metadata(error)
    }
}

/// Finds the nearest `Cargo.toml` relative to a starting path.
pub fn find_manifest(start: impl AsRef<Path>) -> Result<ManifestPath, CargoManifestError> {
    let original = start.as_ref().to_path_buf();
    let mut current = normalize_search_start(start.as_ref());

    loop {
        let candidate = current.join("Cargo.toml");

        if candidate.is_file() {
            return to_manifest_path(candidate);
        }

        let Some(parent) = current.parent() else {
            return Err(CargoManifestError::NotFound(original));
        };

        current = parent.to_path_buf();
    }
}

/// Finds the nearest Cargo workspace root relative to a starting path.
pub fn find_workspace_root(start: impl AsRef<Path>) -> Result<WorkspaceRoot, CargoWorkspaceError> {
    let original = start.as_ref().to_path_buf();
    let mut current = normalize_search_start(start.as_ref());

    loop {
        let manifest_path = current.join("Cargo.toml");

        if manifest_path.is_file() {
            let manifest = CargoManifest::read(&manifest_path)?;
            if manifest.is_workspace() {
                return to_workspace_root(current);
            }
        }

        let Some(parent) = current.parent() else {
            return Err(CargoWorkspaceError::NotFound(original));
        };

        current = parent.to_path_buf();
    }
}

/// Loads a `Cargo.toml` manifest from a file or directory path.
pub fn load_manifest(path: impl AsRef<Path>) -> Result<CargoManifest, CargoManifestError> {
    CargoManifest::read(path)
}

/// Returns `true` when a manifest contains a `[workspace]` table.
#[must_use]
pub fn is_workspace(manifest: &CargoManifest) -> bool {
    manifest.is_workspace()
}

/// Returns workspace members from a starting path.
pub fn workspace_members(
    start: impl AsRef<Path>,
) -> Result<Vec<CargoPackage>, CargoWorkspaceError> {
    Ok(CargoWorkspace::discover(start)?.members)
}

/// Returns workspace package names from a starting path.
pub fn package_names(start: impl AsRef<Path>) -> Result<Vec<String>, CargoWorkspaceError> {
    let mut names = workspace_members(start)?
        .into_iter()
        .map(|package| package.name)
        .collect::<Vec<_>>();
    names.sort();
    Ok(names)
}

/// Returns publishable workspace packages from a starting path.
pub fn publishable_packages(
    start: impl AsRef<Path>,
) -> Result<Vec<CargoPackage>, CargoWorkspaceError> {
    Ok(workspace_members(start)?
        .into_iter()
        .filter(|package| package.publishable)
        .collect())
}

fn resolve_manifest_path(path: &Path) -> Result<ManifestPath, CargoManifestError> {
    let candidate = if path.is_dir() {
        path.join("Cargo.toml")
    } else {
        path.to_path_buf()
    };

    if candidate.is_file() {
        to_manifest_path(candidate)
    } else {
        Err(CargoManifestError::NotFound(candidate))
    }
}

fn normalize_search_start(path: &Path) -> PathBuf {
    if path.is_dir() {
        return path.to_path_buf();
    }

    path.parent()
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

fn to_manifest_path(path: PathBuf) -> Result<ManifestPath, CargoManifestError> {
    let utf8 = Utf8PathBuf::from_path_buf(path.clone()).map_err(CargoManifestError::NonUtf8Path)?;
    Ok(ManifestPath(utf8))
}

fn to_workspace_root(path: PathBuf) -> Result<WorkspaceRoot, CargoWorkspaceError> {
    let utf8 =
        Utf8PathBuf::from_path_buf(path.clone()).map_err(CargoWorkspaceError::NonUtf8Path)?;
    Ok(WorkspaceRoot(utf8))
}

fn dependency_requirement(item: &Item) -> Option<String> {
    item.as_value()
        .and_then(|value| value.as_str())
        .map(ToOwned::to_owned)
        .or_else(|| {
            item.as_table_like()
                .and_then(|table| table.get("version"))
                .and_then(Item::as_value)
                .and_then(|value| value.as_str())
                .map(ToOwned::to_owned)
        })
}

fn dependency_optional(item: &Item) -> bool {
    item.as_table_like()
        .and_then(|table| table.get("optional"))
        .and_then(Item::as_value)
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
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
        CargoEdition, CargoManifest, CargoWorkspace, find_manifest, find_workspace_root,
        package_names, publishable_packages, workspace_members,
    };

    #[test]
    fn reads_manifest_metadata_dependencies_and_features() {
        let temp_dir = TestDir::new("manifest-read");
        write_file(
            &temp_dir.path().join("Cargo.toml"),
            r#"[package]
name = "use-demo"
version = "0.1.0"
edition = "2021"
description = "demo crate"
license = "MIT OR Apache-2.0"
repository = "https://github.com/RustUse/use-demo"
documentation = "https://docs.rs/use-demo"
homepage = "https://rustuse.org"
readme = "README.md"

[dependencies]
serde = { version = "1", optional = true }
semver = "1"

[features]
default = ["serde"]
"#,
        );

        let manifest = CargoManifest::read(temp_dir.path()).expect("manifest should parse");

        assert_eq!(manifest.package_name(), Some("use-demo"));
        assert_eq!(manifest.package_version(), Some("0.1.0"));
        assert_eq!(manifest.edition(), Some(CargoEdition::E2021));
        assert_eq!(
            manifest.repository(),
            Some("https://github.com/RustUse/use-demo")
        );
        assert_eq!(manifest.dependencies().len(), 2);
        assert_eq!(manifest.features().len(), 1);
        assert!(manifest.is_publishable());
    }

    #[test]
    fn finds_manifest_and_workspace_roots() {
        let temp_dir = TestDir::new("workspace-find");
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
repository = "https://github.com/RustUse/use-demo"
"#,
        );

        let nested = temp_dir.path().join("crates").join("use-demo").join("src");
        fs::create_dir_all(&nested).expect("nested directory should be created");

        let manifest = find_manifest(&nested).expect("manifest should be found");
        let root = find_workspace_root(&nested).expect("workspace root should be found");

        assert!(manifest.as_path().ends_with("crates/use-demo/Cargo.toml"));
        assert_eq!(root.as_path().as_std_path(), temp_dir.path());
    }

    #[test]
    fn discovers_workspace_members_and_publishable_packages() {
        let temp_dir = TestDir::new("workspace-members");
        write_file(
            &temp_dir.path().join("Cargo.toml"),
            r#"[workspace]
members = ["crates/use-one", "crates/use-two"]
"#,
        );
        write_file(
            &temp_dir
                .path()
                .join("crates")
                .join("use-one")
                .join("Cargo.toml"),
            r#"[package]
name = "use-one"
version = "0.1.0"
edition = "2021"
repository = "https://github.com/RustUse/use-one"
"#,
        );
        write_file(
            &temp_dir
                .path()
                .join("crates")
                .join("use-one")
                .join("src")
                .join("lib.rs"),
            "pub fn sample() {}\n",
        );
        write_file(
            &temp_dir
                .path()
                .join("crates")
                .join("use-two")
                .join("Cargo.toml"),
            r#"[package]
name = "use-two"
version = "0.1.0"
edition = "2021"
repository = "https://github.com/RustUse/use-two"
publish = false
"#,
        );
        write_file(
            &temp_dir
                .path()
                .join("crates")
                .join("use-two")
                .join("src")
                .join("lib.rs"),
            "pub fn sample() {}\n",
        );

        let workspace = CargoWorkspace::discover(temp_dir.path()).expect("workspace should load");
        let names = package_names(temp_dir.path()).expect("package names should load");
        let publishable =
            publishable_packages(temp_dir.path()).expect("publishable packages should load");
        let members = workspace_members(temp_dir.path()).expect("workspace members should load");

        assert_eq!(workspace.members().len(), 2);
        assert_eq!(members.len(), 2);
        assert_eq!(
            names,
            vec![String::from("use-one"), String::from("use-two")]
        );
        assert_eq!(publishable.len(), 1);
        assert_eq!(publishable[0].name, "use-one");
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
            path.push(format!("use-rust-cargo-{label}-{}-{nanos}", process::id()));
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
