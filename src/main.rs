mod fuzzy;
mod profile;

use anyhow::{Context, Result};
use clap::Parser;
use std::io::IsTerminal;

const AFTER_HELP: &str = "\
The program prints `export` statements for the selected profile to stdout.
Because a child process cannot modify its parent's environment, the output
must be evaluated by your shell:

    bash/zsh:  eval \"$(awsprofile)\"
    fish:      awsprofile | source

Tip: add an alias to your shell config, e.g.
    alias awsp='eval \"$(awsprofile)\"'";

/// Interactively select an AWS profile and export its environment variables
#[derive(Parser)]
#[command(version, about, override_usage = "eval \"$(awsprofile)\"", after_help = AFTER_HELP)]
struct Cli {
    #[arg(short, long, help = "Unset all AWS environment variables")]
    unset: bool
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.unset {
        // Unset all environment variables in current environment that start with "AWS_"
        let env_vars: Vec<(String, String)> = std::env::vars()
            .filter(|(key, _)| key.starts_with("AWS_"))
            .collect();
        if std::io::stdout().is_terminal() {
            eprintln!("# Nothing was applied. To set the variables in your shell, run:");
            eprintln!("#   eval \"$(awsprofile)\"");
        }
        for (key, _) in env_vars {
            println!("unset {}", key);
        }
        return Ok(());
    }

    // Parse AWS profiles
    let profiles = profile::parse_profiles().context("Failed to parse AWS profiles")?;

    if profiles.is_empty() {
        eprintln!("No AWS profiles found");
        return Ok(());
    }

    // Get profile names for fuzzy selection
    let profile_names: Vec<String> = profiles.iter().map(|p| p.name.clone()).collect();

    // Let user select a profile
    let selected = fuzzy::fuzzy_select(profile_names);

    if let Some(selected_name) = selected {
        let profile = profiles
            .iter()
            .find(|p| p.name == selected_name)
            .context("Selected profile not found")?;

        let env_vars = profile.to_env();

        // Only show the usage hint when stdout is a terminal, i.e. the user ran the
        // program directly instead of via `eval "$(awsprofile)"`. When captured by
        // the shell, stdout is a pipe and must contain nothing but shell code.
        if std::io::stdout().is_terminal() {
            eprintln!("# Nothing was applied. To set the variables in your shell, run:");
            eprintln!("#   eval \"$(awsprofile)\"");
        }
        for (key, value) in env_vars {
            println!("export {}=\"{}\"", key, value);
        }
    } else {
        eprintln!("No profile selected");
    }

    Ok(())
}