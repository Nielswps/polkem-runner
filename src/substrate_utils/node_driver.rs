use std::future::Future;
use std::str::FromStr;
use codec::Output;
use futures::StreamExt;
use subxt::{OnlineClient, SubstrateConfig};
use subxt::storage::Storage;
use subxt::utils::H256;
use crate::chain_apis;
use crate::chain_apis::energychain_api::api::runtime_types::pallet_energychain::pallet::EnergyOffer;
use crate::prelude::*;
use crate::substrate_utils::node_connection::NodeConnection;
use super::super::prelude::*;


/// Subscribe to finalized blocks at the NodeConnection
///
/// # Arguments
///
/// * `connection`: NodeConnection to the running substrate node
/// * `block_consumer`: function for consuming fetched blocks
///
/// returns: ()
pub async fn subscribe_to_finalized_blocks<T>(connection: &NodeConnection,
                                              block_consumer: fn(SubstrateBlock) -> T)
                                              -> Result<()>
    where
        T: Future<Output=Result<()>> {
    let mut blocks_sub = match connection.get_api().blocks().subscribe_finalized().await {
        Ok(block_stream) => block_stream,
        Err(e) => return Err(Error::SUBSTRATE(format!("Encountered error while trying to subscribe to finalized blocks: {:?}", e)))
    };

    while let Some(block) = blocks_sub.next().await {
        match block {
            Ok(block) => block_consumer(block).await?,
            Err(e) => return Err(Error::SUBSTRATE(format!("Error occurred while fetching latest block: {:?}", e)))
        }

        get_energy_offers(connection).await;
    }

    Ok(())
}

async fn get_energy_offers(connection: &NodeConnection) {
    let storage_query = chain_apis::energychain_api::api::storage().energychain().energy_offers(
        H256::from_str("0x7e9fe0a9b607e795902361620ccdff72e0c4717f643c82eaaaff4e236b124b4a").expect("Unable to get H256 from public key")
    );

    let latest_block_storage = match connection.get_api().storage().at_latest().await {
        Ok(storage) => storage,
        Err(e) => {
            println!("Latest block could not be fetched due to: {:?}", e);
            return
        }
    };

    let res = match latest_block_storage.fetch(&storage_query).await {
        Ok(energy_offer) => energy_offer,
        Err(e) => {
            println!("Latest block could not be fetched due to: {:?}", e);
            return
        }
    };

    match res {
        None => println!("No offer found..."),
        Some(offer) => println!("Price: {}", offer.price)
    }
}
