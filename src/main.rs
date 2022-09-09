use std::fs;
use std::path::PathBuf;
use clap::{Parser};


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Screenplay file
    #[clap(value_parser, value_hint = clap::ValueHint::FilePath)]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let data = fs::read_to_string(cli.file).expect("Unable to read screenplay file");
    let commands = swordfishlib::from_yaml(&data).expect("Parsing errors in screenplay file");
    swordfishlib::execute(commands);
}
