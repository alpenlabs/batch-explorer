use serde::{Deserialize, Serialize};

/// Represents an L2 Block ID.
pub type L2BlockId = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Checkpoint {
    pub idx: u64,
    pub l1_height: u64,
    pub l2_height: u64,
    pub l2_blockid: String,
}

/// Represents the checkpoint information returned by the RPC.
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
}
