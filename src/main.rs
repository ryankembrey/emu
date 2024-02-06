mod config;
mod input;

use input::get_user_input;
use config::UserDetails;
use clap::{App, Arg};
use dirs;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::process::Command;
use tempfile::NamedTempFile;
use toml::Value;



fn open_editor(file_path: &str) {
    let editor = std::env::var("EDITOR").unwrap_or(String::from("nano"));
    Command::new(editor)
        .arg(file_path)
        .status()
        .expect("Failed to open editor");
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

fn main() {
    if config_file_exists() {
        // Config exists
    } else {
        let answer = get_user_input("Config file does not exist. Generate config? (Y/n)")
            .trim()
            .to_lowercase();

        if answer == "yes" || answer == "y" || answer.is_empty() {
            generate_config();
            println!("Re-run emu to send an email");
            exit(0);
        } else {
            println!("Config file not generated. Exiting.");
            exit(0);
        }
    }

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
