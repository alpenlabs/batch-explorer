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
    pub async fn migrations_applied(&self) -> bool {
        let stmt = Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT COUNT(*) as count FROM seaql_migrations",
        );

        match self.db.query_one(stmt).await {
            Ok(Some(row)) => {
                let count: i64 = row.try_get("", "count").unwrap_or(0);
                count > 0
            }
            _ => false,
        }
    }
}
