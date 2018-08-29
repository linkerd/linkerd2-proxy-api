extern crate tower_grpc_build;

fn main() {
    build_control();
}

fn build_control() {
    let iface_files = &[
        "../proto/destination.proto",
        "../proto/http_types.proto",
        "../proto/net.proto",
        "../proto/tap.proto"
    ];
    let dirs = &["../proto"];

    tower_grpc_build::Config::new()
        .enable_client(true)
        .enable_server(true)
        .build(iface_files, dirs)
        .unwrap();

    // recompile protobufs only if any of the proto files changes.
    for file in iface_files {
        println!("cargo:rerun-if-changed={}", file);
    }
}
