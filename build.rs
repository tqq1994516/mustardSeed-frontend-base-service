// use std::env;

fn main() -> Result<(), std::io::Error> {
    // env::set_var("OUT_DIR", "src");
    tonic_build::configure().build_server(true).compile(
        &[
            "proto/frontend_base_service.proto",
        ],
        &["."],
    )?;
    Ok(())
}