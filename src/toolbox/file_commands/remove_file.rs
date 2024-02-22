use super::*;


pub struct RemoveFileCommand;

#[derive(Serialize, Deserialize)]
pub struct RemoveFileParams {
    path: String,
}

impl Command for RemoveFileCommand {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        // Deserialize the parameters
        let params: RemoveFileParams = serde_json::from_str(parameters)?;
        let binding = expand_path(project_location.join(params.path).to_str().unwrap())?;
        let path = binding.to_str().unwrap();

        // Perform the operation
        fs::remove_file(path)?;

        Ok("Successfully removed file.".to_string())
    }

    fn get_tool_info(&self) -> Tool{
        let mut properties = HashMap::new();
        properties.insert(
            "path".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::String),
                description: Some("The relative path of the file you want to delete.".to_string()),
                ..Default::default()
            }),
        );
        Tool{
            r#type: ToolType::Function,
            function: Function{
                name: String::from("remove_file"),
                description: Some(String::from("Delete a file from the project.")),
                parameters: FunctionParameters{
                    schema_type: JSONSchemaType::Object,
                    properties: Some(properties),
                    required:Some(vec![String::from("path")]),
                }
            }
        }
    }
}
