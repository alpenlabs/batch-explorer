use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::{json, Value};
use tracing::error;
/// `StrataFetcher` struct for fetching checkpoint and block data
pub struct StrataFetcher {
    client: Client,
    endpoint: String, // Fullnode base URL
}

impl StrataFetcher {
    /// Creates a new `StrataFetcher` instance.
    pub fn new(endpoint: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
        }
    }

    /// Fetches the latest index (checkpoint or block) based on the method name.
    ///
    /// # Parameters
    /// * `method` - JSON-RPC method name (e.g., `strata_getLatestCheckpointIndex`)
    ///
    /// # Returns
    /// * `Result<u64>` - Latest index if successful
    pub async fn get_latest_index(&self, method: &str) -> Result<u64> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": [],
            "id": 1
        });
    
        let response = self
            .client
            .post(&self.endpoint)
            .json(&payload)
            .send()
            .await
            .context("Failed to send request")?;
    
        let status = response.status();
        let text = response.text().await.context("Failed to read response body")?;
    
        if !status.is_success() {
            error!(
                "Request to {} failed with status {}: {}",
                self.endpoint, status, text
            );
            return Err(anyhow::anyhow!(
                "Request returned an error status: {} - {}",
                status,
                text
            ));
        }
    
        let json_response: Value = serde_json::from_str(&text)
            .context("Failed to parse JSON response")?;
    
        json_response["result"]
            .as_u64()
            .ok_or_else(|| anyhow::anyhow!("Invalid response format: {}", json_response))
    }

    /// Fetches detailed information for a given index (checkpoint or block).
    ///
    /// # Parameters
    /// * `method` - JSON-RPC method name (e.g., `strata_getCheckpointInfo`)
    /// * `idx` - Index to fetch
    ///
    /// # Returns
    /// * `Result<T>` - Fetched data deserialized into the generic type `T`
    pub async fn fetch_data<T>(&self, method: &str, idx: u64) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": [idx],
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
        
        
        match response.get("result") {
            Some(Value::Null) | None => {
                anyhow::bail!("No data exists for index ID: {}", idx);
            }
            Some(result) => {
                // let data: T = serde_json::from_value(result.clone())
                //     .context("Failed to deserialize response data")?;
                tracing::debug!("Raw response for idx {}: {:?}", idx, result);
                let data  = serde_json::from_value(result.clone())
                    .context("Failed to deserialize response data")?;
                Ok(data)
            }
        }
    }
}