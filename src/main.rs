use std::io;
use std::fs;
use std::path::Path;
use shellexpand::full;
use crate::toolbox::{file_commands, test_commands, Toolbox, version_control, worker_commands};


mod worker;
pub mod lm_wrapper;
pub mod toolbox;
pub mod input_dialog;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the project location
    println!("Please specify the project location: ");
    let mut project_location = String::new();
    while project_location.is_empty() {
        io::stdin().read_line(&mut project_location)?;
        project_location = String::from(full(project_location.trim())?);
        if !Path::new(&project_location).exists() {
            println!("The location: {} does not exist. Please specify a valid location.", project_location);
            println!("Please specify the project location: ");
            project_location.clear();
        }
    }

    println!("-----------------------------------");
    println!("Selected project location: {}", project_location);
    println!("-----------------------------------");

    /*let mut intent_getter = intent_getter::IntentGetter::new(project_location.clone());
    intent_getter.get_intent().await?;*/
    let project_location = Path::new(&project_location).to_path_buf();

    let planner_behaviour = String::from("You are a planner of a coding project in Rust. You are responsible for creating a update_plan.md file in the project.
You will communicate with user who will give you instructions of what he wants changed in the project and you will generate and update the file accordingly containing detailed
development plan for the needed changes. You will create the file with specific development steps which can be checked by the manager when they're done. You will also describe in the
 document what needs to be tested and checked ");
    let mut toolbox = Toolbox::new(project_location.clone());
    toolbox.add_tools(file_commands::get_file_write_tools());
    toolbox.add_tools(file_commands::get_file_read_tools());

   // worker::talk_to_worker(planner_behaviour, project_location.clone(), toolbox)?;

    let manager_behaviour = String::from("You are a manager of a coding project in Rust. You are responsible for managing the project.\
You can call workers who can edit and manage files in the project for you. Your task is to communicate with the user and call these agents.
You also have to check and verify their work is correct and working. If not you call another worker to fix it and so on until the user is satisfied.
You have to give very specific instructions to the coding workers so there cannot be any ambiguity in their task descriptions. Give them only very small tasks as they can
only do a small chunk of coding at a time. Make sure to always check their work. You are also allowed to check their work yourself using read tools or test tools like compiling project.
One you are 100% sure the task has been done perfectly, you can also call git commands.");

    let mut toolbox = Toolbox::new(project_location.clone());
    toolbox.add_tools(worker_commands::get_worker_commands());
    toolbox.add_tools(file_commands::get_file_read_tools());
    toolbox.add_tools(test_commands::get_test_commands());
    toolbox.add_tools(version_control::get_version_control_commands());
    // pretty print the toolbox
    worker::talk_to_worker(manager_behaviour, project_location.clone(), toolbox)?;

    Ok(())
}
