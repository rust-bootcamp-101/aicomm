use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use swiftide::{
    indexing::{
        loaders::FileLoader,
        transformers::{ChunkCode, Embed, MetadataQACode},
        Pipeline,
    },
    integrations::openai::OpenAI,
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
    Pipeline::from_loader(FileLoader::new(".").with_extensions(&["rs"]))
        .then(MetadataQACode::new(client.clone()))
        .then_chunk(ChunkCode::try_for_language_and_chunk_size(
            "rust",
            10..2048,
        )?)
        .then_in_batch(Embed::new(client).with_batch_size(10))
        .then_store_with(store)
        .run()
        .await?;
    Ok(())
}
