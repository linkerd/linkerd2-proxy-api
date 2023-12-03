use std::{fs, path::PathBuf};

/// Read protobuf definitions from `./proto` to generate sources into `./src/gen`.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // List all protobuf definitions.
    let proto_dir = PathBuf::from("proto");
    let mut proto_files = Vec::new();
    for ent in fs::read_dir(&*proto_dir)? {
        let ent = ent?;
        if ent.metadata()?.is_file() {
            let path = ent.path();
            if path.extension().and_then(|e| e.to_str()) == Some("proto") {
                proto_files.push(path);
            }
        }
    }

    // Generate rust bindings.
    let out_dir = PathBuf::from("src").join("gen");
    if let Err(e) = tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .out_dir(out_dir)
        .compile(&proto_files, &[proto_dir])
    {
        eprintln!("{}", e.to_string().replace("\\n", "\n").trim_end());
        std::process::exit(1);
    }

    Ok(())
}
