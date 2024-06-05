use super::*;

pub struct GitCallCommand;

#[derive(Serialize, Deserialize)]
pub struct GitCallParams {
    command: String,
}

impl Command for GitCallCommand {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        // Deserialize the parameters
        let params: GitCallParams = serde_json::from_str(parameters)?;
        let command = params.command;

        // Perform the operation
        let output = std::process::Command::new("git")
            .arg(command)
            .current_dir(&project_location)
            .output()?;

        // Return the compilation result
        if output.status.success() {
            // Join stdout and stderr
            let feedback = String::from_utf8_lossy(&output.stdout).to_string() + &String::from_utf8_lossy(&output.stderr).to_string();
            Ok(feedback)
        } else {
            Err(Error::new(ErrorKind::Other, String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }

    fn get_tool_info(&self) -> Tool {
        let mut properties = HashMap::new();
        properties.insert(
            "args".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::String),
                description: Some("The arguments you want to pass to git. Fx. with args='commit -m 'great commit'' will be called 'git commit -m 'great commit''." .to_string()),
                ..Default::default()
            }),
        );
        Tool{
            r#type: ToolType::Function,
            function: Function{
                name: String::from("git"),
                description: Some(String::from("Call any git command with any parameters.")),
                parameters: FunctionParameters{
                    schema_type: JSONSchemaType::Object,
                    properties: Some(properties),
                    required:Some(vec![String::from("args")]),
                }
            }
        }
    }
}
