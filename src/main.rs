mod config;
mod database;
mod file_processor;
mod metrics;

use std::env;
use crate::file_processor::process_files;

#[tokio::main]
async fn main() {
    // Initialize metrics and Prometheus server
    metrics::init_metrics();

    // Load configurations from AWS Parameter Store
    let config = config::load_config().await.expect("Failed to load configuration");

    // Connect to Postgres database
    let db_pool = database::connect(&config.db_url).await.expect("Database connection failed");

    // Process files in chunks
    let input_dir = env::args().nth(1).expect("Please provide an input directory as an argument");
    process_files(&input_dir, &db_pool, config.chunk_size).await.expect("File processing failed");
}
