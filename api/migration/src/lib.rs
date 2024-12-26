pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20241224_144707_seed;
mod m20241226_042219_seed1;
mod m20241226_100451_create_blocks_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20241224_144707_seed::Migration),
            Box::new(m20241226_042219_seed1::Migration),
            Box::new(m20241226_100451_create_blocks_table::Migration),
        ]
    }
}
