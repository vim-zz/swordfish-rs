use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command<'a> {
    Write {msec: u32, color: Option<&'a str>, text: &'a str},
    Erase {msec: u32, by_chars: Option<&'a str>, amount: Option<u32>},
    Execute {line: &'a str},
    Wait {msec: u32},
    Clear,
    Pause,
    Prompt {text: &'a str, color: Option<&'a str>}
}