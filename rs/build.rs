use std::error::Error;

const DIR: &str = "../proto";
const PROTOS: [&str; 6] = [
    "destination",
    "http_types",
    "identity",
    "inbound",
    "net",
    "tap",
];

fn main() -> Result<(), Box<dyn Error>> {
    let files = PROTOS
        .iter()
        .filter_map(|p| feature_proto(DIR, *p))
        .collect::<Vec<_>>();
    if files.is_empty() {
        return Err("no interfaces enabled".into());
    }

    let build_client = std::env::var_os("CARGO_FEATURE_CLIENT").is_some();
    let build_server = std::env::var_os("CARGO_FEATURE_SERVER").is_some();
    let build_arbitrary = std::env::var_os("CARGO_FEATURE_ARBITRARY").is_some();
    if !build_client && !build_server && !build_arbitrary {
        return Err(
            "either the `client`, `server`, or `arbitrary` features must be enabled".into(),
        );
    }
    tonic_build::configure()
        .build_client(build_client)
        .build_server(build_server)
        .compile(&*files, &[DIR.to_string()])?;

    // recompile protobufs only if any of the proto files changes.
    for file in files.iter() {
        println!("cargo:rerun-if-changed={}", file);
    }
    Ok(())
}

fn feature_proto(dir: &str, name: &str) -> Option<String> {
    let env = format!("CARGO_FEATURE_{}", name.to_uppercase());
    if std::env::var(env).is_ok() {
        Some(format!("{}/{}.proto", dir, name.to_lowercase()))
    } else {
        None
    }
}
