use std::io::Result;

fn main() -> Result<()> {
    prost_build::Config::new()
        .compile_well_known_types()
        .compile_protos(
            &["src/proto/google/protobuf/compiler/plugin.proto"],
            &["src/proto/"],
        )?;
    Ok(())
}
