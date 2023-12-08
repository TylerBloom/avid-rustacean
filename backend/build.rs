use std::{env, process::Command};

fn main() -> Result<(), i32> {
    let wd = env::var("CARGO_MANIFEST_DIR").unwrap();

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
        compile_fe(&wd)?;
    } else if env::var("PROFILE")
        .map(|v| v == "release")
        .unwrap_or_default()
    {
        compile_fe(&wd)?;
    }
    Ok(())
}

fn compile_fe(_wd: &str) -> Result<(), i32> {
    let mut cmd = Command::new("trunk");
    cmd.args(["build", "-d", "../assets", "--filehash", "false"]);

    cmd.arg("--release");
    cmd.arg("../frontend/index.html");
    match cmd.status() {
        Ok(_) => Ok(()),
        Err(_) => Err(1),
    }
}
