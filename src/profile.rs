use anyhow::{Context, Result};
use configparser::ini::Ini;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AwsProfile {
    pub name: String,
    pub credentials: HashMap<String, String>,
    pub config: HashMap<String, String>,
}

impl AwsProfile {
    pub fn to_env(&self) -> Vec<(String, String)> {
        let mut env_vars = vec![("AWS_PROFILE".to_string(), self.name.clone())];

        // Credentials file keys: https://docs.aws.amazon.com/sdkref/latest/guide/settings-reference.html#CredFileSettings
        for (key, value) in &self.credentials {
            let varname = key.to_uppercase();
            if varname.starts_with("AWS_") {
                env_vars.push((varname, value.clone()));
            } else {
                env_vars.push((format!("AWS_{}", varname), value.clone()));
            }
        }

        // Config file keys: https://docs.aws.amazon.com/sdkref/latest/guide/settings-reference.html#ConfigFileSettings
        for (key, value) in &self.config {
            let varname = key.to_uppercase();
            if varname.starts_with("AWS_") {
                env_vars.push((varname, value.clone()));
            } else {
                env_vars.push((format!("AWS_{}", varname), value.clone()));
            }
        }

        env_vars
    }
}

fn get_aws_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;
    Ok(home.join(".aws"))
}

pub fn parse_profiles() -> Result<Vec<AwsProfile>> {
    let config_dir = get_aws_dir()?;
    let mut config = Ini::new();
    let mut credentials = Ini::new();

    config
        .load(
            config_dir
                .join("config")
                .to_str()
                .context("Cannot find config file")?,
        )
        .map_err(|e| anyhow::anyhow!("Failed to load AWS config: {}", e))?;

    credentials
        .load(
            config_dir
                .join("credentials")
                .to_str()
                .context("Cannot find credentials file")?,
        )
        .map_err(|e| anyhow::anyhow!("Failed to load AWS credentials: {}", e))?;

    let mut profiles = Vec::new();

    // Collect profile names from both files. Config sections are prefixed
    // with "profile " (except "default"), credentials sections are not.
    let mut profile_names: Vec<String> = Vec::new();
    for section in config.sections() {
        if section.starts_with("profile ") {
            let name = section.strip_prefix("profile ");
            if let Some(profile) = name {
                let profile_str = profile.to_string();
                if !profile_names.contains(&profile_str) {
                    profile_names.push(profile_str);
                }
            }
        }
    }
    for section in credentials.sections() {
        if !profile_names.contains(&section) {
            profile_names.push(section);
        }
    }

    for name in profile_names {
        let mut profile_credentials = HashMap::new();
        let mut profile_config = HashMap::new();

        let config_section = if name == "default" {
            name.clone()
        } else {
            format!("profile {}", name)
        };

        if let Some(section_map) = config.get_map_ref().get(&config_section) {
            for (key, value) in section_map {
                if let Some(v) = value {
                    profile_config.insert(key.clone(), v.clone());
                }
            }
        }

        if let Some(section_map) = credentials.get_map_ref().get(&name) {
            for (key, value) in section_map {
                if let Some(v) = value {
                    profile_credentials.insert(key.clone(), v.clone());
                }
            }
        }

        profiles.push(AwsProfile {
            name,
            credentials: profile_credentials,
            config: profile_config,
        });
    }

    Ok(profiles)
}
