
use openai_api_rs::v1::api::Client;
use openai_api_rs::v1::chat_completion::*;
use openai_api_rs::v1::common::{GPT3_5_TURBO_0613, GPT4_0125_PREVIEW, GPT4_0613};
use std::env;
use async_recursion::async_recursion;
use async_std::task;
use std::time::Duration;

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
    pub async fn respond(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let req = ChatCompletionRequest::new(
            GPT4_0125_PREVIEW.to_string(),
            self.messages.clone(),
        )
            .tools(self.toolbox.get_all_tools())
            .tool_choice(ToolChoiceType::Auto);

        // Loop until a successful API call
        let response = loop {
            match self.client.chat_completion(req.clone()) {
                Ok(response) => break response, // Exit loop on success
                Err(e) => {
                    println!("Error occurred: {}", e);
                    task::sleep(Duration::from_secs(20)).await;
                    // Optionally, consider adding a retry limit and error handling here
                }
            }
        };

        // Process response
        if let Some(FinishReason::tool_calls) = response.choices[0].finish_reason {
            let tool_calls = response.choices[0].message.tool_calls.as_ref().unwrap();
            for tool_call in tool_calls {
                let name = tool_call.function.name.clone().unwrap();
                let args = tool_call.function.arguments.clone().unwrap();

                self.messages.push(ChatCompletionMessage {
                    role: MessageRole::system,
                    content: Content::Text(format!("Calling tool: {} with arguments: {}", name, args)),
                    name: Some(name.clone()),
                });

                let ans = self.toolbox.call_tool(name.as_str(), args.as_str())?;
                self.messages.push(ChatCompletionMessage {
                    role: MessageRole::function,
                    content: Content::Text(ans),
                    name: Some(name.clone()),
                });

                // Recursively call respond, consider redesign to avoid deep recursion
                return self.respond().await;
            }
        }

        self.messages.push(ChatCompletionMessage {
            role: MessageRole::assistant,
            content: Content::Text(response.choices[0].message.content.clone().unwrap_or_default()),
            name: None,
        });

        Ok(response.choices[0].message.content.clone().unwrap_or_default())
    }
    #[async_recursion]
    pub async fn send_message(&mut self, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        self.messages.push(ChatCompletionMessage {
            role: MessageRole::user,
            content: Content::Text(message.to_string()),
            name: None,
        });
        let response = self.respond().await;
        println!("{:#?}", self.messages);
        return response;
    }
}