use anyhow::anyhow;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{AiAdapter, AiService, Message};

pub struct OpenAIAdapter {
    host: String,
    api_key: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize)]
pub struct OpenAIChatCompletionRequest {
    pub model: String,
    pub messages: Vec<OpenAIMessage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenAIMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<OpenAIChoice>,
    pub usage: OpenAIUsage,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIChoice {
    pub index: u32,
    pub message: OpenAIMessage,
    pub logprobs: Option<i64>,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub completion_tokens_details: Option<OpenAICompletionTokensDetails>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAICompletionTokensDetails {
    pub reasoning_tokens: u32,
}

impl OpenAIAdapter {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            // host: "https://api.openai.com/v1".to_string(),
            host: "https://api.chatanywhere.tech".to_string(), // use for chinese
            api_key: api_key.into(),
            model: model.into(),
            client: Client::new(),
        }
    }
}

impl AiService for OpenAIAdapter {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String> {
        let request = OpenAIChatCompletionRequest {
            model: self.model.clone(),
            messages: messages.iter().map(|m| m.into()).collect(),
        };
        let url = format!("{}/chat/completions", self.host);
        let response = self
            .client
            .post(url)
            .json(&request)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        let mut data: OpenAIChatCompletionResponse = response.json().await?;
        let content = data
            .choices
            .pop()
            .ok_or(anyhow!("No Response"))?
            .message
            .content;
        Ok(content)
    }
}

impl From<Message> for OpenAIMessage {
    fn from(m: Message) -> Self {
        OpenAIMessage {
            role: m.role.to_string(),
            content: m.content,
        }
    }
}

impl From<&Message> for OpenAIMessage {
    fn from(m: &Message) -> Self {
        OpenAIMessage {
            role: m.role.to_string(),
            content: m.content.clone(),
        }
    }
}

impl From<OpenAIAdapter> for AiAdapter {
    fn from(adapter: OpenAIAdapter) -> Self {
        AiAdapter::OpenAI(adapter)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::Role;

    use super::*;

    #[ignore = "OPENAI rate limit"]
    #[tokio::test]
    async fn test_complete() {
        let api_key = env::var("OPENAI_API_KEY").unwrap();
        let adapter = OpenAIAdapter::new(api_key, "gpt-4o-mini");
        let messages = vec![Message {
            role: Role::User,
            content: "Hello".to_string(),
        }];
        let response = adapter.complete(&messages).await.unwrap();
        dbg!(&response);
        assert!(response.len() > 0);
    }
}
