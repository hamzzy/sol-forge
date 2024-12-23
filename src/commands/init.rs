use log::{ info, error };
use std::path::PathBuf;
use anyhow::{ Result, Context };
use std::fs;
use crate::config::AnchorforgeConfig;
use toml;

pub async fn init_project(project_name: String) -> Result<()> {
    info!("Initializing new Anchor project: {}", project_name);
    let project_path = PathBuf::from(&project_name);
    if project_path.exists() {
        error!("Project with the name '{}' already exists.", project_name);
        anyhow::bail!("Project already exists");
    }

    fs
        ::create_dir_all(&project_path)
        .with_context(|| format!("Failed to create project directory: {}", project_name))?;

    // Create a basic anchor project structure
    create_basic_anchor_project_structure(&project_path)?;

    //create config
    let config = AnchorforgeConfig {
        project_name: project_name.clone(),
        program_keypair_path: None,
        cluster_url: None,
    };
    let config_str = toml::to_string(&config).context("Failed to serialize config")?;
    fs
        ::write(project_path.join("anchorforge.toml"), config_str)
        .with_context(|| "Failed to write config file")?;

    info!("Anchor project '{}' initialized successfully.", project_name);
    Ok(())
}
fn create_basic_anchor_project_structure(project_path: &PathBuf) -> Result<()> {
    // Create a basic anchor project structure
    let programs_dir = project_path.join("programs");
    fs::create_dir_all(&programs_dir).context("Failed to create programs directory")?;

    let tests_dir = project_path.join("tests");
    fs::create_dir_all(&tests_dir).context("Failed to create tests directory")?;

    let migrations_dir = project_path.join("migrations");
    fs::create_dir_all(&migrations_dir).context("Failed to create migrations directory")?;

    fs
        ::create_dir_all(programs_dir.join("my-anchor-program"))
        .context("Failed to create program directory")?;
    fs
        ::write(
            programs_dir.join("my-anchor-program").join("Cargo.toml"),
            r#"[package]

        Use code with caution.Rust

        name = "my-anchor-program"
        version = "0.1.0"
        description = "Created with AnchorForge"
        edition = "2021"

        [lib]
        crate-type = ["cdylib", "rlib"]

        [dependencies]
        anchor-lang = "0.27.0"

        [features]
        no-entrypoint = []
        no-idl = []
        no-log-ix-name = []
        "#
        )
        .context("Failed to write program's Cargo.toml")?;

    fs
        ::write(
            programs_dir.join("my-anchor-program").join("src").join("lib.rs"),
            r#"use anchor_lang::prelude::*;
    

        Use code with caution.

        declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

        #[program]
        pub mod my_anchor_program {
        use super::*;
        pub fn initialize(ctx: Context<Initialize>, data: u64) -> Result<()> {
        msg!("Hello, Anchor!");
        ctx.accounts.my_account.data = data;
        Ok(())
        }
        }

        #[derive(Accounts)]
        pub struct Initialize<'info> {
        #[account(init, payer = user, space = 8 + 8)]
        pub my_account: Account<'info, MyAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
        }

        #[account]
        pub struct MyAccount {
        pub data: u64,
        }
        "#
        )
        .context("Failed to write program's lib.rs")?;
    Ok(())
}
