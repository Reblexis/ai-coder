use super::*;
use std::io::ErrorKind;

mod compile;

pub fn get_test_commands() -> HashMap<String, Box<dyn Command>> {
    let mut tools:HashMap<String, Box<dyn Command>> = HashMap::new();
    tools.insert("compile".to_string(), Box::new(compile::CompileCommand{}));
    tools
}