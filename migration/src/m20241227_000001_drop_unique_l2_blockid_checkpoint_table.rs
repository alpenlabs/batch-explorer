use sea_orm_migration::prelude::*;
use sea_orm::{DbBackend, Statement};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Use raw SQL to drop the UNIQUE constraint
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                r#"ALTER TABLE "checkpoints" DROP CONSTRAINT "checkpoints_l2_block_id_key""#.to_owned(),
            ))
            .await
            .map(|_| ())?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute(Statement::from_string(
                DbBackend::Postgres,
                r#"ALTER TABLE "checkpoints" ADD CONSTRAINT "checkpoints_l2_block_id_key" UNIQUE ("l2_block_id")"#.to_owned(),
            ))
            .await
            .map(|_| ())?;

        Ok(())
    }
}