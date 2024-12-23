use log::{info, debug, error};
    use std::path::PathBuf;
    use anyhow::{Result, Context};
    use std::process::Command;
    use crate::config::get_config;

    pub async fn deploy_program() -> Result<()> {
        info!("Deploying program");
        let config = get_config()?;
       let program_path = PathBuf::from(&config.project_name).join("programs").join("my-anchor-program");
        let so_path =  program_path.join("target/deploy/my_anchor_program.so");
        if !so_path.exists() {
           anyhow::bail!("Can't find the program .so file {}", so_path.display());
        }

        let mut cmd = Command::new("solana");
        cmd.arg("deploy")
           .arg(so_path);

        if let Some(keypair_path) = &config.program_keypair_path {
             cmd.arg("--keypair")
                .arg(keypair_path);
        }
         if let Some(cluster_url) = &config.cluster_url {
             cmd.arg("--url")
                .arg(cluster_url);
         }
           debug!("Running command: {:?}", cmd);
        let output = cmd.output()
            .context("Failed to deploy program")?;
        if !output.status.success() {
           let stderr = String::from_utf8_lossy(&output.stderr);
            error!("Failed to deploy program: \n{}", stderr);
             anyhow::bail!("Deployment failed")
         }
        info!("Program deployed successfully");
        Ok(())
     }