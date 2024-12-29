//! This module contains the `StrataFetcher` struct, which is responsible for
//! fetching checkpoint data from the Strata API.

use anyhow::{Context, Result};
use entity::checkpoint::RpcCheckpointInfo;
use reqwest::Client;
use serde_json::{json, Value}; // Provides better error handling

pub struct StrataFetcher {
    client: Client,
    endpoint: String, // Fullnode base URL
}

impl StrataFetcher {
    /// Creates a new StrataFetcher instance.
    ///
    /// # Parameters
    /// * `endpoint` - Base URL of the Strata fullnode
    ///
    /// # Examples
    /// ```
    /// let fetcher = StrataFetcher::new("http://fullnode.example.com".to_string());
    /// ```
    pub fn new(endpoint: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
        }
    }

    /// Fetches the last checkpoint index from the fullnode
    pub async fn get_last_checkpoint_index(&self) -> Result<i64> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": "strata_getLatestCheckpointIndex",
            "params": [],
            "id": 1
        });

        let response: Value = self
            .client
            .post(&self.endpoint)
            .json(&payload)
            .send()
            .await
            .context("Failed to send request")?
            .error_for_status()
            .context("Request returned an error status")?
            .json()
            .await
            .context("Failed to parse JSON response")?;

        // Extract the last checkpoint index from the response
        response
            .get("result")
            .and_then(|value| value.as_i64())
            .ok_or_else(|| anyhow::anyhow!("Failed to fetch last checkpoint index"))
    }

    /// Fetches checkpoint information from the Strata fullnode.
    ///
    /// Makes a JSON-RPC call to `strata_getCheckpointInfo` endpoint
    /// and deserializes the response into a checkpoint object.
    ///
    /// # Parameters
    /// * `idx` - Checkpoint index to fetch
    ///
    /// # Returns
    /// * `Result<RpcCheckpointInfo>` - Checkpoint data if successful
    ///
    /// # Errors
    /// * Network request failures
    /// * Invalid response format
    /// * Missing checkpoint data
    /// * Deserialization errors
    ///
    /// # Examples
    /// ```
    /// let checkpoint = fetcher.fetch_checkpoint(1234).await?;
    /// println!("Fetched checkpoint: {:?}", checkpoint);
    /// ```
    pub async fn fetch_checkpoint(&self, idx: i64) -> Result<RpcCheckpointInfo> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": "strata_getCheckpointInfo",
            "params": [idx],
            "id": 1
        });

        let response: Value = self
            .client
            .post(&self.endpoint)
            .json(&payload)
            .send()
            .await
            .context("Failed to send request")? // Attach context to the error
            .error_for_status()
            .context("Request returned an error status")?
            .json()
            .await
            .context("Failed to parse JSON response")?;

        // Handle `null` result explicitly
        match response.get("result") {
            Some(Value::Null) | None => {
                anyhow::bail!("No data exists for checkpoint ID: {}", idx);
            }
            Some(result) => {
                let checkpoint: RpcCheckpointInfo = serde_json::from_value(result.clone())
                    .context("Failed to deserialize checkpoint data")?;
                Ok(checkpoint)
            }
        }
    }
}
