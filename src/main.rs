mod fuzzy;
mod profile;

use anyhow::{Context, Result};

fn main() -> Result<()> {
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

        println!("# Source this output to set AWS environment variables");
        println!("# Usage: eval $(awsprofile)");
        for (key, value) in env_vars {
            println!("export {}=\"{}\"", key, value);
        }
    } else {
        eprintln!("No profile selected");
    }

    Ok(())
}
