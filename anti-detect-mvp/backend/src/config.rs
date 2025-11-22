use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Deserialize)]
struct FileConfig {
    jwt_secret: String,
}

pub fn load_jwt_secret() -> anyhow::Result<String> {
    dotenv::dotenv().ok();

    if let Ok(secret) = std::env::var("JWT_SECRET") {
        let trimmed = secret.trim();
        if !trimmed.is_empty() {
            return Ok(trimmed.to_owned());
        }
    }

    if let Ok(path) = std::env::var("CONFIG_FILE") {
        if Path::new(&path).exists() {
            return read_secret_from_file(&path);
        }
    }

    if Path::new("config/settings.toml").exists() {
        return read_secret_from_file("config/settings.toml");
    }

    #[cfg(test)]
    if Path::new("config/test.toml").exists() {
        return read_secret_from_file("config/test.toml");
    }

    anyhow::bail!("JWT secret not configured. Set JWT_SECRET or provide config/settings.toml");
}

fn read_secret_from_file(path: &str) -> anyhow::Result<String> {
    let content = fs::read_to_string(path)?;
    let config: FileConfig = toml::from_str(&content)?;

    let secret = config.jwt_secret.trim();
    if secret.is_empty() {
        anyhow::bail!("JWT secret is empty in configuration file: {path}");
    }

    Ok(secret.to_owned())
}
