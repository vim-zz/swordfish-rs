use serde::{Deserialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command<'a> {
    Clear,
    Erase {msec: u32, by_chars: Option<&'a str>, amount: Option<u32>},
    Execute {line: &'a str},
    NewLine,
    Pause,
    Prompt {text: &'a str, color: Option<&'a str>},
    Wait {msec: u32},
    Write {msec: u32, color: Option<&'a str>, text: &'a str},
}