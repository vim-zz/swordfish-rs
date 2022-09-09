use clap::{Parser};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author = "Ofer Affias", version, about, long_about = None)]
struct Cli {
    /// Screenplay file, YAML formatted list of commands and their args 
    #[clap(value_parser, value_hint = clap::ValueHint::FilePath)]
    file: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let data = fs::read_to_string(cli.file).expect("Unable to read screenplay file");
    let commands = swordfishlib::from_yaml(&data).expect("Parsing errors in screenplay file");
    swordfishlib::execute(commands).expect("Runtime error");
}
