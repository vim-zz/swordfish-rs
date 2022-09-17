use swordfishlib;
use std::fs;

fn main() {
    let data = fs::read_to_string("examples/intro.yaml").expect("Unable to read screenplay file");
    let commands = swordfishlib::from_yaml(&data).expect("Parsing errors in screenplay file");
    swordfishlib::execute(commands).unwrap();
}
