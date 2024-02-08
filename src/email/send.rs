use crate::config::UserDetails;
use crate::input::get_user_input;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_email(user_details: UserDetails, to_email: &str, subject: &str, body: &str) {
    // Convert the body to String
    let body_string: String = body.to_string();

    // Build the email message
    let email = Message::builder()
        .from(user_details.email.parse().unwrap())
        .to(to_email.parse().unwrap())
        .subject(subject)
        .body(body_string)
        .unwrap();
    // Sending confirmation
    let send_confirmation = get_user_input("Send the mail? (Y/n)");
    if send_confirmation.to_lowercase() == "y" || send_confirmation.is_empty() {
        // Set up credentials
        let creds: Credentials = Credentials::new(
            user_details.email.to_string(),
            user_details.password.to_string(),
        );

        // Open a remote connection to the email host
        let mailer: SmtpTransport = SmtpTransport::relay(&user_details.host)
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
}
