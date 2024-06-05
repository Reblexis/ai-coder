use super::*;

pub struct CargoCommand;

#[derive(Serialize, Deserialize)]
pub struct CargoParams {
    command: String,
}

impl Command for CargoCommand{
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        // Deserialize the parameters
        let params: CargoParams = serde_json::from_str(parameters)?;
        let command = params.command;

        let output = std::process::Command::new("cargo")
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
                description: Some("The arguments you want to pass to cargo. Fx. with args='run' will be called 'cargo run'." .to_string()),
                ..Default::default()
            }),
        );
        Tool {
            r#type: ToolType::Function,
            function: Function {
                name: String::from("cargo"),
                description: Some(String::from("Calls cargo with the given arguments. Can be used for compilation, running tests, etc.")),
                parameters: FunctionParameters{
                    schema_type: JSONSchemaType::Object,
                    properties: Some(properties),
                    required:Some(vec![String::from("args")]),
                }
            }
        }
    }
}