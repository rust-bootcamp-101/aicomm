use std::collections::HashSet;

use anyhow::Result;
use chat_core::Message;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgListener, PgPoolOptions},
    PgPool,
};
use swiftide::{
    integrations::ollama::Ollama,
    query::{
        answers::Simple,
        query_transformers::{Embed, GenerateSubquestions},
        response_transformers::Summary,
        Pipeline,
    },
    traits::{EmbeddingModel, SimplePrompt},
};
use swiftide_pgvector::PgVectorBuilder;
use tracing::{info, warn};

use crate::{AppConfig, VECTOR_SIZE};

#[derive(Debug)]
struct Notification {
    // users being impacted, so we should send the notification to them
    bot_id: i64,
    event: Message,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatMessageCreated {
    members: HashSet<i64>,
    message: Message,
}

pub async fn setup_pg_listener(config: &AppConfig) -> Result<()> {
    let db_url = &config.server.db_url;
    let mut listener = PgListener::connect(db_url).await?;
    listener.listen("chat_message_created").await?;
    info!("Listening to chat_message_created");
    let client = Ollama::default()
        // ollama embed model: https://ollama.com/blog/embedding-models
        .with_default_embed_model("mxbai-embed-large")
        .with_default_prompt_model("llama3.2")
        .to_owned();
    let pool = PgPoolOptions::new().connect(db_url).await?;
    let bots = get_bots(&pool).await?;
    let mut stream = listener.into_stream();
    while let Some(Ok(notif)) = stream.next().await {
        if let Some(notification) = Notification::load(notif.channel(), notif.payload(), &bots) {
            info!("Received notification: {:?}", notification);
            let pool = pool.clone();
            let client = client.clone();
            tokio::spawn(async move {
                let bot_id = notification.bot_id;
                let user_id = notification.event.sender_id;
                if let Err(e) = notification.process(&pool, client.clone(), client).await {
                    warn!(
                        "Bot {} failed to send notification to user {}: {}",
                        bot_id, user_id, e
                    );
                }
            });
        }
    }

    Ok(())
}

impl Notification {
    pub(crate) fn load(r#type: &str, payload: &str, bots: &HashSet<i64>) -> Option<Self> {
        match r#type {
            "chat_message_created" => {
                let payload: ChatMessageCreated = serde_json::from_str(payload).ok()?;
                let mut members = payload.members;
                members.remove(&payload.message.sender_id);

                // only process if it's a direct message
                if members.len() == 1 {
                    let bot_id = members.iter().next().unwrap();
                    if bots.contains(bot_id) {
                        return Some(Self {
                            bot_id: *bot_id,
                            event: payload.message,
                        });
                    }
                }
                None
            }
            _ => None,
        }
    }

    pub(crate) async fn process(
        self,
        pool: &PgPool,
        client: impl SimplePrompt + Clone + 'static,
        embed_client: impl EmbeddingModel + Clone + 'static,
    ) -> Result<()> {
        let store = PgVectorBuilder::default()
            .pool(pool.clone())
            .vector_size(VECTOR_SIZE as _)
            .build()?;
        let pipeline = Pipeline::default()
            .then_transform_query(GenerateSubquestions::from_client(client.clone()))
            .then_transform_query(Embed::from_client(embed_client))
            .then_retrieve(store)
            .then_transform_response(Summary::from_client(client.clone()))
            .then_answer(Simple::from_client(client));

        info!("Processing notification: {:?}", self.event.id);
        let ret = pipeline.query(self.event.content).await?;
        let answer = ret.answer();
        info!("Got answer. Writing to db...");
        let _: (i64,) = sqlx::query_as(
            r#"
                        INSERT INTO messages (chat_id, sender_id, content)
                        VALUES ($1, $2, $3)
                        RETURNING id
                    "#,
        )
        .bind(self.event.chat_id)
        .bind(self.bot_id)
        .bind(answer)
        .fetch_one(pool)
        .await?;

        Ok(())
    }
}

async fn get_bots(pool: &PgPool) -> Result<HashSet<i64>> {
    let bots: Vec<(i64,)> = sqlx::query_as(
        r#"
        SELECT id FROM users WHERE is_bot = TRUE
    "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(bots.into_iter().map(|b| b.0).collect())
}
