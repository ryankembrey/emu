use clap::{App, Arg};

pub fn build_cli() -> clap::App<'static, 'static> {
    App::new("emu")
        .version("0.2.0")
        .about("Email Utility: Send emails over CLI")
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
}
