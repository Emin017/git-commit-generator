mod api;
mod config;
mod git;

use anyhow::{Context, Result, bail};
use clap::Parser;
use colored::Colorize;
use config::{CommitFormat, Config};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Commit message format
    #[arg(short, long, value_enum, default_value_t = CommitFormat::Conventional)]
    format: CommitFormat,

    /// Deepseek model to use
    #[arg(short, long, default_value = "deepseek-chat")]
    model: String,

    /// Show recent commit titles (0 = don't show)
    #[arg(short, long, default_value_t = 0)]
    git_history: usize,
}

#[tokio::main]
async fn main() -> Result<()> {
    print_banner();

    dotenv::dotenv().ok();

    let args = Args::parse();

    let config = Config {
        model: args.model,
        format: args.format,
        api_key: std::env::var("DEEPSEEK_API_KEY").context(
            "🔑 Unable to find DEEPSEEK_API_KEY in environment, please set it in .env file",
        )?,
    };

    let diff = git::get_staged_diff().context("Failed to get staged diff")?;

    if diff.is_empty() {
        // Returns an error if there are no changes to commit
        bail!(
            "{} {}",
            "⚠️".yellow(),
            "No changes to commit, please staget your changes first"
        );
    }

    let history_titles = if args.git_history > 0 {
        match git::get_recent_commit_titles(args.git_history) {
            Ok(titles) => {
                println!(
                    "{} {}",
                    "📜".cyan(),
                    format!("Reference {} recent commit titles", args.git_history).cyan()
                );
                Some(titles)
            }
            Err(e) => {
                println!(
                    "{} {}",
                    "⚠️".yellow(),
                    format!("Failed to get commit history: {}", e).yellow()
                );
                None
            }
        }
    } else {
        None
    };

    let _ = api::generate_commit_message(&config, &diff, history_titles)
        .await
        .context("Failed to generate commit message")?;

    Ok(())
}

fn print_banner() {
    let banner = r#"
     _____  _  _      ___                          _  _
    |  __ \(_)| |    / __|                        (_)| |
    | |  \/|_|| |_  | |     ___   _ __ ___   _ __  _ | |_
    | | __|  ||  _| | |    / _ \ | '_ ` _ \ | '_ \| || __|
    | |_\ \| || |   | |___| (_) || | | | | || | | | || |_
     \____/|_||_|    \_____\___/ |_| |_| |_||_| |_|_| \__|

            🚀 Powered by DeepSeek AI 🤖
    "#;

    println!("{}", banner.bright_purple());
}
