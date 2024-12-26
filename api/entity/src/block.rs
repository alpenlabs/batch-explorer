use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::{NotSet, Set};
use serde::{Deserialize, Serialize};

/// Represents the Block model for the database
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "blocks")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub block_hash: String, // Represents the block_id as a hash
    pub height: i64,         // Represents the block height
    pub checkpoint_idx: i64, // Foreign key to the checkpoint index
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

/// Implements conversion from `RpcBlockHeader` to `ActiveModel` for the `blocks` table
impl From<RpcBlockHeader> for ActiveModel {
    fn from(header: RpcBlockHeader) -> Self {
        Self {
            block_hash: Set(hex::encode(header.block_id)), // Convert block_id (u8 array) to hex string
            height: Set(header.block_idx as i64),
            checkpoint_idx: NotSet, // Leave unset, to be filled when associating with a checkpoint
        }
    }
}

// /// Implements conversion from `Model` (database row) back to `RpcBlockHeader`
// impl From<Model> for RpcBlockHeader {
//     fn from(model: Model) -> Self {
//         RpcBlockHeader {
//             block_idx: model.height as u64,
//             timestamp: 0, // Placeholder: Needs to be populated from additional data
//             block_id: hex::decode(&model.block_hash)
//                 .unwrap_or_else(|_| [0; 32].to_vec())
//                 .try_into()
//                 .unwrap_or([0; 32]), // Convert hex string to a u8 array
//             prev_block: [0; 32], // Placeholder
//             l1_segment_hash: [0; 32], // Placeholder
//             exec_segment_hash: [0; 32], // Placeholder
//             state_root: [0; 32], // Placeholder
//         }
//     }
// }
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcBlockHeader {
    /// The index of the block representing height.
    pub block_idx: u64,

    /// The timestamp of when the block was created in UNIX epoch format.
    pub timestamp: u64,

    /// hash of the block's contents.
    pub block_id: String,

    /// previous block
    pub prev_block: String,

    // L1 segment hash
    pub l1_segment_hash: String,

    /// Hash of the execution segment
    pub exec_segment_hash: String,

    /// The root hash of the state tree
    pub state_root: String,
}
/// Simplified representation of a block header with only fields of interest
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BlockInfo {
    /// The index of the block (height)
    pub block_idx: u64,

    /// The hash of the block's contents
    pub block_id: [u8; 32],
}
