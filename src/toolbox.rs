use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
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

#[derive(Serialize, Deserialize)]
pub struct WriteFile{
    path: String,
    contents: String,
}

#[derive(Debug)]
pub struct PresentFiles{
    files: Vec<String>,
}

#[derive(Debug)]
pub struct FileContents {
    contents: String,
}

#[derive(Serialize, Deserialize)]
pub struct EditFileInfo{
    path: String,
    start_line: usize,
    end_line: usize,
    new_contents: String,
}

pub fn expand_path(path: &str) -> Result<PathBuf, std::io::Error> {
    let expanded_path = full(path)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Failed to expand path"))?;
    Ok(PathBuf::from(expanded_path.as_ref()))
}

impl Toolbox{
    pub fn new(project_location: String)->Self{
        Toolbox{
            project_location: PathBuf::from(expand_path(&project_location).unwrap()),
        }
    }

    pub fn call_tool(&self, tool_name: String, parameters: String)->String{
        // TODO: Ensure that only allowed tools are called

        println!("Calling tool: {} with arguments: {}", tool_name, parameters);

        match tool_name.as_str() {
            "view_files" => {
                let files = self.view_files(parameters);
                let pretty_string = format!("{:#?}", files);
                pretty_string
            }
            "read_file" => {
                let contents = self.read_file(parameters);
                let pretty_string = format!("{:#?}", contents);
                pretty_string
            }
            "create_file"=>{
                let contents = self.create_file(parameters);
                let pretty_string = format!("{:#?}", contents);
                pretty_string
            }
            "create_dir" => {
                let contents = self.create_dir(parameters);
                let pretty_string = format!("{:#?}", contents);
                pretty_string
            }
            "edit_file" => {
                let contents = self.edit_file(parameters);
                let pretty_string = format!("{:#?}", contents);
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
            self.get_read_file_tool(),
            self.get_create_file_tool(),
            self.get_create_dir_tool(),
            self.get_edit_file_tool(),
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

    pub fn get_read_file_tool(&self)->Tool{
        let mut properties = HashMap::new();
        properties.insert(
            "path".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::String),
                description: Some("The relative path of the file you want to read.".to_string()),
                ..Default::default()
            }),
        );
        Tool{
            r#type: ToolType::Function,
            function: Function{
                name: String::from("read_file"),
                description: Some(String::from("Read the contents of a file with line numbers. The numbers aren't part of the file. They are added for edit purposes.")),
                parameters: FunctionParameters{
                    schema_type: JSONSchemaType::Object,
                    properties: Some(properties),
                    required:Some(vec![String::from("path")]),
                }
            }
        }
    }

