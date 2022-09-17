mod commands;
mod terminal;
mod functions;
mod state;

use anyhow::Result;
use commands::Command;
use state::State;
use commands::Runnable;

pub fn from_yaml(data: &str) -> Result<Vec<Command>> {
    let yaml = serde_yaml::from_str(data)?;
    Ok(yaml)
}

const DELAY_AFTER_EXECUTE: u32 = 250;

pub fn execute(commands: Vec<Command>) -> Result<()> {
    let mut state = State {
        prompt: None,
        cursor: 0,
        speed_factor: 1,
    };

    for cmd in commands {
        cmd.run(&mut state)?;
    }

    Ok(())
}
