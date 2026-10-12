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

        if let Some(access_key) = self.credentials.get("aws_access_key_id") {
            env_vars.push(("AWS_ACCESS_KEY_ID".to_string(), access_key.clone()));
        }

        if let Some(secret_key) = self.credentials.get("aws_secret_access_key") {
            env_vars.push(("AWS_SECRET_ACCESS_KEY".to_string(), secret_key.clone()));
        }

        if let Some(session_token) = self.credentials.get("aws_session_token") {
            env_vars.push(("AWS_SESSION_TOKEN".to_string(), session_token.clone()));
        }

        if let Some(region) = self.credentials.get("region") {
            env_vars.push(("AWS_REGION".to_string(), region.clone()));
        }

        if let Some(endpoint_url) = self.config.get("endpoint_url") {
            env_vars.push(("AWS_ENDPOINT_URL".to_string(), endpoint_url.clone()));
        }

        // AWS_NO_SIGN_REQUEST
        // AWS_REQUEST_PAYER
        // AWS_S3_ENDPOINT
        // AWS_VIRTUAL_HOSTING

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

    for section in config.sections() {
        let mut profile_credentials = HashMap::new();
        let mut profile_config = HashMap::new();

        if let Some(section_map) = config.get_map_ref().get(&section) {
            for (key, value) in section_map {
                if let Some(v) = value {
                    profile_config.insert(key.clone(), v.clone());
                }
            }
        }

        if let Some(section_map) = credentials.get_map_ref().get(&section) {
            for (key, value) in section_map {
                if let Some(v) = value {
                    profile_credentials.insert(key.clone(), v.clone());
                }
            }
        }

        profiles.push(AwsProfile {
            name: section.clone(),
            credentials: profile_credentials,
            config: profile_config,
        });
    }

    Ok(profiles)
}
