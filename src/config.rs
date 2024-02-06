use crate::input::get_user_input;
use serde_derive::{Deserialize, Serialize};
use std::path::Path;
use std::process::exit;
use toml::Value;

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

pub fn config_file_exists() -> bool {
    let config_path = dirs::config_dir().map(|p| p.join("emu/config.toml"));

    if let Some(path) = config_path {
        return Path::new(&path).exists();
    }

    false
}

pub fn generate_config() {
    // Prompt the user for details
    let password = get_user_input("Enter your email password:");
    let email = get_user_input("Enter your email address:");
    let host = get_user_input("Enter your email host (e.g., smtp.gmail.com):");

    let user_details = UserDetails {
        password,
        email,
        host,
    };

    // Create a TOML Value from UserDetails
    let toml_value =
        toml::to_string(&user_details).expect("Failed to serialize UserDetails to TOML");

    // Create a TOML table with "details" key
    let mut toml_table = toml::value::Table::new();
    toml_table.insert("details".to_string(), toml::Value::String(toml_value));

    // Write the TOML table to a file
    let toml_string = toml::to_string(&toml_table).expect("Failed to serialize TOML table");
    write_config_file(&toml_string);

    println!("Config file generated successfully!");
    println!("Re-run emu to send an email");
    exit(0);
}

fn write_config_file(config: &str) {
    use std::fs::File;
    use std::io::Write;

    // Specify your TOML file path
    let toml_config_path = dirs::config_dir().unwrap().join("emu/config.toml");

    // Create the config file
    let mut file = File::create(&toml_config_path).expect("Failed to create config file");

    // Write the config to the file
    file.write_all(config.as_bytes())
        .expect("Failed to write to config file");
}
