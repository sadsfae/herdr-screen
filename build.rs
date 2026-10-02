fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=HERDR_BUILD_CHANNEL");
    println!("cargo:rerun-if-env-changed=HERDR_BUILD_ID");
    println!("cargo:rerun-if-env-changed=HERDR_BUILD_COMMIT");

    // The fork's release version lives in RELEASE_VERSION (repo root), kept
    // separate from Cargo.toml (the upstream runtime version). Expose it to the
    // crate so `--version` can report the fork identity and its own release.
    println!("cargo:rerun-if-changed=RELEASE_VERSION");
    let release = std::fs::read_to_string("RELEASE_VERSION")
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "0.0.0".to_string());
    println!("cargo:rustc-env=HERDR_RELEASE_VERSION={release}");
}