    pub fn get_create_file_tool(&self)->Tool{
        let mut properties = HashMap::new();
        properties.insert(
            "path".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::String),
                description: Some("The relative path of the file you want to create".to_string()),
                ..Default::default()
            }),
        );
        properties.insert(
            "contents".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::String),
                description: Some("The contents you want to write to the file. Avoid writing line numbers.".to_string()),
                ..Default::default()
            }),
        );
        Tool{
            r#type: ToolType::Function,
            function: Function{
                name: String::from("create_file"),
                description: Some(String::from("Create a file and write the contents. Avoid writing line numbers")),
                parameters: FunctionParameters{
                    schema_type: JSONSchemaType::Object,
                    properties: Some(properties),
                    required:Some(vec![String::from("path"), String::from("contents")]),
                }
            }
        }
    }

    pub fn get_create_dir_tool(&self)->Tool{
        let mut properties = HashMap::new();
        properties.insert(
            "path".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::String),
                description: Some("The relative path of the directory you want to create.".to_string()),
                ..Default::default()
            }),
        );
        Tool{
            r#type: ToolType::Function,
            function: Function{
                name: String::from("create_dir"),
                description: Some(String::from("Create a directory.")),
                parameters: FunctionParameters{
                    schema_type: JSONSchemaType::Object,
                    properties: Some(properties),
                    required:Some(vec![String::from("path")]),
                }
            }
        }
    }

    pub fn get_edit_file_tool(&self)->Tool{
        let mut properties = HashMap::new();
        properties.insert(
            "path".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::String),
                description: Some("The relative path of the file you want to edit.".to_string()),
                ..Default::default()
            }),
        );
        properties.insert(
            "start_line".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::Number),
                description: Some("The index (starting with 1) of the start line of the line range you will be replacing.".to_string()),
                ..Default::default()
            }),
        );
        properties.insert(
            "end_line".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::Number),
                description: Some("The index (starting with 1) of the first line you will not reach in the interval, the interval is [start_line, end_line).".to_string()),
                ..Default::default()
            }),
        );
        properties.insert(
            "new_contents".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::String),
                description: Some("The new contents you want to replace the interval with.".to_string()),
                ..Default::default()
            }),
        );
        Tool{
            r#type: ToolType::Function,
            function: Function{
                name: String::from("edit_file"),
                description: Some(String::from("Replace lines of the given file in a specified range [start_line, end_line) with new_contents. The new contents can have more or less lines than the interval size (even 0)")),
                parameters: FunctionParameters{
                    schema_type: JSONSchemaType::Object,
                    properties: Some(properties),
                    required:Some(vec![String::from("path"), String::from("start_line"), String::from("end_line"), String::from("new_contents")]),
                }
            }
        }
    }

    fn expand_path(&self, path: &str)->Result<String, std::io::Error>{
        let final_path = self.project_location.join(path);
        let expanded_path = expand_path(final_path.to_str().unwrap()).unwrap();
        if !expanded_path.exists(){
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Path does not exist"));
        }
        if !expanded_path.starts_with(&self.project_location){
            return Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Path is not within the project location, please type only local paths."));
        }
        Ok(expanded_path.display().to_string())
    }

    pub fn view_files(&self, parameters: String) -> Result<PresentFiles, std::io::Error>{
        let info: RelativePath = serde_json::from_str(&parameters)?;

        let final_path = self.expand_path(&info.path)?;

        let paths = fs::read_dir(final_path)?;
        let mut files = Vec::new();
        for file in paths {
            match file {
                Ok(file) => {
                    // join path.path and file_name
                    let local_path = Path::new(&info.path.clone()).join(file.file_name().clone());
                    let file_type = file.file_type()?;
                    files.push(format!("{} -- {}", local_path.to_str().unwrap(), if file_type.is_dir(){"dir"} else {"file"}));
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(PresentFiles{
            files,
        })
    }

    pub fn read_file(&self, parameters: String) -> Result<FileContents, std::io::Error>{
        let info: RelativePath = serde_json::from_str(&parameters)?;

        let final_path = expand_path(self.project_location.join(info.path).to_str().unwrap())?;

        // Read line by line and then append line number to the start of each line and then join them into tring again (with newline char)
        let contents = fs::read_to_string(final_path)?;
        let contents = contents.lines().enumerate().map(|(i, line)| format!("{}. {}", i+1, line)).collect::<Vec<String>>().join("\n");
        Ok(FileContents{
            contents,
        })
    }

    pub fn create_file(&self, parameters: String) -> Result<(), std::io::Error>{
        let info: WriteFile = serde_json::from_str(&parameters)?;
        let final_path = expand_path(self.project_location.join(info.path).to_str().unwrap())?;

        fs::write(final_path, info.contents)?;
        Ok(())
    }

    pub fn create_dir(&self, parameters: String) -> Result<(), std::io::Error>{
        let info: RelativePath = serde_json::from_str(&parameters)?;
        let final_path = expand_path(self.project_location.join(info.path).to_str().unwrap())?;

        fs::create_dir(final_path)?;
        Ok(())
    }

    pub fn edit_file(&self, parameters: String) -> Result<(), std::io::Error>{
        let info: EditFileInfo = serde_json::from_str(&parameters)?;
        let final_path = expand_path(self.project_location.join(info.path).to_str().unwrap())?;

        let contents = fs::read_to_string(final_path.clone())?;
        let mut lines: Vec<&str> = contents.lines().collect();

        lines.splice((info.start_line-1)..(info.end_line-1), info.new_contents.lines());
        let new_contents = lines.join("\n");
        fs::write(final_path, new_contents)?;
        Ok(())
    }
}