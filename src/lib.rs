use std::string::String;
use crate::substrate_utils::node_connection::NodeConnection;
use crate::prelude::*;
use crate::substrate_utils::node_driver;

pub mod chain_apis;
pub mod substrate_utils;
pub mod error;
pub mod prelude;

/// Log finalised block hashes using the logging agent. Returns any encountered errors
///
/// # Arguments
/// * `connection`: NodeConnection to the running substrate node
///
/// returns: Result<(), Box<dyn Error>>
pub async fn log_finalised_blocks(connection: &NodeConnection, log_endpoint: String) -> std::result::Result<(), Box<dyn std::error::Error>> {
    async fn log_block(block: SubstrateBlock, log_url: &String) -> Result<()> {
        let block_hash = block.hash();

        let log_client = reqwest::Client::new();
        let res = log_client
            .post(log_url)
            .body(format!("Block {} was observed as finalised", block_hash))
            .send()
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Log(format!("POST request gave: {:?}", e)))
        }
    }

    node_driver::subscribe_to_finalised_blocks(connection, |block| log_block(block, &log_endpoint)).await.expect("Should be able to subscribe to blocks");

    Ok(())
}

/// Log proposed and finalised block hashes using the logging agent. Returns any encountered errors
///
/// # Arguments
/// * `connection`: NodeConnection to the running substrate node
///
/// returns: Result<(), Box<dyn Error>>
pub async fn  log_proposed_and_finalised_blocks(connection_1: NodeConnection, connection_2: NodeConnection, log_endpoint: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    async fn log_block(block: SubstrateBlock, log_url: &String, block_state: &str) -> Result<()> {
        let block_hash = block.hash();

        let log_client = reqwest::Client::new();
        let res = log_client
            .post(log_url)
            .body(format!("Block {} was observed as {}", block_hash, block_state))
            .send()
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Log(format!("POST request gave: {:?}", e)))
        }
    }

    // Start subscriber threads
    let log = String::from(log_endpoint);
    let handle_1 = tokio::spawn(async move {
        node_driver::subscribe_to_best_blocks(&connection_1, |block| log_block(block, &log, "proposed")).await.expect("Should be able to subscribe to best blocks");
    });

    let log = String::from(log_endpoint);
    let handle_2 = tokio::spawn(async move {
        node_driver::subscribe_to_finalised_blocks(&connection_2, |block| log_block(block, &log, "finalised")).await.expect("Should be able to subscribe to finalised blocks");
    });

    // Trigger subscriber tasks
    handle_1.await?;
    handle_2.await?;

    Ok(())
}