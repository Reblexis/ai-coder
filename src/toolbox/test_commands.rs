use super::*;
use std::io::ErrorKind;

mod cargo;

pub fn get_test_commands() -> HashMap<String, Box<dyn Command>> {
    let mut tools:HashMap<String, Box<dyn Command>> = HashMap::new();
    tools.insert("cargo".to_string(), Box::new(cargo::CargoCommand{}));
    tools
}