use clap::Parser;
use std::path::PathBuf;

use gtk::prelude::*;

mod config;
mod gui;

#[derive(Parser, Clone)]
#[command(name = "Runex")]
#[command(version = "0.1")]
#[command(about = "An application launcher inspired by Powertoys Run", long_about = None)]
pub struct CliArgs {
    /// Set the configuration file path to use
    #[arg(long)]
    config: Option<PathBuf>,

    /// Set the stylesheet file path to use
    #[arg(long)]
    style: Option<PathBuf>,
}

const APP_ID: &str = "com.beenman.runex";

fn main() -> Result<glib::ExitCode, anyhow::Error> {
    let cli_args = CliArgs::parse();
    let config = match &cli_args.config {
        Some(path) => config::from_file(path)?,
        None => config::from_env_or_home()?,
    };

    dbg!(&config);

    let app = gui::build_application(APP_ID, cli_args.clone(), config.clone())?;

    Ok(app.run())
}
