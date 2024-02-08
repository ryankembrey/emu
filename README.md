# emu
`emu` (Email Utility) is a straightforward Command-Line Interface (CLI) program designed to simplify the process of sending emails from your terminal. It has been designed with 
the intent of being used in automated shell scripts.
## Features
- Send emails through a CLI Interface
- Support for editing the email body with any text editor


## Installation 


Clone the repository
```bash
git clone https://github.com/ryankembrey/emu ; cd emu
```
Build the binary
```bash
cargo build --release
```
Copy the binary to a directory in your `$PATH`. For example: 
```bash
cp ./target/release/emu ~/.local/bin/emu
```

Alternatively, downlaod the binary from the [releases page](https://github.com/ryankembrey/emu/releases).

## Usage
To use `emu`, simply run 
```bash
emu
```

The first time `emu` is run, the program will prompt you for details to enter into your local config. The program asks for:
- Your email address 
- Your password (use a app-specifc password for Gmail accounts)
- The SMTP host (default is for Gmail)

`emu` consists of two modes: prompt-mode and command-mode. If `emu` is run with no arguments, this will enter the program into prompt mode, meaning it will ask for the fields it needs. If arguments are provided, `emu` will enter command-mode in which it needs three arguments.
```bash
emu --recipient <RECIPIENT> --subject <SUBJECT> --body <BODY>
```
Alternatively, you can specify the path to a file to be read with `--file`. This option should be used in place of `--body` like so.
```bash
emu --recipient <RECIPIENT> --subject <SUBJECT> --file <FILE_PATH>
```

Below are all the flags for `emu`.
```bash
FLAGS:
    -h, --help                      Prints help information
    -V, --version                   Prints version information

OPTIONS:
    -b, --body <body>               Email body text
    -f, --file <file>               Path to a file containing the email body
    -r, --recipient <recipient>     Recipient email address
    -s, --subject <subject>         Email subject
```
## Todo
- [x] Add config generation
- [ ] Add file attachment support
- [ ] Add support for more email providers
- [ ] Add reply support
- [ ] Add email queue
- [ ] Add read from file mode
- [ ] Add encryption for local password storage
