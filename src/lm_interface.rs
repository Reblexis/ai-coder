
use std::env;

use rust_gpt::{RequestBuilder, ChatModel, SendRequest, chat::*};
struct LMInterface{
    openai_api_key: String,
    messages: Vec<ChatMessage>,
}
impl LMInterface{
    pub fn new()->Self{
        let openai_api_key = env::var("OPENAI_API_KEY").unwrap();
        LMInterface{
            openai_api_key,
            messages: vec![],
        }
    }
    pub async fn send_message(&mut self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.messages.push(ChatMessage {
            role: Role::User,
            content: Some(message.to_string()),
        });

        let request = RequestBuilder::new(ChatModel::Gpt35Turbo, self.openai_api_key.clone())
            .messages(self.messages.clone())
            .build_chat();
        let result = request.send().await.unwrap();
        let response = result.choices[0].message.content.as_ref().unwrap();
        Ok(response.clone())
    }
}