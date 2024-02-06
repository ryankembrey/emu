use std::io;
use std::process::Command;

pub fn get_user_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn open_editor(file_path: &str) {
    let editor = std::env::var("EDITOR").unwrap_or(String::from("nano"));
    Command::new(editor)
        .arg(file_path)
        .status()
        .expect("Failed to open editor");
}
