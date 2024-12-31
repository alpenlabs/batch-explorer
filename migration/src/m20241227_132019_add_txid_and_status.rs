use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20221201_000002_add_batch_txid_status"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Checkpoints::Table)
                    .add_column(
                        ColumnDef::new(Checkpoints::BatchTxid)
                            .string()
                            .not_null()
                            .default("-"),
                    )
                    .add_column(
                        ColumnDef::new(Checkpoints::Status)
                            .string()
                            .not_null()
                            .default("-"),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Checkpoints::Table)
                    .drop_column(Checkpoints::BatchTxid)
                    .drop_column(Checkpoints::Status)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(Iden)]
#[allow(dead_code)]
enum Checkpoints {
    Table,
    Idx,
    L1Start,
    L1End,
    L2Start,
    L2End,
    L2BlockId,
    BatchTxid,
    Status,
}