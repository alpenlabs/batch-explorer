use crate::models::RpcCheckpointInfo;
use reqwest::Client;
use serde_json::{json, Value};
use anyhow::{Result, Context}; // Provides better error handling

pub struct StrataFetcher {
    client: Client,
    endpoint: String, // Fullnode base URL
}

impl StrataFetcher {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
        }
    }


    pub async fn fetch_checkpoint(&self, idx: u64) -> Result<RpcCheckpointInfo> {
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
            _ => anyhow::bail!("Invalid response format"),
        }
    }
}