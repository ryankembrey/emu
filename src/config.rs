use toml::Value;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDetails {
    pub password: String,
    pub email: String,
    pub host: String,
}

impl UserDetails {
    pub fn from_toml(toml: &Value) -> Result<Self, &'static str> {
        let details_table = toml["details"].as_table().ok_or("Invalid TOML structure")?;
        let password = details_table["password"]
            .as_str()
            .ok_or("Missing or invalid password")?
            .to_string();
        let email = details_table["email"]
            .as_str()
            .ok_or("Missing or invalid email")?
            .to_string();
        let host = details_table["host"]
            .as_str()
            .ok_or("Missing or invalid host")?
            .to_string();
        Ok(UserDetails {
            password,
            email,
            host,
        })
    }
}
