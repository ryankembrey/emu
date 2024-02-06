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
        let password = toml["password"]
            .as_str()
            .ok_or("Missing or invalid password")?
            .to_string();
        let email = toml["email"]
            .as_str()
            .ok_or("Missing or invalid email")?
            .to_string();
        let host = toml["host"]
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

fn config_file_exists() -> bool {
    let config_path = dirs::config_dir().map(|p| p.join("emu/config.toml"));

    if let Some(path) = config_path {
        return Path::new(&path).exists();
    }

    false
}

fn generate_config() {
    // Prompt the user for details
    let email = get_user_input("Enter your email address:");
    let password = get_user_input("Enter your email password:");
    let mut host = get_user_input("Enter your email host (default: smtp.gmail.com):");
    if host.trim().is_empty() {
        host = String::from("smpt.gmail.com");
    }

    let user_details = UserDetails {
        password,
        email,
        host,
    };

    // Specify your TOML file path
    let config_dir = dirs::config_dir().unwrap().join("emu");
    let toml_config_path = config_dir.join("config.toml");

    // Create the emu folder if it doesn't exist
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).expect("Failed to create emu folder");
    }

    // Serialize UserDetails directly to TOML
    let toml_string = toml::to_string_pretty(&user_details).expect("Failed to serialize to TOML");

    // Write the config to the file
    std::fs::write(&toml_config_path, toml_string).expect("Failed to write to config file");

    println!("Config file generated successfully!");
    println!("Re-run emu to send an email");
    exit(0);
}

pub fn handle_config() {
    if config_file_exists() {
        // Config exists
    } else {
        let answer = get_user_input("Config file does not exist. Generate config? (Y/n)")
            .trim()
            .to_lowercase();

        if answer == "yes" || answer == "y" || answer.is_empty() {
            generate_config();
        } else {
            println!("Config file not generated. Exiting.");
            exit(0);
        }
    }
}
