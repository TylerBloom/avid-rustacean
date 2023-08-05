/* This build script ensure that everything needed to run the SquireCore server is in its place.
 * Primarily, this includes the static assets for the frontend, including the index, wasm app, and
 * JS bindings. Trunk is used to compile and generate the app and the JS bindings.
 */

use std::{env, process::Command};

fn main() -> Result<(), i32> {
    let wd = env::var("CARGO_MANIFEST_DIR").unwrap();
    let fe_path = format!("{wd}/../frontend");

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
            panic!("failed to install rustup")
        }

        // Install `trunk` to compile the frontend
        if !std::process::Command::new("cargo")
            .args(["install", "trunk"])
            .status()
            .expect("failed to run rustup")
            .success()
        {
            panic!("failed to install rustup")
        }
    }

    let mut cmd = Command::new("trunk");
    cmd.args(["build", "-d", "../assets", "--filehash", "false"]);

    if Ok("release".to_owned()) == env::var("PROFILE") {
        cmd.arg("--release");
    }
    cmd.arg(format!("{fe_path}/index.html"));
    match cmd.status().map(|s| s.success()) {
        Ok(false) | Err(_) => return Err(1),
        _ => {}
    }
    println!("cargo:rerun-if-changed={fe_path}");
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
