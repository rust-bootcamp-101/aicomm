use chat_core::{AgentType, ChatAgent};
use serde::{Deserialize, Serialize};
use tracing::info;
use utoipa::ToSchema;

use crate::{AppError, AppState};

#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct CreateAgent {
    pub name: String,
    pub r#type: AgentType,
    pub prompt: String,
    #[serde(default = "default_map")] // 默认值为 {}
    pub args: serde_json::Value,
}

#[derive(Debug, Clone, Default, ToSchema, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct UpdateAgent {
    pub id: u64,
    #[serde(default)]
    pub prompt: String,
    #[serde(default)]
    pub args: serde_json::Value,
}

impl AppState {
    /// Create a new agent in a chat
    pub async fn create_agent(
        &self,
        input: CreateAgent,
        chat_id: u64,
    ) -> Result<ChatAgent, AppError> {
        // check if agent exists
        if self.agent_name_exists(chat_id, &input.name).await? {
            info!("Agent {} already exists in chat {chat_id}", input.name);
            return Err(AppError::CreateAgentError(format!(
                "Agent {} already exists",
                input.name
            )));
        }

        let agent = sqlx::query_as(
            r#"
            INSERT INTO chat_agents (chat_id, name, type, prompt, args)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
        "#,
        )
        .bind(chat_id as i64)
        .bind(input.name)
        .bind(input.r#type)
        .bind(input.prompt)
        .bind(input.args)
        .fetch_one(&self.pool)
        .await?;
        Ok(agent)
    }

    /// List all agent in a chat
    pub async fn list_agents(&self, chat_id: u64) -> Result<Vec<ChatAgent>, AppError> {
        let agents = sqlx::query_as(
            r#"
            SELECT * FROM chat_agents WHERE chat_id = $1 ORDER BY id ASC
        "#,
        )
        .bind(chat_id as i64)
        .fetch_all(&self.pool)
        .await?;
        Ok(agents)
    }

    /// Update agent in a chat
    pub async fn update_agent(
        &self,
        input: UpdateAgent,
        chat_id: u64,
    ) -> Result<ChatAgent, AppError> {
        let agent_id = input.id;
        // check if agent exists
        if !self.agent_id_exists(chat_id, agent_id).await? {
            info!("Agent {agent_id} does not exists in chat {chat_id}");
            return Err(AppError::CreateAgentError(format!(
                "Agent {} does not exists",
                agent_id,
            )));
        }

        let prompt = input.prompt;
        let args = input.args;
        let agent = match (prompt.as_str(), &args) {
            ("", _) => {
                // no prompt
                sqlx::query_as(
                    r#"
                    UPDATE chat_agents
                        SET
                            args = $1,
                            updated_at = NOW()
                    WHERE chat_id = $2 AND id = $3
                    RETURNING *
                "#,
                )
                .bind(args)
                .bind(chat_id as i64)
                .bind(agent_id as i64)
                .fetch_one(&self.pool)
                .await?
            }
            (_, _) => {
                sqlx::query_as(
                    r#"
                    UPDATE chat_agents
                        SET
                            prompt = $1,
                            args = $2,
                            updated_at = NOW()
                    WHERE chat_id = $3 and id = $4
                    RETURNING *
                "#,
                )
                .bind(prompt)
                .bind(args)
                .bind(chat_id as i64)
                .bind(agent_id as i64)
                .fetch_one(&self.pool)
                .await?
            }
        };
        Ok(agent)
    }

    /// Check if an agent name exists in a chat
    pub async fn agent_name_exists(&self, chat_id: u64, name: &str) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar(
            r#"
        SELECT EXISTS (SELECT 1 FROM chat_agents WHERE chat_id = $1 AND name = $2) "#,
        )
        .bind(chat_id as i64)
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        Ok(exists)
    }

    /// Check if an agent id exists in a chat
    pub async fn agent_id_exists(&self, chat_id: u64, agent_id: u64) -> Result<bool, AppError> {
        let exists = sqlx::query_scalar(
            r#"
        SELECT EXISTS (SELECT 1 FROM chat_agents WHERE chat_id = $1 AND id = $2) "#,
        )
        .bind(chat_id as i64)
        .bind(agent_id as i64)
        .fetch_one(&self.pool)
        .await?;
        Ok(exists)
    }
}

impl CreateAgent {
    pub fn new(
        name: impl Into<String>,
        r#type: AgentType,
        prompt: impl Into<String>,
        args: impl Serialize,
    ) -> Self {
        Self {
            name: name.into(),
            r#type,
            prompt: prompt.into(),
            args: serde_json::to_value(args).unwrap(),
        }
    }
}

impl UpdateAgent {
    pub fn new(id: u64, prompt: impl Into<String>, args: impl Serialize) -> Self {
        Self {
            id,
            prompt: prompt.into(),
            args: serde_json::to_value(args).unwrap(),
        }
    }
}

fn default_map() -> serde_json::Value {
    serde_json::Value::Object(serde_json::Map::new())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[tokio::test]
    async fn create_agent_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let input = CreateAgent::new(
            "agent X1",
            AgentType::Proxy,
            "You are a helpful assistant",
            "{}",
        );
        let agent = state.create_agent(input, 1).await?;
        assert_eq!(agent.chat_id, 1);
        assert_eq!(agent.r#type, AgentType::Proxy);
        assert_eq!(agent.prompt, "You are a helpful assistant");
        assert_eq!(agent.args, sqlx::types::Json(serde_json::json!("{}")));
        Ok(())
    }

    /*

    -- insert agent to chat
    INSERT INTO chat_agents (chat_id, name, type, prompt, args)
        VALUES (1, 'translation', 'proxy', 'If language is Chinese, translate to English. if language is English, translate to Chinese. Please reply with the translated content directly. No explanation is needed. Here is the content: ', '{}');
         */
    #[tokio::test]
    async fn list_agent_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let agents = state.list_agents(1).await?;
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name, "translation");
        assert_eq!(agents[0].r#type, AgentType::Proxy);
        assert_eq!(agents[0].prompt, "If language is Chinese, translate to English. if language is English, translate to Chinese. Please reply with the translated content directly. No explanation is needed. Here is the content: ");
        assert_eq!(agents[0].args, sqlx::types::Json(serde_json::json!({})));
        Ok(())
    }

    #[tokio::test]
    async fn update_agent_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        // Create an agent
        let input = CreateAgent::new(
            "test agent",
            AgentType::Proxy,
            "You are a helpful assistant",
            "{}",
        );
        let agent = state.create_agent(input, 1).await?;
        assert_eq!(agent.chat_id, 1);
        assert_eq!(agent.r#type, AgentType::Proxy);
        assert_eq!(agent.prompt, "You are a helpful assistant");
        assert_eq!(agent.args, sqlx::types::Json(serde_json::json!("{}")));

        let input = UpdateAgent::new(
            agent.id as _,
            "changed prompt",
            serde_json::json!("{'a': 123}"),
        );
        let update_agent = state.update_agent(input, agent.chat_id as _).await?;
        assert_eq!(agent.id, update_agent.id);
        assert_eq!(agent.chat_id, update_agent.chat_id);
        assert_eq!(agent.r#type, update_agent.r#type);
        assert_eq!(update_agent.prompt, "changed prompt");
        assert_eq!(
            update_agent.args,
            sqlx::types::Json(serde_json::json!("{'a': 123}"))
        );
        Ok(())
    }
}
