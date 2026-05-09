#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin umbrella re-exports for RustUse Rust ecosystem primitives.

pub use use_cargo;
pub use use_crate;
pub use use_release;
pub use use_version;

/// Commonly used RustUse Rust ecosystem primitives.
pub mod prelude {
    pub use use_cargo::{
        find_manifest, find_workspace_root, is_workspace, load_manifest, package_names,
        publishable_packages, workspace_members, CargoEdition, CargoManifest, CargoPackage,
        CargoWorkspace, ManifestPath, WorkspaceRoot,
    };
    pub use use_crate::{
        crate_name_to_module_name, expected_docs_url, expected_repository_url, is_use_prefixed,
        is_valid_crate_name, module_name_to_crate_name, normalize_crate_name,
        validate_crate_metadata, CrateMetadata, CrateName, DocumentationUrl, PublishStatus,
        RepositoryUrl,
    };
    pub use use_release::{ReleaseCheck, ReleaseIssue, ReleasePlan, ReleaseReport, ReleaseStatus};
    pub use use_version::{
        compare_versions, is_prerelease, next_major, next_minor, next_patch, parse_version,
        ReleaseLevel, Version, VersionBump, VersionPolicy,
    };
}

#[cfg(test)]
mod tests {
    use super::prelude::{expected_repository_url, next_patch, parse_version};

    #[test]
    fn reexports_common_helpers() {
        let version = parse_version("0.1.0").expect("version should parse");

        assert_eq!(
            expected_repository_url("use-release").as_str(),
            "https://github.com/RustUse/use-release"
        );
        assert_eq!(next_patch(&version).to_string(), "0.1.1");
    }
}
