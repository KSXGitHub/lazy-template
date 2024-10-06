#![cfg(feature = "std")]
use cargo_metadata::MetadataCommand;
use cargo_toml::Manifest;
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use semver::Version;
use std::fs;

#[test]
fn sync() {
    let ws_root = MetadataCommand::new()
        .exec()
        .expect("the command `cargo metadata` should execute successfully")
        .workspace_root;
    dbg!(&ws_root);
    let rust_toolchain = ws_root
        .join("rust-toolchain")
        .pipe(fs::read_to_string)
        .expect("the rust-toolchain file should be readable")
        .trim()
        .pipe(Version::parse)
        .expect("the content rust-toolchain should be valid semver");
    dbg!(&rust_toolchain);
    let Version { major, minor, .. } = &rust_toolchain;
    let rust_version = ws_root
        .join("Cargo.toml")
        .pipe(Manifest::from_path)
        .expect("Cargo.toml should be readable")
        .package
        .expect("package should be defined")
        .rust_version
        .expect("rust_version should be defined");
    let rust_version = rust_version
        .get()
        .expect("rust-version should be valid")
        .as_str();
    dbg!(rust_version);
    assert_eq!(rust_version, format!("{major}.{minor}"));
}
