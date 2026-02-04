use clap::Parser;

mod cli;
mod cmd;

use crate::cli::{Cli, Commands};

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Languages {
            skipped_languages,
            num_languages,
            output,
        } => {
            cmd::languages::generate(
                cli.github_username,
                cli.skip_private,
                cli.skip_forks,
                cli.skip_archived,
                skipped_languages,
                num_languages,
                output,
            )
            .await
        }
    }
}
