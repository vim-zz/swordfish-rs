use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Command {
    Write {msec: u32, text: String},
    Erase {msec: u32, by_chars: String},
    Execute {line: String},
    Wait {msec: u32},
    Pause,
}

