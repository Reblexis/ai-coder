use super::*;
use std::io::ErrorKind;

mod git_call;

pub fn get_version_control_commands() -> HashMap<String, Box<dyn Command>> {
    let mut tools:HashMap<String, Box<dyn Command>> = HashMap::new();
    tools.insert("git".to_string(), Box::new(git_call::GitCallCommand{}));
    tools
}
