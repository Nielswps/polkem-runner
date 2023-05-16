use crate::substrate_utils::node_connection::NodeConnection;
use crate::prelude::*;
use crate::substrate_utils::node_driver;

pub mod chain_apis;
pub mod substrate_utils;
pub mod error;
pub mod prelude;

/// Log finalized block hashes using the logging agent. Returns any encountered errors
///
/// # Arguments
/// * `connection`: NodeConnection to the running substrate node
///
/// returns: Result<(), Box<dyn Error>>
pub async fn log_finalised_blocks(connection: &NodeConnection) -> std::result::Result<(), Box<dyn std::error::Error>> {
    async fn handle_block(block: SubstrateBlock) -> Result<()> {
        let block_hash = block.hash();

        let log_client = reqwest::Client::new();
        let res = log_client.post("logger-service/")
            .body(format!("Block {} was observed as finalised", block_hash))
            .send()
            .await;

        match res {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::Log(format!("POST request gave: {:?}", e)))
        }
    }

    node_driver::subscribe_to_finalized_blocks(connection, |block| handle_block(block)).await.expect("Should be able to subscribe to blocks");

    Ok(())
}

/// Print finalized block data to standard out. Returns any encountered errors
///
/// # Arguments
/// * `connection`: NodeConnection to the running substrate node
///
/// returns: Result<(), Box<dyn Error>>
pub async fn print_finalized_blocks(connection: &NodeConnection) -> std::result::Result<(), Box<dyn std::error::Error>> {
    async fn handle_block(block: SubstrateBlock) -> Result<()> {
        let block_number = block.header().number;
        let block_hash = block.hash();

        println!("Block #{block_number}:");
        println!("  Hash: {block_hash}");
        println!("  Extrinsics:");

        let body = match block.body().await {
            Ok(block_body) => block_body,
            Err(e) => return Err(Error::Substrate(format!("Encountered error while trying to get block body: {:?}", e)))
        };
        for ext in body.extrinsics() {
            let idx = ext.index();
            let events = match ext.events().await {
                Ok(extrinsic_events) => extrinsic_events,
                Err(e) => return Err(Error::Substrate(format!("Encountered error while trying to get extrinsic events: {:?}", e)))
            };

            let bytes_hex = format!("0x{}", hex::encode(ext.bytes()));

            println!("    Extrinsic #{idx}:");
            println!("      Bytes: {bytes_hex}");
            println!("      Events:");

            for evt in events.iter() {
                let evt = match evt {
                    Ok(details) => details,
                    Err(e) => return Err(Error::Substrate(format!("Encountered error while trying to get event details: {:?}", e)))
                };

                let pallet_name = evt.pallet_name();
                let event_name = evt.variant_name();

                println!("        {pallet_name}_{event_name}");
            }
        };

        Ok(())
    }

    node_driver::subscribe_to_finalized_blocks(connection, |block| handle_block(block)).await.expect("Should be able to subscribe to blocks");

    Ok(())
}