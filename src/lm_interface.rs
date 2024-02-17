
use std::env;

use rust_gpt::{RequestBuilder, ChatModel, SendRequest, chat::*};
struct LMInterface{
    openai_api_key: String,
}
impl LMInterface{
    pub fn new()->Self{
        let openai_api_key = env::var("OPENAI_API_KEY").unwrap();
        LMInterface{
            openai_api_key,
        }
    }
    pub async fn send_message(&self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let request = rust_gpt::RequestBuilder::new(ChatModel::Gpt35Turbo, self.openai_api_key.clone())
            .messages(vec![ChatMessage{
                role: Role::User,
                content: Some("Create a readme.md for a checkers game (project) made in Rust. This readme will contain the files which the project is composed of and what each file does. Also it will describe in detail, how the project works as a whole and how can a user interact with it.".to_string()),
            }]).
            build_chat();

        let result = request.send().await.unwrap();
        let response = result.choices[0].message.content.as_ref().unwrap();
        println!("{response}");

        Ok(response.clone())
    }
}