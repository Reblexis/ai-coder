use openai_api_rs::v1::chat_completion::*;
use crate::lm_wrapper::LMInterface;
use crate::toolbox::Toolbox;
use crate::toolbox::{file_commands, worker_commands, test_commands, version_control};
use crate::input_dialog::read_stdin;
use std::path::{Path, PathBuf};

pub struct Worker{
    pub lm: LMInterface,
}

impl Worker{
    pub fn new(project_location: PathBuf, behaviour: String, toolbox: Toolbox)->Self{
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

    pub fn send_message(&mut self, message: &str)->Result<String, Box<dyn std::error::Error>>{
        let result = self.lm.send_message(message)?;
        Ok(result)
    }
}

pub fn talk_to_worker(behaviour: String, project_location: PathBuf)->Result<(), Box<dyn std::error::Error>>{
    let mut toolbox = Toolbox::new(project_location.clone());
    toolbox.add_tools(worker_commands::get_worker_commands());
    toolbox.add_tools(file_commands::get_file_read_tools());
    toolbox.add_tools(test_commands::get_test_commands());
    toolbox.add_tools(version_control::get_version_control_commands());
    let mut worker = Worker::new(project_location, behaviour, toolbox);
    let mut ended = false;
    while !ended {
        let user_message = read_stdin();
        if user_message.trim().is_empty() {
            println!("Empty message! Please write a valid message.");
            continue;
        }
        let result = worker.send_message(&user_message)?;
        println!("{}", result);
    }
    Ok(())
}

pub fn call_worker(message: String, project_location: PathBuf)->Result<String, Box<dyn std::error::Error>>{
    let mut toolbox = Toolbox::new(project_location.clone());
    toolbox.add_tools(file_commands::get_all_file_tools());
    let system_message = String::from("You are a worker whose task is to complete the task you are given by a message.\
     You are working on a coding project in Rust.\
     If you think the task is too long and that you wouldn't be able to complete it in one go, report that back to the user.\
      Don't ask any questions, only report back. You won't get any response. The user is able to send only the initial message.");
    let mut worker = Worker::new(project_location.clone(), system_message, toolbox);
    let result = worker.send_message(&message)?;

    Ok(result)
}
