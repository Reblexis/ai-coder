use std::io;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

pub trait Tool {
    fn execute(&self, parameters: &str) -> Result<String, std::io::Error>;
}

pub struct Toolbox {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl Toolbox {
    pub fn new() -> Self {
        let mut toolbox = Toolbox {
            tools: HashMap::new(),
        };

        toolbox
    }

    fn register_tool(&mut self, name: &str, tool: Box<dyn Tool>) {
        self.tools.insert(name.to_string(), tool);
    }

    pub fn call_tool(&self, tool_name: &str, parameters: &str) -> Result<String, std::io::Error> {
        match self.tools.get(tool_name) {
            Some(tool) => tool.execute(parameters),
            None => Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Tool not found")),
        }
    }
}