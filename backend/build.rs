use std::{
    env,
    fs::File,
    io::{BufWriter, Read, Write},
    process::Command,
};

use flate2::{write::GzEncoder, Compression};

fn main() -> Result<(), i32> {
    // Install external dependency (in the shuttle container only)
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
        || env::var("PROFILE")
            .map(|v| v == "release")
            .unwrap_or_default()
    {
        compile_fe();
    }
    Ok(())
}

fn compile_fe() {
    // Calls trunk to compile the frontend
    let mut cmd = Command::new("trunk");
    cmd.args(["build", "-d", "../assets", "--filehash", "false"]);

    cmd.arg("--release");
    cmd.arg("../frontend/index.html");
    if let Err(e) = cmd.status() {
        panic!("Failed to compile frontend!\n{e}");
    }

    // Compresses the WASM module
    let mut wasm_file =
        File::open("../assets/avid-rustacean-frontend_bg.wasm").expect("Failed to open WASM file");
    let mut wasm_data = Vec::new();
    wasm_file.read_to_end(&mut wasm_data).unwrap();

    let output_file = File::create("../assets/avid-rustacean-frontend_bg.wasm.gz").unwrap();
    let mut encoder = GzEncoder::new(BufWriter::new(output_file), Compression::default());
    if let Err(e) = encoder.write_all(&wasm_data).map_err(|_| 1) {
        panic!("Failed to compress WASM module!\n{e}");
    }
}
