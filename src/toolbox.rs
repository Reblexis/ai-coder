pub mod file_commands;
mod worker_commands;

use std::fs;
use std::io::Error;
use openai_api_rs::v1::chat_completion::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};
use openai_api_rs::v1::chat_completion::*;

pub trait Command {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, std::io::Error>;
    fn get_tool_info(&self) -> Tool;
}

pub struct Toolbox {
    tools: HashMap<String, Box<dyn Command>>,
    project_location: PathBuf,
}

impl Toolbox {
    pub fn new(project_location: PathBuf) -> Self {
        let mut toolbox = Toolbox {
            tools: HashMap::new(),
            project_location,
        };

        toolbox
    }

    fn register_tool(&mut self, name: &str, tool: Box<dyn Command>) {
        self.tools.insert(name.to_string(), tool);
    }

    pub fn call_tool(&self, tool_name: &str, parameters: &str) -> Result<String, std::io::Error> {
        match self.tools.get(tool_name) {
            Some(tool) => tool.execute(parameters, self.project_location.clone()),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Tool not found")),
        }
    }
}
