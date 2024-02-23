use std::io;
use std::fs;
use std::path::Path;
use toolbox::file_commands::expand_path;


mod worker;
pub mod lm_wrapper;
pub mod toolbox;
pub mod input_dialog;
mod manager_agent;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Specify the project location
    println!("Please specify the project location: ");
    let mut project_location = String::new();
    while project_location.is_empty() {
        io::stdin().read_line(&mut project_location)?;
        project_location = String::from(expand_path(project_location.trim()).unwrap().to_str().unwrap());
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
    let manager_behaviour = String::from("You are a manager of a coding project in Rust. You are responsible for managing the project.\
You can call workers who can edit and manage files in the project for you. Your task is to communicate with the user and call these agents.
You also have to check and verify their work is correct and working. If not you call another worker to fix it and so on until the user is satisfied.
You have to give very specific instructions to the coding workers so there cannot be any ambiguity in their task descriptions. Give them only very small tasks as they can
only do a small chunk of coding at a time. Make sure to always check their work.");

    worker::talk_to_worker(manager_behaviour, project_location.clone())?;
    Ok(())
}
