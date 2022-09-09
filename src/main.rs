mod commands;

use std::fs;
use commands::Command;
use std::{process, thread, time};
use std::io::{self, stdout, Write};
use colored::Colorize;
use std::path::PathBuf;
use clap::{Parser};

const BACKSPACE: char = 8u8 as char;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Screenplay file
    #[clap(value_parser, value_hint = clap::ValueHint::FilePath)]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let data = fs::read_to_string(cli.file).expect("Unable to read file");
    let commands: Vec<Command> = serde_yaml::from_str(&data).unwrap();
    parse(commands);
}

fn parse(commands: Vec<Command>) {
    for cmd in commands {
        match cmd {
            Command::Write {msec, text, color} => {
                for c in text.chars() { 
                    thread::sleep(time::Duration::from_millis(msec.into()));
                    colorful_print(&c.to_string(), color);
                    stdout().flush().unwrap();
                }
            },
            Command::Erase {msec, by_chars} => {
                for _ in by_chars.chars() { 
                    thread::sleep(time::Duration::from_millis(msec.into()));
                    print!("{} {}", BACKSPACE, BACKSPACE);
                    stdout().flush().unwrap();
                }
            },
            Command::Execute {line} => {
                println!("");

                let args = line.split(char::is_whitespace).collect::<Vec<&str>>();

                if let Some((cmd, args)) = args.split_first() {
                    process::Command::new(cmd)
                        .args(args)
                        .spawn()
                        .expect("ls command failed to start");
                }
            },
            Command::Wait {msec} => {
                thread::sleep(time::Duration::from_millis(msec.into()));
            },
            Command::Pause => {
                let mut answer = String::new();
                io::stdin().read_line(&mut answer)
                  .ok()
                  .expect("Failed to read line");
            },
        }
    }
}

fn colorful_print(c: &str, color: Option<&str>) {
    match color {
        Some("black") => print!("{}", c.black()),
        Some("red") => print!("{}", c.red()),
        Some("green") => print!("{}", c.green()),
        Some("yellow") => print!("{}", c.yellow()),
        Some("blue") => print!("{}", c.blue()),
        Some("magenta") => print!("{}", c.magenta()),
        Some("cyan") => print!("{}", c.cyan()),
        Some("white") => print!("{}", c.white()),
        Some("bright_black") => print!("{}", c.bright_black()),
        Some("bright_red") => print!("{}", c.bright_red()),
        Some("bright_green") => print!("{}", c.bright_green()),
        Some("bright_yellow") => print!("{}", c.bright_yellow()),
        Some("bright_blue") => print!("{}", c.bright_blue()),
        Some("bright_magenta") => print!("{}", c.bright_magenta()),
        Some("bright_cyan") => print!("{}", c.bright_cyan()),
        Some("bright_white") => print!("{}", c.bright_white()),
        Some(_) | None => print!("{c}"),
    }
}