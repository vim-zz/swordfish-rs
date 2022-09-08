mod commands;

use std::fs;
use commands::Command;
use std::{process, thread, time};
use std::io::{self, stdout, Write};
use rand::Rng;

const BACKSPACE: char = 8u8 as char;

fn main() {
    let data = fs::read_to_string("tests/screenplay.yaml").expect("Unable to read file");
    let commands: Vec<Command> = serde_yaml::from_str(&data).unwrap();
    parse(commands);
}

fn parse(commands: Vec<Command>) {
    for cmd in commands {
        match cmd {
            Command::Write {msec, text} => {
                for c in text.chars() { 
                    thread::sleep(time::Duration::from_millis(msec.into()));
                    print!("{c}");
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
            Command::Wait {msec, rand} => {
                let range = rand as i32;
                let num = rand::thread_rng().gen_range(-range..range);
                let wait = ((msec as i32) + num).abs() as u32;
                thread::sleep(time::Duration::from_millis((wait).into()));
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