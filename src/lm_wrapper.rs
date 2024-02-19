
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::*;
use openai_api_rs::v1::common::{GPT3_5_TURBO_0613, GPT4_0125_PREVIEW, GPT4_0613};
use std::env;
use async_recursion::async_recursion;

use crate::toolbox::Toolbox;

pub struct LMInterface{
    client: Client,
    messages: Vec<ChatCompletionMessage>,
    toolbox: Toolbox,
}
impl LMInterface{
    pub fn new(messages: Vec<ChatCompletionMessage>, toolbox: Toolbox)->Self{
        let client = Client::new(env::var("OPENAI_API_KEY").unwrap().to_string());
        LMInterface{
            client,
            messages,
            toolbox,
        }
    }
    pub fn add_message(&mut self, message: ChatCompletionMessage){
        self.messages.push(message);
    }
    #[async_recursion]
    pub async fn respond(&mut self)-> Result<String, Box<dyn std::error::Error>>{
        let req = ChatCompletionRequest::new(
            GPT3_5_TURBO_0613.to_string(),
            self.messages.clone(),
        )
            .tools(self.toolbox.get_all_tools())
            .tool_choice(ToolChoiceType::Auto);

        // pretty print the messages
        println!("{:#?}", self.messages);

        let result = self.client.chat_completion(req)?;


        let mut answer: String = "".to_string();
        match result.choices[0].finish_reason {
            Some(FinishReason::tool_calls) => {
                let tool_calls = result.choices[0].message.tool_calls.as_ref().unwrap();
                for tool_call in tool_calls {
                    let name = tool_call.function.name.clone().unwrap();
                    let args = tool_call.function.arguments.clone().unwrap();

                    self.messages.push(ChatCompletionMessage {
                        role: MessageRole::function,
                        content: Content::Text(format!("Calling tool: {} with arguments: {}", name, args)),
                        name: Some(name.clone()),
                    });

                    let ans = self.toolbox.call_tool(name.clone(), args);
                    self.messages.push(ChatCompletionMessage {
                        role: MessageRole::function,
                        content: Content::Text(ans),
                        name: Some(name.clone()),
                    });

                    answer = self.respond().await?;
                }
            }
            _ => {
                self.messages.push(ChatCompletionMessage {
                    role: MessageRole::assistant,
                    content: Content::Text(result.choices[0].message.content.clone().unwrap_or("Error occured!".to_string())),
                    name: None,
                });
                return Ok(result.choices[0].message.content.clone().unwrap_or("Error occured!".to_string()));
            }
        }
        println!("{:#?}", self.messages);
        Ok(answer)
    }
    #[async_recursion]
    pub async fn send_message(&mut self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.messages.push(ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text(message.to_string()),
            name: None,
        });
        return self.respond().await;
    }
}