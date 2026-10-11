fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=HERDR_BUILD_CHANNEL");
    println!("cargo:rerun-if-env-changed=HERDR_BUILD_ID");
    println!("cargo:rerun-if-env-changed=HERDR_BUILD_COMMIT");

    // The fork's release version lives in RELEASE_VERSION (repo root), kept
    // separate from Cargo.toml (the upstream runtime version). Expose it to the
    // crate so `--version` can report the fork identity and its own release.
    // Release tooling that knows the published version before the file is
    // pinned (packaging/release.sh, the macOS CI job, the AUR PKGBUILD) passes
    // it in HERDR_RELEASE_VERSION; the file is the fallback for local builds.
    println!("cargo:rerun-if-changed=RELEASE_VERSION");
    println!("cargo:rerun-if-env-changed=HERDR_RELEASE_VERSION");
    let release = std::env::var("HERDR_RELEASE_VERSION")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(|| {
            std::fs::read_to_string("RELEASE_VERSION")
                .ok()
                .map(|value| value.trim().to_string())
                .filter(|value| !value.is_empty())
        })
        .unwrap_or_else(|| "0.0.0".to_string());
    println!("cargo:rustc-env=HERDR_RELEASE_VERSION={release}");
}
