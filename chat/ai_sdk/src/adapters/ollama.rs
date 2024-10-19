use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{AiService, Message};

pub struct OllamaAdapter {
    host: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize)]
pub struct OllamaChatCompletionRequest {
    pub model: String,
    pub messages: Vec<OllamaMessage>,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct OllamaChatCompletionResponse {
    pub model: String,
    pub created_at: String,
    pub message: OllamaMessage,
    pub done: bool,
    pub total_duration: u64,
    pub load_duration: u64,
    pub prompt_eval_count: u32,
    pub prompt_eval_duration: u64,
    pub eval_count: u32,
    pub eval_duration: u64,
}

impl OllamaAdapter {
    pub fn new(host: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            model: model.into(),
            client: Client::new(),
        }
    }

    pub fn new_local(model: impl Into<String>) -> Self {
        Self {
            host: "http://localhost:11434".into(),
            model: model.into(),
            client: Client::new(),
        }
    }
}

impl Default for OllamaAdapter {
    fn default() -> Self {
        Self::new_local("llama3.2")
    }
}

impl AiService for OllamaAdapter {
    async fn complete(&self, messages: &[Message]) -> anyhow::Result<String> {
        let request = OllamaChatCompletionRequest {
            model: self.model.clone(),
            messages: messages.iter().map(|m| m.into()).collect(),
            stream: false,
        };
        let url = format!("{}/api/chat", self.host);
        let response = self.client.post(url).json(&request).send().await?;
        let data: OllamaChatCompletionResponse = response.json().await?;
        let content = data.message.content;
        Ok(content)
    }
}

impl From<Message> for OllamaMessage {
    fn from(m: Message) -> Self {
        OllamaMessage {
            role: m.role.to_string(),
            content: m.content,
        }
    }
}

impl From<&Message> for OllamaMessage {
    fn from(m: &Message) -> Self {
        OllamaMessage {
            role: m.role.to_string(),
            content: m.content.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Role;

    #[ignore]
    #[tokio::test]
    async fn ollama_complete_should_work() {
        let adapter = OllamaAdapter::new_local("llama3.2");
        let messages = vec![Message {
            role: Role::User,
            content: "Hello".to_string(),
        }];
        let content = adapter.complete(&messages).await.unwrap();
        println!("content: {}", content);
    }
}
