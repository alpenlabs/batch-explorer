use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
/// Represents an L2 Block ID.
pub type L2BlockId = String;

/// Represents the checkpoint information returned by the RPC.
/// Name for this struct comes from the Strata RPC endpoint.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RpcCheckpointInfo {
    /// The index of the checkpoint
    pub idx: u64,
    /// The L1 height range that the checkpoint covers (start, end)
    pub l1_range: (u64, u64),
    /// The L2 height range that the checkpoint covers (start, end)
    pub l2_range: (u64, u64),
    /// The L2 block ID that this checkpoint covers
    pub l2_blockid: L2BlockId,

    // These are optional as current version of fullnode does not return them
    pub batch_txid: Option<String>,
    pub status: Option<String>,
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "checkpoints")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub idx: i64,
    pub l1_start: i64,
    pub l1_end: i64,
    pub l2_start: i64,
    pub l2_end: i64,
    pub l2_block_id: L2BlockId,
    pub batch_txid: String,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
impl From<RpcCheckpointInfo> for ActiveModel {
    fn from(info: RpcCheckpointInfo) -> Self {
        Self {
            idx: Set(info.idx as i64),
            l1_start: Set(info.l1_range.0 as i64),
            l1_end: Set(info.l1_range.1 as i64),
            l2_start: Set(info.l2_range.0 as i64),
            l2_end: Set(info.l2_range.1 as i64),
            l2_block_id: Set(info.l2_blockid),
            batch_txid: Set(info.batch_txid.unwrap_or_else(|| "-".to_string())), // Default value
            status: Set(info.status.unwrap_or_else(|| "-".to_string())),         // Default value
        }
    }
}

impl From<Model> for RpcCheckpointInfo {
    fn from(model: Model) -> Self {
        Self {
            idx: model.idx as u64,
            l1_range: (model.l1_start as u64, model.l1_end as u64),
            l2_range: (model.l2_start as u64, model.l2_end as u64),
            l2_blockid: model.l2_block_id,
            batch_txid: Some(model.batch_txid.clone()),
            status: Some(model.status.clone()),
        }
    }
}
