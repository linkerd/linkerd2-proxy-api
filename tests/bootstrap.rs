//! A test that regenerates the Rust protobuf bindings.
//!
//! It can be run via:
//!
//! ```no_run
//! cargo test --test=bootstrap
//! ```

/// Generates protobuf bindings into src/gen and fails if the generated files do
/// not match those that are already checked into git
#[test]
fn bootstrap() {
    let out_dir = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("gen");
    generate(&*out_dir);
    if changed(&*out_dir) {
        panic!("protobuf interfaces do not match generated sources");
    }
}

/// Generates protobuf bindings into the given directory
fn generate(out_dir: &std::path::Path) {
    let iface_files = &[
        "proto/destination.proto",
        "proto/http_types.proto",
        "proto/identity.proto",
        "proto/inbound.proto",
        "proto/net.proto",
        "proto/tap.proto",
    ];
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir(out_dir)
        .compile(iface_files, &["proto"])
        .expect("failed to compile protobuf");
}

/// Returns true if the given path contains files that have changed since the
/// last Git commit
fn changed(path: &std::path::Path) -> bool {
    let status = std::process::Command::new("git")
        .arg("diff")
        .arg("--exit-code")
        .arg("--")
        .arg(path)
        .status()
        .expect("failed to run git");
    !status.success()
}
