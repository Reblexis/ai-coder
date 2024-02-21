use super::super::toolbox::{Tool};
use super::super::file_tools::expand_path;
use std::fs;
use std::io::Error;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

pub struct ViewFilesTool;

#[derive(Serialize, Deserialize)]
pub struct ViewFilesParams {
    path: String,
}

impl Tool for ViewFilesTool {
    fn execute(&self, parameters: &str) -> Result<String, Error> {
        // Deserialize the parameters
        let params: ViewFilesParams = serde_json::from_str(parameters)?;
        let path = params.path;

        // Perform the operation
        let mut output = String::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            output.push_str(&format!("{} -- {}\n", entry.path().display(), if file_type.is_dir() { "dir" } else { "file" }));
        }

        Ok(output)
    }
}