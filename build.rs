use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(&["src/track_data_types.proto", "src/easings.proto", "src/s2c_commands.proto", "src/c2s_commands.proto"], &["src/"])?;
    Ok(())
}