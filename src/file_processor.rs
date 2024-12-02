use sqlx::PgPool;
use std::path::Path;
use tokio::fs;

pub async fn process_files(input_dir: &str, db_pool: &PgPool, chunk_size: usize) -> Result<(), Box<dyn std::error::Error>> {
    let paths = fs::read_dir(input_dir).await?;
    let mut buffer = Vec::new();

    for path in paths {
        let path = path?.path();
        if path.extension().map(|ext| ext == "csv" || ext == "parquet").unwrap_or(false) {
            let records = read_file(&path).await?;
            buffer.extend(records);

            if buffer.len() >= chunk_size {
                insert_into_db(&buffer, db_pool).await?;
                buffer.clear();
            }
        }
    }

    if !buffer.is_empty() {
        insert_into_db(&buffer, db_pool).await?;
    }

    Ok(())
}

async fn read_file(path: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    // TODO CSV/Parquet reads
    Ok(vec!["data".to_string()])
}

async fn insert_into_db(data: &[String], db_pool: &PgPool) -> Result<(), sqlx::Error> {
    // TODO bulk insert
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[tokio::test]
    async fn test_process_files() {
        let test_dir = "./test_data";
        tokio::fs::create_dir(test_dir).await.unwrap();
        let test_file = format!("{}/test.csv", test_dir);

        // Create a temporary test file
        let mut file = File::create(&test_file).unwrap();
        writeln!(file, "col1,col2\nval1,val2").unwrap();

        let db_pool = PgPool::connect("postgres://user:password@localhost/test_db")
            .await
            .unwrap();

        let result = process_files(test_dir, &db_pool, 10).await;

        assert!(result.is_ok());

        // Clean up
        tokio::fs::remove_file(test_file).await.unwrap();
        tokio::fs::remove_dir(test_dir).await.unwrap();
    }
}
