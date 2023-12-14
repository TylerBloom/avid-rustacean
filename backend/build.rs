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
    {
        // Install the `wasm32-unknown-unknown` target
        if !std::process::Command::new("rustup")
            .args(["target", "add", "wasm32-unknown-unknown"])
            .status()
            .expect("failed to run rustup")
            .success()
        {
            panic!("failed to install wasm32 target")
        }

        // Install `trunk` to compile the frontend
        if !std::process::Command::new("cargo")
            .args(["install", "trunk"])
            .status()
            .expect("failed to run cargo install")
            .success()
        {
            panic!("failed to install trunk")
        }
        compile_fe()?;
    } else if env::var("PROFILE")
        .map(|v| v == "release")
        .unwrap_or_default()
    {
        compile_fe()?;
    }
    Ok(())
}

fn compile_fe() -> Result<(), i32> {
    // Calls trunk to compile the frontend
    let mut cmd = Command::new("trunk");
    cmd.args(["build", "-d", "../assets", "--filehash", "false"]);

    cmd.arg("--release");
    cmd.arg("../frontend/index.html");
    if cmd.status().is_err() {
        return Err(1);
    }

    // Compresses the WASM module
    let mut wasm_file =
        File::open("../assets/avid-rustacean-frontend_bg.wasm").expect("Failed to open WASM file");
    let mut wasm_data = Vec::new();
    wasm_file.read_to_end(&mut wasm_data).unwrap();

    let output_file = File::create("../assets/avid-rustacean-frontend_bg.wasm.gz").unwrap();
    let mut encoder = GzEncoder::new(BufWriter::new(output_file), Compression::default());
    encoder.write_all(&wasm_data).map_err(|_| 1)?;
    Ok(())
}
