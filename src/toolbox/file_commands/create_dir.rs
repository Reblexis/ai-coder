use super::*;

pub struct CreateDirCommand;

#[derive(Serialize, Deserialize)]
pub struct CreateDirParams {
    path: String,
}

impl Command for CreateDirCommand {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        // Deserialize the parameters
        let params: CreateDirParams = serde_json::from_str(parameters)?;
        let path = expand_path(project_location, params.path.as_str())?;

        // Perform the operation
        fs::create_dir(path)?;

        Ok("Successfully created directory.".to_string())
    }

    fn get_tool_info(&self) -> Tool{
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
}
