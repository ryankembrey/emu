mod app;
mod config;
mod input;

use app::build_cli;
use config::{config_file_exists, generate_config, UserDetails};
use dirs;
use input::{get_user_input, open_editor};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::fs;
use std::io::Read;
use std::process::exit;
use tempfile::NamedTempFile;
use toml::Value;

fn main() {
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

        // Build the email message
        let email = Message::builder()
            .from(my_details.email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject(subject)
            .body(body)
            .unwrap();

        // Sending confirmation
        let send_confirmation = get_user_input("Send the mail? (Y/n)");
        if send_confirmation.to_lowercase() == "y" {
            // Set up credentials
            let creds: Credentials = Credentials::new(
                my_details.email.to_string(),
                my_details.password.to_string(),
            );

            // Open a remote connection to Gmail
            let mailer: SmtpTransport = SmtpTransport::relay(&my_details.host)
                .unwrap()
                .credentials(creds)
                .build();

            // Send email
            if let Err(e) = mailer.send(&email) {
                eprintln!("Error sending email: {:?}", e);
            } else {
                println!("Email sent successfully!");
            }
        } else {
            println!("Email not sent.");
        }
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

        // Build the email message
        let email: Message = Message::builder()
            .from(my_details.email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject(subject)
            .body(body)
            .unwrap();

        // Sending conirmation
        let send_confirmation = get_user_input("Send the mail? (Y/n)");
        if send_confirmation.to_lowercase() == "y" {
            // Set up credentials
            let creds: Credentials = Credentials::new(
                my_details.email.to_string(),
                my_details.password.to_string(),
            );

            // Open a remote connection to Gmail
            let mailer: SmtpTransport = SmtpTransport::relay(&my_details.host)
                .unwrap()
                .credentials(creds)
                .build();

            // Send the email
            if let Err(e) = mailer.send(&email) {
                eprintln!("Error sending email: {:?}", e);
            } else {
                println!("Email sent successfully!");
            }
        } else {
            println!("Email not sent.");
        }
    }
}
