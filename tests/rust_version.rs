#![cfg(feature = "std")]
use pipe_trait::Pipe;
use pretty_assertions::assert_eq;
use semver::Version;
use std::{fs, path::Path};

#[test]
fn sync() {
    let rust_toolchain = env!("CARGO_MANIFEST_DIR")
        .pipe(Path::new)
        .join("rust-toolchain")
        .pipe(fs::read_to_string)
        .expect("the rust-toolchain file should be readable")
        .trim()
        .pipe(Version::parse)
        .expect("the content rust-toolchain should be valid semver");
    dbg!(&rust_toolchain);
    let Version { major, minor, .. } = &rust_toolchain;
    let rust_version = env!("CARGO_PKG_RUST_VERSION");
    dbg!(rust_version);
    assert_eq!(rust_version, format!("{major}.{minor}"));
}
