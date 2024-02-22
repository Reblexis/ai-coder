use super::*;


pub struct ViewFilesCommand;

#[derive(Serialize, Deserialize)]
pub struct ViewFilesParams {
    path: String,
}

impl Command for ViewFilesCommand {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        // Deserialize the parameters
        let params: ViewFilesParams = serde_json::from_str(parameters)?;
        let binding = expand_path(project_location.join(params.path).to_str().unwrap())?;
        let path = binding.to_str().unwrap();

        // Perform the operation
        let mut output = String::new();
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            output.push_str(&format!("{} -- {}\n", entry.path().display(), if file_type.is_dir() { "dir" } else { "file" }));
        }

        Ok(output)
    }

    fn get_tool_info(&self) -> Tool{
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
}