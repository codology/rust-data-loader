#[cfg(test)]
mod tests {
    use rust_data_ingest::file_processor::process_files;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_file_processing_integration() {
        let db_url = "postgres://user:password@localhost/test_db";
        let db_pool = PgPool::connect(db_url).await.unwrap();

        let test_dir = "./test_data";
        tokio::fs::create_dir(test_dir).await.unwrap();

        let test_file = format!("{}/test.csv", test_dir);
        tokio::fs::write(&test_file, "col1,col2\nval1,val2").await.unwrap();

        let result = process_files(test_dir, &db_pool, 2).await;
        assert!(result.is_ok());

        tokio::fs::remove_file(&test_file).await.unwrap();
        tokio::fs::remove_dir(test_dir).await.unwrap();
    }
}
