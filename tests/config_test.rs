#[cfg(test)]
mod tests {
    use rust_data_ingest::config::load_config;

    #[tokio::test]
    async fn test_load_config_integration() {
        let result = load_config().await;
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(!config.db_url.is_empty());
        assert!(config.chunk_size > 0);
    }
}
