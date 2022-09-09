use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Command<'a> {
    Write {msec: u32, color: Option<&'a str>, text: &'a str},
    Erase {msec: u32, by_chars: &'a str},
    Execute {line: &'a str},
    Wait {msec: u32},
    Clear,
    Pause,
}

