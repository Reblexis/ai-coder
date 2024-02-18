use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use shellexpand::full;

use openai_api_rs::v1::chat_completion::*;

pub struct Toolbox{
    project_location: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct RelativePath{
    path: String,
}

#[derive(Debug)]
pub struct PresentFiles{
    files: Vec<String>,
}

fn expand_path(path: &str) -> Result<PathBuf, std::io::Error> {
    let expanded_path = full(path)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to expand path"))?;
    Ok(PathBuf::from(expanded_path.as_ref()))
}

impl Toolbox{
    pub fn new(project_location: String)->Self{
        Toolbox{
            project_location: PathBuf::from(project_location),
        }
    }

    pub fn call_tool(&self, tool_name: String, parameters: String)->String{
        // TODO: Ensure that only allowed tools are called

        println!("Calling tool: {} with arguments: {}", tool_name, parameters);

        match tool_name.as_str() {
            "view_files" => {
                let path: RelativePath = serde_json::from_str(&parameters).unwrap();
                let files = self.view_files(path);
                let pretty_string = format!("{:#?}", files);
                pretty_string
            }
            _ => {
                String::from("Tool not found")
            }
        }
    }

    pub fn get_all_tools(&self)->Vec<Tool>{
        vec![
            self.get_view_files_tool(),
        ]
    }

    pub fn get_view_files_tool(&self)->Tool{
        let mut properties = HashMap::new();
        properties.insert(
            "path".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::String),
                description: Some("The relative path of the directory you want to view.".to_string()),
                ..Default::default()
            }),
        );
        Tool{
            r#type: ToolType::Function,
            function: Function{
                name: String::from("view_files"),
                description: Some(String::from("View information about the files in the directory.")),
                parameters: FunctionParameters{
                    schema_type: JSONSchemaType::Object,
                    properties: Some(properties),
                    required:Some(vec![String::from("path")]),
                }
            }
        }
    }

    pub fn view_files(&self, path: RelativePath) -> PresentFiles{
        let final_path = self.project_location.join(path.path);
        println!("Viewing files at: {}", final_path.display());

        let paths = fs::read_dir(final_path).unwrap();
        let mut files = Vec::new();
        for path in paths {
            files.push(path.unwrap().path().display().to_string());
        }
        PresentFiles{
            files,
        }
    }
}