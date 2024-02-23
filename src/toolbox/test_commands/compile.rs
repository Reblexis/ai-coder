use super::*;

pub struct CompileCommand;

#[derive(Serialize, Deserialize)]
pub struct CompileParams {}

impl Command for CompileCommand {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        let output = std::process::Command::new("cargo")
            .arg("build")
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
        Tool {
            r#type: ToolType::Function,
            function: Function {
                name: String::from("compile"),
                description: Some(String::from("Tries to compile the current project using the cargo build command. If it doesn't succeed, it will return the error message.")),
                parameters: FunctionParameters {
                    schema_type: JSONSchemaType::Object,
                    properties: None,
                    required: None,
                },
            }
        }
    }
}