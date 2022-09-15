mod commands;
mod terminal;

use anyhow::Result;
use std::{cmp, process, thread, time};
use std::io::{self, stdout, Write};
use commands::Command;
use terminal::BACKSPACE;

pub fn from_yaml(data: &str) -> Result<Vec<Command>> {
    let yaml = serde_yaml::from_str(data)?;
    Ok(yaml)
}

const DELAY_AFTER_EXECUTE: u32 = 250;

pub fn execute(commands: Vec<Command>) -> Result<()> {
    let mut prompt = None;
    let mut cursor = 0;
    let mut speed_factor = 1;

    for cmd in commands {
        match cmd {
            Command::Write {msec, text, color} => {
                for c in text.chars() { 
                    delay(msec, speed_factor);
                    print!("{}", terminal::colorful(&c.to_string(), color));
                    stdout().flush()?;
                }
                cursor += text.len();
            },
            Command::Erase {msec, by_chars, amount} => {
                let deletions = match (by_chars, amount) {
                    (Some(by_chars), None) => by_chars.len(),
                    (None, Some(amount)) => amount as usize,
                    (Some(by_chars), Some(amount)) => amount as usize + by_chars.len(),
                    (None, None) => 0,
                };

                // Remove the deletions up till the cursor
                let deletions = cmp::min(deletions, cursor);
                cursor -= deletions;
                erase(deletions, msec, speed_factor)?;
            },
            Command::Execute {line} => {
                println!("");
                let words = shellwords::split(line).unwrap();

                if let Some((cmd, args)) = words.split_first() {
                    process::Command::new(cmd).args(args).spawn()?;
                }
                delay(DELAY_AFTER_EXECUTE, speed_factor);
                show_prompt(&prompt)?;
                cursor = 0;
            },
            Command::Wait {msec} => {
                delay(msec, speed_factor);
            },
            Command::Pause => {
                let mut answer = String::new();
                io::stdin().read_line(&mut answer)?;
            },
            Command::Clear => {
                print!("\x1B[2J\x1B[1;1H");
                show_prompt(&prompt)?;
                cursor = 0;
            }
            Command::Prompt {text, color} => {
                let ps1 = terminal::colorful(text, color);
                prompt = Some(ps1);
                show_prompt(&prompt)?;
                cursor = 0;
            },
            Command::NewLine => {
                print!("\n");
                show_prompt(&prompt)?;
                cursor = 0;
            },
            Command::Turbo {by} => {
                speed_factor = by;
            },
        }
    }

    Ok(())
}


fn show_prompt(prompt: &Option<String>) -> Result<()> {
    if let Some(ps1) = prompt {
        print!("{ps1} ");
        stdout().flush()?;
    }
    Ok(())
}

fn erase(amount: usize, msec: u32, speed_factor: u32) -> Result<()> {
    for _ in 0..amount { 
        delay(msec, speed_factor);
        print!("{} {}", BACKSPACE, BACKSPACE);
        stdout().flush()?;
    }
    Ok(())
}

fn delay(msec: u32, speed_factor: u32) {
    let t = msec / speed_factor;
    thread::sleep(time::Duration::from_millis(t.into()));
}
