use super::*;

pub struct RunTestsCommand;

#[derive(Serialize, Deserialize)]
pub struct RunTestsParams {}

impl Command for RunTestsCommand {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        let output = std::process::Command::new("cargo")
            .arg("test")
            .current_dir(&project_location)
            .output()?;

        if output.status.success() {
            Ok(String::from("Tests succeeded."))
        } else {
            Err(Error::new(ErrorKind::Other, String::from_utf8_lossy(&output.stderr).to_string()))
        }
    }

    fn get_tool_info(&self) -> Tool{
        // Tool info with function name, description, and no parameters
    }
}