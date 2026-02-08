use std::path::Path;

use clap::Parser;
use color_eyre::eyre::bail;

mod cli;
mod cmd;

use crate::cli::{Cli, Commands};

fn validate_output(output: &Path) -> color_eyre::Result<()> {
    if output.is_dir() {
        bail!("output must be a file. got dir `{}`", output.display());
    } else if output.extension().is_some_and(|ext| ext != "svg") {
        bail!("output must end in .svg. got `{}`", output.display());
    }
    Ok(())
}

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
            validate_output(&output)?;
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
        Commands::Contributions { output } => {
            validate_output(&output)?;
            cmd::contributions::generate(
                cli.github_username,
                cli.skip_private,
                cli.skip_forks,
                cli.skip_archived,
                output,
            )
            .await
        }
    }
}
