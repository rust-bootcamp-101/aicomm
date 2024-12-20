use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use swiftide::{
    integrations::openai::OpenAI,
    query::{
        answers,
        query_transformers::{Embed, GenerateSubquestions},
        response_transformers::Summary,
        Pipeline,
    },
};
use swiftide_pgvector::PgVector;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

const VECTOR_SIZE: usize = 1024;

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    // let client = Ollama::default()
    //     // ollama embed model: https://ollama.com/blog/embedding-models
    //     .with_default_embed_model("mxbai-embed-large")
    //     .with_default_prompt_model("llama3.2")
    //     .to_owned();

    let client = OpenAI::builder()
        .default_embed_model("text-embedding-3-small")
        .default_prompt_model("gpt-4o-mini")
        .build()?;
    let pool = PgPoolOptions::new()
        .connect("postgres://postgres:postgres@localhost:5432/swiftide_rag")
        .await?;
    let store = PgVector::try_new(pool, VECTOR_SIZE).await?;
    let pipeline = Pipeline::default()
        .then_transform_query(GenerateSubquestions::from_client(client.clone()))
        .then_transform_query(Embed::from_client(client.clone()))
        .then_retrieve(store)
        .then_transform_response(Summary::from_client(client.clone()))
        .then_answer(answers::Simple::from_client(client));

    let result = pipeline
        .query("这个代码在做什么事情？请用中文简单回答")
        .await?;

    println!("{}", result.answer());
    Ok(())
}
