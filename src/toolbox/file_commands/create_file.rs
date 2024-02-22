use super::*;

pub struct CreateFileCommand;

#[derive(Serialize, Deserialize)]
pub struct CreateFileParams {
    path: String,
    contents: String,
}

impl Command for CreateFileCommand {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        // Deserialize the parameters
        let params: CreateFileParams = serde_json::from_str(parameters)?;
        let path = expand_path(project_location.join(params.path).to_str()?)?.to_str()?;

        // Perform the operation
        fs::write(path, params.contents)?;

        Ok("Successfully created directory.".to_string())
    }

    fn get_tool_info(&self) -> Tool{
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
}
