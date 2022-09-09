mod commands;
mod terminal;

use anyhow::Result;
use std::{process, thread, time};
use std::io::{self, stdout, Write};
use commands::Command;
use terminal::BACKSPACE;

pub fn from_yaml(data: &str) -> Result<Vec<Command>> {
    let yaml = serde_yaml::from_str(data)?;
    Ok(yaml)
}

pub fn execute(commands: Vec<Command>) -> Result<()> {
    for cmd in commands {
        match cmd {
            Command::Write {msec, text, color} => {
                for c in text.chars() { 
                    thread::sleep(time::Duration::from_millis(msec.into()));
                    terminal::colorful_print(&c.to_string(), color);
                    stdout().flush()?;
                }
            },
            Command::Erase {msec, by_chars} => {
                for _ in by_chars.chars() { 
                    thread::sleep(time::Duration::from_millis(msec.into()));
                    print!("{} {}", BACKSPACE, BACKSPACE);
                    stdout().flush()?;
                }
            },
            Command::Execute {line} => {
                println!("");
                let words = shellwords::split(line).unwrap();

                if let Some((cmd, args)) = words.split_first() {
                    process::Command::new(cmd).args(args).spawn()?;
                }
            },
            Command::Wait {msec} => {
                thread::sleep(time::Duration::from_millis(msec.into()));
            },
            Command::Pause => {
                let mut answer = String::new();
                io::stdin().read_line(&mut answer)?;
            },
            Command::Clear => {
                print!("\x1B[2J\x1B[1;1H");
            },
        }
    }

    Ok(())
}

