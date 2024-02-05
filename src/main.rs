use clap::{App, Arg};
use std::fs;
use std::io;
use std::io::Read;
use std::process::Command;
use tempfile::NamedTempFile;
use toml::Value;
use serde_derive::Deserialize;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use dirs;

#[derive(Debug, Deserialize)]
struct UserDetails {
    password: String,
    email: String,
    host: String,
}

impl UserDetails {
    fn from_toml(toml: &Value) -> Result<Self, &'static str> {
        let details_table = toml["details"].as_table().ok_or("Invalid TOML structure")?;
        let password = details_table["password"].as_str().ok_or("Missing or invalid password")?.to_string();
        let email = details_table["email"].as_str().ok_or("Missing or invalid email")?.to_string();
        let host = details_table["host"].as_str().ok_or("Missing or invalid host")?.to_string();
        Ok(UserDetails { password, email, host })
    }
}

fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn open_editor(file_path: &str) {
    let editor = std::env::var("EDITOR").unwrap_or(String::from("nano")); // Use nano if EDITOR is not set
    Command::new(editor).arg(file_path).status().expect("Failed to open editor");
}

fn main() {
    let matches = App::new("emu")
        .version("0.1.0")
        .about("Email Utitly: Send emails over CLI")
        .after_help("If no arguments are provided, the program will prompt you for the required information.")
        .arg(
            Arg::with_name("recipient")
                .short("r")
                .long("recipient")
                .takes_value(true)
                .required(false)
                .help("Recipient's email address"),
        )
        .arg(
            Arg::with_name("subject")
                .short("s")
                .long("subject")
                .takes_value(true)
                .required(false)
                .help("Email subject"),
        )
        .arg(
            Arg::with_name("body")
                .short("b")
                .long("body")
                .takes_value(true)
                .conflicts_with("file")
                .help("Email body text"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .takes_value(true)
                .conflicts_with("body")
                .help("Path to a file containing the email body"),
        )
        .get_matches();

    if matches.is_present("recipient") {
        let to_email = matches.value_of("recipient").unwrap_or_default().to_string();
        let subject = matches.value_of("subject").unwrap_or_default().to_string();

        let body = if let Some(body_text) = matches.value_of("body") {
            body_text.to_string()
        } else if let Some(file_path) = matches.value_of("file") {
            fs::read_to_string(file_path).expect("Failed to read file")
        } else {
            // Prompt the user to enter the body if neither body nor file is provided
            get_user_input("Enter the email body:")
        };

        // Continue with the rest of your code...
        let toml_config_path = dirs::config_dir().unwrap().join("emu/config.toml");
        let toml_file_contents = fs::read_to_string(&toml_config_path).expect("Error reading TOML config file");
        let toml: Value = toml::from_str(&toml_file_contents).expect("Error parsing TOML");
        let my_details = UserDetails::from_toml(&toml).expect("Error creating UserDetails from TOML");

        // Build the email message
        let email = Message::builder()
            .from(my_details.email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject(subject)
            .body(body)
            .unwrap();

        // Prompt the user before sending the email
        let send_confirmation = get_user_input("Send the mail? (Y/n)");
        if send_confirmation.to_lowercase() == "y" {
            // Set up credentials
            let creds: Credentials = Credentials::new(my_details.email.to_string(), my_details.password.to_string());

            // Open a remote connection to Gmail
            let mailer: SmtpTransport = SmtpTransport::relay(&my_details.host)
                .unwrap()
                .credentials(creds)
                .build();

            // Send the email with enhanced error handling
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

        // Open the file with the user's preferred text editor
        open_editor(temp_file.path().to_str().unwrap());

        // Read the content of the temporary file for the email body
        let mut body = String::new();
        temp_file.read_to_string(&mut body).expect("Failed to read temporary file");

        // Specify your TOML file path
        let toml_config_path = dirs::config_dir().unwrap().join("emu/config.toml");

        // Read the TOML file contents
        let toml_file_contents = fs::read_to_string(&toml_config_path).expect("Error reading TOML config file");

        // Parse the TOML contents
        let toml: Value = toml::from_str(&toml_file_contents).expect("Error parsing TOML");

        // Create UserDetails from the parsed TOML
        let my_details = UserDetails::from_toml(&toml).expect("Error creating UserDetails from TOML");

        // Build the email message
        let email: Message = Message::builder()
            .from(my_details.email.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject(subject)
            .body(body)
            .unwrap();

        // Prompt the user before sending the email
        let send_confirmation = get_user_input("Send the mail? (Y/n)");
        if send_confirmation.to_lowercase() == "y" {
            // Set up credentials
            let creds: Credentials = Credentials::new(my_details.email.to_string(), my_details.password.to_string());

            // Open a remote connection to Gmail
            let mailer: SmtpTransport = SmtpTransport::relay(&my_details.host)
                .unwrap()
                .credentials(creds)
                .build();

            // Send the email with enhanced error handling
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
