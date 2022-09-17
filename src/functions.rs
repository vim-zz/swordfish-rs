use anyhow::Result;
use std::{thread, time};
use std::io::{stdout, Write};
use crate::terminal::BACKSPACE;

pub fn delay(msec: u32, speed_factor: u32) {
    let t = msec / speed_factor;
    thread::sleep(time::Duration::from_millis(t.into()));
}

pub fn erase(amount: usize, msec: u32, speed_factor: u32) -> Result<()> {
    for _ in 0..amount { 
        delay(msec, speed_factor);
        print!("{} {}", BACKSPACE, BACKSPACE);
        stdout().flush()?;
    }
    Ok(())
}

pub fn show_prompt(prompt: &Option<String>) -> Result<()> {
    if let Some(ps1) = prompt {
        print!("{ps1} ");
        stdout().flush()?;
    }
    Ok(())
}

pub fn show_title(text: &str) -> Result<()> {
    write!(stdout(), "\x1B]0;{text}\x07")?;
    Ok(())
}