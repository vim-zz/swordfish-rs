use colored::Colorize;

pub const BACKSPACE: char = 8u8 as char;

pub fn colorful_print(c: &str, color: Option<&str>) {
    match color {
        Some("black") => print!("{}", c.black()),
        Some("red") => print!("{}", c.red()),
        Some("green") => print!("{}", c.green()),
        Some("yellow") => print!("{}", c.yellow()),
        Some("blue") => print!("{}", c.blue()),
        Some("magenta") => print!("{}", c.magenta()),
        Some("cyan") => print!("{}", c.cyan()),
        Some("white") => print!("{}", c.white()),
        Some("bright_black") => print!("{}", c.bright_black()),
        Some("bright_red") => print!("{}", c.bright_red()),
        Some("bright_green") => print!("{}", c.bright_green()),
        Some("bright_yellow") => print!("{}", c.bright_yellow()),
        Some("bright_blue") => print!("{}", c.bright_blue()),
        Some("bright_magenta") => print!("{}", c.bright_magenta()),
        Some("bright_cyan") => print!("{}", c.bright_cyan()),
        Some("bright_white") => print!("{}", c.bright_white()),
        Some(_) | None => print!("{c}"),
    }
}