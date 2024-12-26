use sea_orm_migration::prelude::*;
use rand::Rng;
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Delete existing seeded values before reseeding
        clear_seeded_checkpoints(manager).await?;
        // Seed the checkpoints table with new random data
        seed_checkpoints(manager).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Delete all the seed data
        clear_seeded_checkpoints(manager).await?;
        Ok(())
    }
}

// Function to clear all seeded checkpoints
async fn clear_seeded_checkpoints(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    manager
        .get_connection()
        .execute_unprepared("DELETE FROM checkpoints WHERE idx < 10")
        .await?;
    Ok(())
}

// Function to seed the checkpoints table with random data
async fn seed_checkpoints(manager: &SchemaManager<'_>) -> Result<(), DbErr> {
    for idx in 0..=9 {
        let l1_start: i64 = rand::thread_rng().gen_range(1..1000);
        let l1_end: i64 = l1_start + rand::thread_rng().gen_range(1..100);
        let l2_start: i64 = rand::thread_rng().gen_range(1..1000);
        let l2_end: i64 = l2_start + rand::thread_rng().gen_range(1..100);
        let l2_block_id: String = Uuid::new_v4().to_string();

        let query = Query::insert()
            .into_table(Checkpoints::Table)
            .columns([
                Checkpoints::Idx,
                Checkpoints::L1Start,
                Checkpoints::L1End,
                Checkpoints::L2Start,
                Checkpoints::L2End,
                Checkpoints::L2BlockId,
            ])
            .values_panic([
                idx.into(),
                l1_start.into(),
                l1_end.into(),
                l2_start.into(),
                l2_end.into(),
                l2_block_id.into(),
            ])
            .to_owned();

        manager.exec_stmt(query).await?;
    }

    Ok(())
}

#[derive(Iden)]
enum Checkpoints {
    Table,
    Idx,
    L1Start,
    L1End,
    L2Start,
    L2End,
    L2BlockId,
}