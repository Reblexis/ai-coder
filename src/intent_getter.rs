use std::io;
use std::collections::HashMap;
use std::env;

use openai_api_rs::v1::chat_completion::*;

use crate::lm_wrapper::LMInterface;
use crate::toolbox::Toolbox;

pub struct IntentGetter{
    pub lm: LMInterface,
    pub toolbox: Toolbox,
}

impl IntentGetter{
    pub fn new(project_location: String)->Self{
        let mut toolbox = Toolbox::new(project_location.to_string());
        const SYSTEM_PROMPT: &str = "You are a coding ai. However your current task is just to discover what exactly does the user want to change about the project.
He will give you a description of what he wants, and you will according to that generate a document describing how it would work in detail and what exactly would be changed. If he is not satisfied, you iterate over the process again.
Once he is satisfied, you will call the function 'edit_description' and pass the new description as an argument. This will change the description of the project update to the new one you generated.";

        let lm_interface = LMInterface::new(vec![
            ChatCompletionMessage{
                role: MessageRole::system,
                content: Content::Text(SYSTEM_PROMPT.to_string()),
                name: None,
            }
        ],
        toolbox.get_all_tools()
        );

        IntentGetter{
            lm: lm_interface,
            toolbox,
        }
    }

    pub async fn get_intent(& mut self)->Result<(), Box<dyn std::error::Error>>{
        println!("Please describe what do you want to change about this project.");

        let mut ended = false;

        while !ended {
            let mut user_message = String::new();
            io::stdin().read_line(&mut user_message)?;
            let result = self.lm.send_message(&user_message).await?;

            match result.choices[0].finish_reason {
                None => {
                    println!("No finish_reason");
                    println!("{}", result.choices[0].message.content.clone().unwrap());
                }
                Some(FinishReason::stop) => {
                    println!("Stop");
                    println!("{}", result.choices[0].message.content.clone().unwrap());
                }
                Some(FinishReason::length) => {
                    println!("Length");
                }
                Some(FinishReason::tool_calls) => {
                    println!("ToolCalls");

                    let tool_calls = result.choices[0].message.tool_calls.as_ref().unwrap();
                    for tool_call in tool_calls {
                        let name = tool_call.function.name.clone().unwrap();
                        let args = tool_call.function.arguments.clone().unwrap();

                        self.toolbox.call_tool(name, args);
                    }
                }
                Some(FinishReason::content_filter) => {
                    println!("ContentFilter");
                }
                Some(FinishReason::null) => {
                    println!("Null");
                }
            }
        }

        Ok(())
    }
}

