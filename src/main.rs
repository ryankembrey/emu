mod app;
mod config;
mod email;
mod input;

use app::build_cli;
use config::{handle_config, UserDetails};
use dirs;
use input::{get_user_input, open_editor};
use std::fs;
use std::io::Read;
use tempfile::NamedTempFile;
use toml::Value;

use self::email::send_email;

fn main() {
    handle_config();

    let matches = build_cli().get_matches();

    if matches.is_present("recipient") {
        let to_email = matches
            .value_of("recipient")
            .unwrap_or_default()
            .to_string();
        let subject = matches.value_of("subject").unwrap_or_default().to_string();

        let body = if let Some(body_text) = matches.value_of("body") {
            body_text.to_string()
        } else if let Some(file_path) = matches.value_of("file") {
            fs::read_to_string(file_path).expect("Failed to read file")
        } else {
            // Prompt the user to enter the body if neither body nor file is provided
            get_user_input("Enter the email body:")
        };

        let toml_config_path = dirs::config_dir().unwrap().join("emu/config.toml");
        let toml_file_contents =
            fs::read_to_string(&toml_config_path).expect("Error reading TOML config file");
        let toml: Value = toml::from_str(&toml_file_contents).expect("Error parsing TOML");
        let my_details =
            UserDetails::from_toml(&toml).expect("Error creating UserDetails from TOML");

        // Use the function from the email module
        email::send_email(my_details, &to_email, &subject, &body);
    } else {
        let to_email = get_user_input("Enter the email of the recipient:");
        let subject = get_user_input("Enter the subject of the email:");

        // Create a temporary file
        let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");

        // Open the file
        open_editor(temp_file.path().to_str().unwrap());

        // Read the content of the temporary file
        let mut body = String::new();
        temp_file
            .read_to_string(&mut body)
            .expect("Failed to read temporary file");

        // Specify your TOML file path
        let toml_config_path = dirs::config_dir().unwrap().join("emu/config.toml");

        // Read the TOML file contents
        let toml_file_contents =
            fs::read_to_string(&toml_config_path).expect("Error reading TOML config file");

        // Parse the TOML contents
        let toml: Value = toml::from_str(&toml_file_contents).expect("Error parsing TOML");

        // Create UserDetails from the parsed TOML
        let my_details =
            UserDetails::from_toml(&toml).expect("Error creating UserDetails from TOML");

        send_email(my_details, &to_email, &subject, &body);
    }
}
