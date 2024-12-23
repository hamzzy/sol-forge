use log::{info, debug, error};
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::process::Command;
use crate::config::get_config;
pub async fn compile_program() -> Result<()> {
    info!("Compiling Anchor program");
    // First check if there is a config file
    let config = get_config()?;

    let program_path = PathBuf::from(&config.project_name).join("programs").join("my-anchor-program");
    if !program_path.exists() {
        anyhow::bail!("Can't find the program directory {}", program_path.display());
    }
    let mut cmd = Command::new("cargo");
    cmd.arg("build-bpf").current_dir(program_path);
    debug!("Running command: {:?}", cmd);

    let output = cmd.output()
       .context("Failed to execute cargo build-bpf")?;

     if !output.status.success() {
       let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Failed to compile program: \n{}", stderr);
        anyhow::bail!("Compilation failed")
     }
    info!("Anchor program compiled successfully");
    Ok(())
}