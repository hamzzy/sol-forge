use log::{info, debug};
use anyhow::{Result, Context};
use std::process::Command;

pub async fn start_devnet() -> Result<()> {
    info!("Starting Solana devnet");
     let mut cmd = Command::new("solana-test-validator");
     cmd.arg("--no-bpf-upgrade")
        .arg("--mint")
        .arg("1000000000");
      debug!("Running command: {:?}", cmd);

     let output = cmd.spawn()
        .context("Failed to start solana-test-validator")?;
      info!("Solana devnet started successfully");
      Ok(())
 }

pub async fn stop_devnet() -> Result<()> {
    info!("Stopping Solana devnet");
    //Not the best way to stop devnet. Requires manual kill or use some other lib to stop the process by finding the PID.
    info!("Solana devnet stop is not implemented");
    Ok(())
 }