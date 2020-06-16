use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    build_control()?;
    Ok(())
}

fn build_control() -> Result<(), Box<dyn Error>> {
    let iface_files = &[
        "../proto/destination.proto",
        "../proto/http_types.proto",
        "../proto/identity.proto",
        "../proto/net.proto",
        "../proto/tap.proto",
    ];
    let dirs = &["../proto"];

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .compile(iface_files, dirs)?;

    // recompile protobufs only if any of the proto files changes.
    for file in iface_files {
        println!("cargo:rerun-if-changed={}", file);
    }

    Ok(())
}
