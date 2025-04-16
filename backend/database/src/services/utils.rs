use crate::connection::DatabaseWrapper;
use sea_orm::Order;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub(crate) fn resolve_order(order: Option<&str>) -> Order {
    match order {
        Some("asc") => Order::Asc,
        Some("desc") => Order::Desc,
        _ => Order::Desc,
    }
}

pub async fn wait_until_migration(database: &Arc<DatabaseWrapper>) {
    // Wait until the migration is done
    loop {
        tracing::info!("Waiting for migration to complete...");
        if database.has_all_tables().await {
            break;
        }
        sleep(Duration::from_secs(5)).await;
    }
}
