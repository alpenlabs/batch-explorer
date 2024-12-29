pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241226_100451_create_blocks_table;
mod m20241227_132019_add_txid_and_status;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241226_100451_create_blocks_table::Migration),
            Box::new(m20241227_132019_add_txid_and_status::Migration),
        ]
    }
}
