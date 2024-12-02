# Data Load Service (Rust)

## Features
- Processes millions of CSV and Parquet files in chunks.
- Inserts data into a Postgres database.
- Uses Prometheus for monitoring.
- Reads secrets from AWS Parameter Store.

### Outline
```bash
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── database.rs
│   ├── file_processor.rs
│   ├── metrics.rs
├── tests/
│   ├── config_test.rs
│   ├── database_test.rs
│   ├── file_processor_test.rs
├── Dockerfile
├── docker-compose.yml
├── Prometheus/
│   ├── prometheus.yml
├── Cargo.toml
├── README.md
```

## How to Run
1. Build and run the Docker container:
   ```bash
   docker-compose up --build
   ```

2. Pass input directory
   ```bash
   docker run rust-data-ingest /data/input_dir
    ```
## Configs
Set the following AWS Parameter Store keys:
* DB_URL: Postgres connection string.
* CHUNK_SIZE: Number of records to process per chunk.

## Testing
    ```bash
    cargo test
    ```
