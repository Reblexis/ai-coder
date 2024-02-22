use super::*;


pub struct EditFileCommand;

#[derive(Serialize, Deserialize)]
pub struct EditFileParams{
    path: String,
    start_line: usize,
    end_line: usize,
    new_contents: String,
}

impl Command for EditFileCommand {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        // Deserialize the parameters
        let params: EditFileParams = serde_json::from_str(parameters)?;
        let binding = expand_path(project_location.join(params.path).to_str().unwrap())?;
        let path = binding.to_str().unwrap();

        let contents = fs::read_to_string(path.clone())?;
        let mut lines: Vec<&str> = contents.lines().collect();

        lines.splice((params.start_line-1)..(params.end_line-1), params.new_contents.lines());
        let new_contents = lines.join("\n");
        fs::write(path, new_contents)?;
        Ok("Successfully edited file.".to_string())
    }

    fn get_tool_info(&self) -> Tool{
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
}
