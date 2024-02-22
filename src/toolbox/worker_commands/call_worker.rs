use super::*;
use crate::lm_wrapper::LMInterface;
use tokio::runtime::Runtime;
use crate::worker;

pub struct CallWorkerCommand;

#[derive(Serialize, Deserialize)]
pub struct CallWorkerParams {
    system_message: String,
}

impl Command for CallWorkerCommand {
    fn execute(&self, parameters: &str, project_location: PathBuf) -> Result<String, Error> {
        // Deserialize the parameters
        let params: CallWorkerParams = serde_json::from_str(parameters)?;

        // Call the worker
        let mut rt = Runtime::new().unwrap();
        let result = rt.block_on(worker::call_worker(params.system_message, project_location));

        let pretty_result = format!("{:#?}", result);
        Ok({pretty_result})
    }

    fn get_tool_info(&self) -> Tool{
        let mut properties = HashMap::new();
        properties.insert(
            "system_message".to_string(),
            Box::new(JSONSchemaDefine {
                schema_type: Some(JSONSchemaType::String),
                description: Some("The system message, you want to send to the worker. You should describe precisely the task you want the agent to do\
                 or where it can find the task description.".to_string()),
                ..Default::default()
            }),
        );
        Tool{
            r#type: ToolType::Function,
            function: Function{
                name: String::from("call_worker"),
                description: Some(String::from("Calls a worker which can do a task for you. He can edit, create or manipulate files. \
                He can create code according to instructions but only in a small chunk. Don't give him big tasks.")),
                parameters: FunctionParameters{
                    schema_type: JSONSchemaType::Object,
                    properties: Some(properties),
                    required:Some(vec![String::from("system_message")]),
                }
            }
        }

    }
}
