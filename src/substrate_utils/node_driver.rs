use std::future::Future;
use codec::Output;
use futures::StreamExt;
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
    }

    Ok(())
}
