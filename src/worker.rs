use openai_api_rs::v1::chat_completion::*;
use crate::lm_wrapper::LMInterface;
use crate::toolbox::Toolbox;
use crate::input_dialog::read_stdin;

pub struct Worker{
    pub lm: LMInterface,
}

impl Worker{
    pub fn new(project_location: String, behaviour: String)->Self{
        let mut toolbox = Toolbox::new(project_location);

        let lm_interface = LMInterface::new(vec![
            ChatCompletionMessage{
                role: MessageRole::system,
                content: Content::Text(behaviour),
                name: None,
            }
        ], toolbox);

        Worker{
            lm: lm_interface,
        }
    }

    pub async fn get_intent(& mut self)->Result<(), Box<dyn std::error::Error>>{
        println!("Please describe what do you want to change about this project.");

        let mut ended = false;
        while !ended {
            let user_message = read_stdin();
            if user_message.trim().is_empty() {
                println!("Empty message! Please write a valid message.");
                continue;
            }
            let result = self.lm.send_message(&user_message).await?;
            println!("{}", result);
        }


        Ok(())
    }
}

