use airnope::{telegram, telegram::AirNope};
use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use std::env;

mod bench;
mod cache;
mod cli;
mod demo;
mod repl;

const DEFAULT_LOG_LEVEL: &str = "INFO";

fn init_log() -> Result<()> {
    let mut default_used = false;
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", DEFAULT_LOG_LEVEL);
        default_used = true;
    }
    pretty_env_logger::init();
    if default_used {
        log::info!(
            "No RUST_LOG environment variable found, using default log level: {}",
            DEFAULT_LOG_LEVEL
        );
    }
    Ok(())
}

fn detect_mode(arg: Option<AirNope>) -> AirNope {
    if let Some(mode) = arg {
        return mode;
    }
    if env::var("HOST").is_ok() && env::var("PORT").is_ok() {
        log::info!("HOST and PORT are set, so starting as webhook server");
        AirNope::Webhook
    } else {
        AirNope::LongPooling
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    init_log()?;
    let args = Cli::parse();
    match args.command {
        Commands::Bot { mode, shutdown_in } => telegram::run(detect_mode(mode), shutdown_in).await,
        Commands::Repl => repl::run().await,
        Commands::Demo => demo::run().await,
        Commands::Download => cache::download_all().await,
        Commands::Bench { label } => bench::run(label).await,
        Commands::CleanCache { dry_run } => cache::clean_rust_bert_cache(dry_run).await,
    }
}
