use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub github_username: String,

    /// don't include repos that are forks
    #[arg(long, default_value_t = true)]
    pub skip_forks: bool,

    /// don't include private repos
    #[arg(long, default_value_t = false)]
    pub skip_private: bool,

    /// don't include archived repos
    #[arg(long, default_value_t = false)]
    pub skip_archived: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// pie chart of top used languages in user's repos
    Languages {
        /// don't include these languages
        #[arg(short, long, value_delimiter = ',')]
        skipped_languages: Vec<String>,

        /// how many languages to show
        /// the rest will be merged into "Other"
        /// 0 means infinite
        #[arg(short, long, default_value_t = 5)]
        num_languages: usize,

        /// where to put output svg
        /// `-` to use stdout
        #[arg(short, long, default_value = "languages.svg")]
        output: PathBuf,
    },
    /// general stats of forks, PRs, issues
    Contributions {
        #[arg(short, long, default_value = "contributions.svg")]
        output: PathBuf,
    },
}
