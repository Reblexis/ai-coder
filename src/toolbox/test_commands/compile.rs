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

        if output.status.success() {
            Ok(String::from("Compilation succeeded."))
        } else {
            Err(Error::new(ErrorKind::Other, String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }

    fn get_tool_info(&self) -> Tool{
        // Tool info with function name, description, and no parameters
    }
}