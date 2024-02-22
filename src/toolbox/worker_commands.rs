use super::*;

mod call_worker;


pub fn get_worker_commands() -> HashMap<String, Box<dyn Command>> {
    let mut tools:HashMap<String, Box<dyn Command>> = HashMap::new();
    tools.insert("call_worker".to_string(), Box::new(call_worker::CallWorkerCommand{}));
    tools
}