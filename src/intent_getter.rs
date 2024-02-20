use std::io;
use std::collections::HashMap;
use std::env;
use async_recursion::async_recursion;

use openai_api_rs::v1::chat_completion::*;

use crate::lm_wrapper::LMInterface;
use crate::toolbox::Toolbox;

pub struct IntentGetter{
    pub lm: LMInterface,
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
        ], toolbox);

        IntentGetter{
            lm: lm_interface,
        }
    }

    pub async fn get_intent(& mut self)->Result<(), Box<dyn std::error::Error>>{
        println!("Please describe what do you want to change about this project.");

        let mut ended = false;

        while !ended {
            let mut user_message = String::new();
            io::stdin().read_line(&mut user_message)?;
            let result = self.lm.send_message(&user_message).await?;
            println!("{}", result);
        }

        Ok(())
    }
}

pub struct Coder{
    pub lm: LMInterface,
}

impl Coder{
    pub fn new(project_location: String)->Self{
        let mut toolbox = Toolbox::new(project_location.to_string());
        const SYSTEM_PROMPT: &str = "You are a coding ai. The user will give you a document path containing the information about the \
        update you should do to the project. If you think anything is unclear, ask. Otherwise you will implement step by step the changes, don't implement everything at once.
         Just update a few things and finish. You will have chance to update more later. After you do a few changes, inform the user.";

        let lm_interface = LMInterface::new(vec![
            ChatCompletionMessage{
                role: MessageRole::system,
                content: Content::Text(SYSTEM_PROMPT.to_string()),
                name: None,
            }
        ], toolbox);

        Coder{
            lm: lm_interface,
        }
    }

    pub async fn start_coding(& mut self)->Result<(), Box<dyn std::error::Error>>{
        println!("Please write the path of the document containing the information about the update you want to be done.");

        let mut ended = false;

        while !ended {
            let mut user_message = String::new();
            io::stdin().read_line(&mut user_message)?;
            let result = self.lm.send_message(&user_message).await?;
            println!("{}", result);
        }

        Ok(())
    }
}

