mod utils;

pub mod middlewares;

use thiserror::Error;
pub use utils::*;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub ws_id: i64, // workspace_id
    #[sqlx(default)]
    pub ws_name: String,
    pub fullname: String,
    pub email: String,
    #[sqlx(default)]
    #[serde(skip)]
    pub password_hash: Option<String>,
    #[sqlx(default)]
    pub is_bot: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, ToSchema, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workspace {
    pub id: i64,
    pub name: String,
    #[serde(alias = "ownerId")]
    pub owner_id: i64,
    #[serde(alias = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, ToSchema, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatUser {
    pub id: i64,
    pub fullname: String,
    pub email: String,
}

#[derive(Debug, ToSchema, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chat {
    pub id: i64,
    #[serde(alias = "wsId")]
    pub ws_id: i64,
    pub name: Option<String>,
    pub r#type: ChatType,
    pub members: Vec<i64>,

    pub agents: Vec<i64>,

    #[serde(alias = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(
    Debug, Default, ToSchema, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, sqlx::Type,
)]
#[sqlx(type_name = "chat_type", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase"))]
pub enum ChatType {
    #[default]
    #[serde(alias = "single", alias = "Single")]
    Single,
    #[serde(alias = "group", alias = "Group")]
    Group,
    #[serde(alias = "private_channel", alias = "privateChannel")]
    PrivateChannel,
    #[serde(alias = "public_channel", alias = "publicChannel")]
    PublicChannel,
}

#[derive(
    Debug, Default, ToSchema, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, sqlx::Type,
)]
#[sqlx(type_name = "agent_type", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase"))]
pub enum AgentType {
    #[default]
    #[serde(alias = "proxy", alias = "Proxy")]
    Proxy,
    #[serde(alias = "reply", alias = "Reply")]
    Reply,
    #[serde(alias = "tap", alias = "Tap")]
    Tap,
}

#[derive(
    Debug, Default, ToSchema, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, sqlx::Type,
)]
#[sqlx(type_name = "adapter_type", rename_all = "snake_case")]
#[serde(rename_all(serialize = "camelCase"))]
pub enum AdapterType {
    #[default]
    #[serde(alias = "ollama", alias = "Ollama")]
    Ollama,
    #[sqlx(rename = "openai")]
    #[serde(alias = "openai", alias = "OpenAI")]
    OpenAI,
}

#[derive(Debug, Clone, FromRow, ToSchema, Serialize, Deserialize, PartialEq)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct ChatAgent {
    pub id: i64,
    #[serde(alias = "chatId")]
    pub chat_id: i64,
    pub name: String,
    pub r#type: AgentType,
    pub adapter: AdapterType,
    pub model: String,
    pub prompt: String,
    pub args: sqlx::types::Json<serde_json::Value>, // TODO: change to custom type
    #[serde(alias = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(alias = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, ToSchema, Clone, PartialEq, Eq, FromRow, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Message {
    pub id: i64,
    #[serde(alias = "chatId")]
    pub chat_id: i64,
    #[serde(alias = "senderId")]
    pub sender_id: i64,
    pub modified_content: Option<String>,
    pub content: String,
    pub files: Vec<String>,
    #[serde(alias = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[allow(async_fn_in_trait)]
pub trait Agent {
    async fn process(&self, msg: &str, ctx: &AgentContext) -> Result<AgentDecision, AgentError>;
}

#[derive(Debug, Clone)]
pub enum AgentDecision {
    Modify(String),
    Reply(String),
    Delete,
    None,
}

#[derive(Debug, Default, Clone)]
pub struct AgentContext {}

#[derive(Error, Debug)]
pub enum AgentError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("{0}")]
    AnyError(#[from] anyhow::Error),
}

impl User {
    pub fn new(id: i64, fullname: &str, email: &str) -> Self {
        Self {
            id,
            ws_id: 0,
            ws_name: "".to_string(),
            fullname: fullname.to_string(),
            email: email.to_string(),
            password_hash: None,
            is_bot: false,
            created_at: chrono::Utc::now(),
        }
    }
}
