
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::*;
use openai_api_rs::v1::common::{GPT3_5_TURBO_0613, GPT4_0125_PREVIEW, GPT4_0613};
use std::env;

pub struct LMInterface{
    client: Client,
    messages: Vec<ChatCompletionMessage>,
    tools: Vec<Tool>,
}
impl LMInterface{
    pub fn new(messages: Vec<ChatCompletionMessage>, tools: Vec<Tool>)->Self{
        let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
        assert!(tools.len() > 0, "Please provide at least one tool!");
        LMInterface{
            client,
            messages,
            tools,
        }
    }
    pub fn add_message(&mut self, message: ChatCompletionMessage){
        self.messages.push(message);
    }
    pub async fn send_message(&mut self, message: &str) -> Result<ChatCompletionResponse, Box<dyn std::error::Error>> {
        self.messages.push(ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text(message.to_string()),
            name: None,
        });

        let req = ChatCompletionRequest::new(
            GPT4_0125_PREVIEW.to_string(),
            self.messages.clone(),
        )
            .tools(self.tools.clone())
            .tool_choice(ToolChoiceType::Auto);

        // pretty print the messages
        println!("{:#?}", self.messages);

        let result = self.client.chat_completion(req)?;

        self.messages.push(ChatCompletionMessage {
            role: MessageRole::assistant,
            content: Content::Text(result.choices[0].message.content.clone().unwrap_or("Error occured!".to_string())),
            name: None,
        });

        Ok(result)
    }
}