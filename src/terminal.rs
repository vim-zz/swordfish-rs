use colored::Colorize;

pub const BACKSPACE: char = 8u8 as char;

pub fn colorful(c: &str, color: Option<&str>) -> String {
    match color {
        Some("black") => format!("{}", c.black()),
        Some("red") => format!("{}", c.red()),
        Some("green") => format!("{}", c.green()),
        Some("yellow") => format!("{}", c.yellow()),
        Some("blue") => format!("{}", c.blue()),
        Some("magenta") => format!("{}", c.magenta()),
        Some("cyan") => format!("{}", c.cyan()),
        Some("white") => format!("{}", c.white()),
        Some("bright_black") => format!("{}", c.bright_black()),
        Some("bright_red") => format!("{}", c.bright_red()),
        Some("bright_green") => format!("{}", c.bright_green()),
        Some("bright_yellow") => format!("{}", c.bright_yellow()),
        Some("bright_blue") => format!("{}", c.bright_blue()),
        Some("bright_magenta") => format!("{}", c.bright_magenta()),
        Some("bright_cyan") => format!("{}", c.bright_cyan()),
        Some("bright_white") => format!("{}", c.bright_white()),
        Some(_) | None => format!("{c}"),
    }
}