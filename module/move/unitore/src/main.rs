//!
// use unitore::retriever::FeedClient;
// use unitore::feed_config::read_feed_config;
pub use unitore::executor;

#[ tokio::main ]
async fn main() -> Result< (), Box< dyn std::error::Error + Send + Sync > >
{
  executor::execute().await
}
