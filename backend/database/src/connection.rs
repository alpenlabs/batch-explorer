use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Statement};
pub struct DatabaseWrapper {
    pub db: DatabaseConnection,
}

impl DatabaseWrapper {
    /// Create a new database wrapper with the given database URL
    pub async fn new(database_url: &str) -> Self {
        let db = Database::connect(database_url)
            .await
            .expect("Failed to connect to PostgreSQL {database_url}");
        Self { db }
    }
    pub async fn has_all_tables(&self) -> bool {
        let required_tables = ["blocks", "checkpoints"];

        for table in &required_tables {
            let stmt = Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Postgres,
                r#"SELECT COUNT(*) as count FROM information_schema.tables 
                   WHERE table_schema = 'public' AND table_name = $1"#,
                vec![(*table).into()],
            );

            let row = self.db.query_one(stmt).await;

            match row {
                Ok(Some(res)) => {
                    let count: i64 = res.try_get("", "count").unwrap_or(0);
                    if count == 0 {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}
