use crate::pgu64::PgU64;
use anyhow::Error;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::result::Result;
use std::str::FromStr;
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
    /// Info on txn where checkpoint is committed on chain
    pub commitment: Option<RpcCheckpointCommitmentInfo>,
    /// Confirmation status of checkpoint
    pub confirmation_status: Option<RpcCheckpointConfStatus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RpcCheckpointConfStatus {
    /// Pending to be posted on L1
    Pending,
    /// Confirmed on L1
    Confirmed,
    /// Finalized on L1
    Finalized,
}

impl FromStr for RpcCheckpointConfStatus {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(RpcCheckpointConfStatus::Pending),
            "confirmed" => Ok(RpcCheckpointConfStatus::Confirmed),
            "finalized" => Ok(RpcCheckpointConfStatus::Finalized),
            _ => Err(Error::msg(format!("Invalid status: {}", s))),
        }
    }
}

impl Display for RpcCheckpointConfStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let status_str = match self {
            RpcCheckpointConfStatus::Pending => "pending",
            RpcCheckpointConfStatus::Confirmed => "confirmed",
            RpcCheckpointConfStatus::Finalized => "finalized",
        };
        write!(f, "{}", status_str)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcCheckpointCommitmentInfo {
    /// block where checkpoint was posted
    pub blockhash: String,

    /// txid of txn for this checkpoint
    pub txid: String,

    /// wtxid of txn for this checkpoint
    pub wtxid: String,

    /// The height of the block where the checkpoint was posted.
    pub height: u64,

    /// The position of the checkpoint in the block.
    pub position: u32,
}

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, DeriveActiveModelBehavior, Serialize, Deserialize,
)]
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

impl From<RpcCheckpointInfo> for ActiveModel {
    fn from(info: RpcCheckpointInfo) -> Self {
        Self {
            idx: Set(PgU64(info.idx).to_i64()),
            l1_start: Set(PgU64(info.l1_range.0).to_i64()),
            l1_end: Set(PgU64(info.l1_range.1).to_i64()),
            l2_start: Set(PgU64(info.l2_range.0).to_i64()),
            l2_end: Set(PgU64(info.l2_range.1).to_i64()),
            l2_block_id: Set(info.l2_blockid),
            batch_txid: Set(info
                .commitment
                .as_ref()
                .map_or("-".to_string(), |c| c.txid.clone())), // Extracting `txid`
            status: Set(info
                .confirmation_status
                .as_ref()
                .map_or("-".to_string(), |s| format!("{:?}", s))), // Convert enum to string
        }
    }
}

impl From<Model> for RpcCheckpointInfo {
    fn from(model: Model) -> Self {
        Self {
            idx: PgU64::from_i64(model.idx).0,
            l1_range: (
                PgU64::from_i64(model.l1_start).0,
                PgU64::from_i64(model.l1_end).0,
            ),
            l2_range: (
                PgU64::from_i64(model.l2_start).0,
                PgU64::from_i64(model.l2_end).0,
            ),
            l2_blockid: model.l2_block_id,
            commitment: Some(RpcCheckpointCommitmentInfo {
                blockhash: String::new(),
                txid: model.batch_txid,
                wtxid: String::new(),
                height: 0,
                position: 0,
            }),
            confirmation_status: model.status.parse().ok(), // Convert status string to `RpcCheckpointConfStatus`
        }
    }
}
