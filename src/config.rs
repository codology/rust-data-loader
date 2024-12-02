use aws_sdk_ssm::Client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub db_url: String,
    pub chunk_size: usize,
}

pub async fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let client = Client::new(&aws_config::load_from_env().await);
    let db_url = client.get_parameter().name("DB_URL").with_decryption(true).send().await?.parameter.unwrap().value.unwrap();
    let chunk_size: usize = client.get_parameter().name("CHUNK_SIZE").with_decryption(true).send().await?.parameter.unwrap().value.unwrap().parse()?;

    Ok(Config { db_url, chunk_size })
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_ssm::Client;

    #[tokio::test]
    async fn test_load_config_success() {
        let mock_config = Config {
            db_url: "postgres://user:password@localhost/test_db".to_string(),
            chunk_size: 100,
        };

        // Simulate mocked environment
        let client = Client::from_env(); // Configure mock SSM client
        let result = load_config().await;

        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.db_url, mock_config.db_url);
        assert_eq!(config.chunk_size, mock_config.chunk_size);
    }
}