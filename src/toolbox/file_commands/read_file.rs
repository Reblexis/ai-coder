use super::*;


pub struct ReadFileCommand;

#[derive(Serialize, Deserialize)]
pub struct ReadFileParams {
    path: String,
}

impl Command for ReadFileCommand {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        // Deserialize the parameters
        let params: ReadFileParams = serde_json::from_str(parameters)?;
        let binding = expand_path(project_location.join(params.path).to_str().unwrap())?;
        let path = binding.to_str().unwrap();


        let contents = fs::read_to_string(path)?;
        let contents = contents.lines().enumerate().map(|(i, line)| format!("{}. {}", i+1, line)).collect::<Vec<String>>().join("\n");

        Ok(contents)
    }

    fn get_tool_info(&self) -> Tool{
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
}
