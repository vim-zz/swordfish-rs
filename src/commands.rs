use anyhow::Result;
use std::{cmp, process};
use std::io::{self, stdout, Write as IOWrite};
use serde::{Deserialize};
use enum_dispatch::enum_dispatch;
use crate::{functions, terminal, state::State};

#[enum_dispatch]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    Clear(Clear),
    Erase(Erase),
    Execute(Execute),
    NewLine(NewLine),
    Pause(Pause),
    Prompt(Prompt),
    Wait(Wait),
    Write(Write),
    Turbo(Turbo),
}

#[enum_dispatch(Command)]
pub trait Runnable {
    fn run(&self, state: &mut State) -> Result<()>;
}

#[derive(Debug, Deserialize)]
pub struct Clear {}

impl Runnable for Clear {
    fn run(&self, state: &mut State) -> Result<()> {
        print!("\x1B[2J\x1B[1;1H");
        functions::show_prompt(&state.prompt)?;
        state.cursor = 0;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Erase {
    pub msec: u32, 
    pub by_chars: Option<String>, 
    pub amount: Option<u32>,
}

impl Runnable for Erase {
    fn run(&self, state: &mut State) -> Result<()> {
        let deletions = match (self.by_chars.as_ref(), self.amount) {
            (Some(by_chars), None) => by_chars.len(),
            (None, Some(amount)) => amount as usize,
            (Some(by_chars), Some(amount)) => amount as usize + by_chars.len(),
            (None, None) => 0,
        };

        // Remove the deletions up till the cursor
        let deletions = cmp::min(deletions, state.cursor);
        state.cursor -= deletions;
        functions::erase(deletions, self.msec, state.speed_factor)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Execute {
    pub line: String, 
}

impl Runnable for Execute {
    fn run(&self, state: &mut State) -> Result<()> {
        println!("");
        let words = shellwords::split(&self.line).unwrap();

        if let Some((cmd, args)) = words.split_first() {
            process::Command::new(cmd).args(args).spawn()?;
        }
        functions::delay(crate::DELAY_AFTER_EXECUTE, state.speed_factor);
        functions::show_prompt(&state.prompt)?;
        state.cursor = 0;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct NewLine {}

impl Runnable for NewLine {
    fn run(&self, state: &mut State) -> Result<()> {
        print!("\n");
        functions::show_prompt(&state.prompt)?;
        state.cursor = 0;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Pause {}

impl Runnable for Pause {
    fn run(&self, _state: &mut State) -> Result<()> {
        let mut answer = String::new();
        io::stdin().read_line(&mut answer)?;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Prompt {
    pub text: String, 
    pub color: Option<String>,
}

impl Runnable for Prompt {
    fn run(&self, state: &mut State) -> Result<()> {
        let ps1 = terminal::colorful(&self.text, self.color.as_deref());
        state.prompt = Some(ps1);
        functions::show_prompt(&state.prompt)?;
        state.cursor = 0;
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Wait {
    pub msec: u32,
}

impl Runnable for Wait {
    fn run(&self, state: &mut State) -> Result<()> {
        functions::delay(self.msec, state.speed_factor);
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Write {
    pub msec: u32, 
    pub color: Option<String>, 
    pub text: String,
}

impl Runnable for Write {
    fn run(&self, state: &mut State) -> Result<()> {
        for c in self.text.chars() { 
            functions::delay(self.msec, state.speed_factor);
            print!("{}", terminal::colorful(&c.to_string(), self.color.as_deref()));
            stdout().flush()?;
        }
        state.cursor += self.text.len();
        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Turbo {
    pub by: u32,
}

impl Runnable for Turbo {
    fn run(&self, state: &mut State) -> Result<()> {
        state.speed_factor = self.by;
        Ok(())
    }
}
