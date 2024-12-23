use log::{info, debug, error};
use std::path::PathBuf;
use anyhow::{Result, Context};
use std::process::Command;
use crate::config::get_config;

pub async fn run_tests() -> Result<()> {
     info!("Running tests");
     let config = get_config()?;
   let test_path = PathBuf::from(&config.project_name);

    let mut cmd = Command::new("anchor");
     cmd.arg("test")
        .current_dir(test_path);
        debug!("Running command: {:?}", cmd);
     let output = cmd.output()
        .context("Failed to run tests")?;

    if !output.status.success() {
       let stderr = String::from_utf8_lossy(&output.stderr);
        error!("Tests failed: \n{}", stderr);
         anyhow::bail!("Tests failed")
     }
     info!("Tests run successfully");
    Ok(())
}