use std::io;
use std::fs;
use std::path::Path;
use crate::toolbox::expand_path;

mod intent_getter;
pub mod lm_wrapper;
pub mod toolbox;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut intent_getter = intent_getter::IntentGetter::new(project_location.clone());
    intent_getter.get_intent().await?;
    Ok(())
}
