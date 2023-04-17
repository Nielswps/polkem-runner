#[allow(unused)] //todo!("Remove")
use crate::substrate_utils::node_connection::NodeConnection;
use crate::prelude::*;
use crate::substrate_utils::node_driver;

use futures::StreamExt;
use subxt::ext::sp_core::Pair;
use subxt::{OnlineClient, SubstrateConfig, tx::PairSigner};
use subxt::blocks::ExtrinsicEvents;
use subxt::events::EventDetails;

pub mod chain_apis;
pub mod substrate_utils;
pub mod error;
pub mod prelude;

#[allow(unused)] //todo!("Remove")
pub async fn something(connection: &NodeConnection) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // let pair = subxt::ext::sp_core::sr25519::Pair::from_string_with_seed("0x76c10f83e6014ac6c5ab6de573bd7fa7be327b6445b755fd0db1c58b9320e6a6", None).unwrap();
    // let signer: PairSigner<SubstrateConfig, subxt::ext::sp_core::sr25519::Pair> = PairSigner::<SubstrateConfig, subxt::ext::sp_core::sr25519::Pair>::new(pair.0);
    //
    // let payload = chain_apis::energychain_api::api::tx().energychain().create_energy_offer(500, 4);
    //
    // connection.get_api().tx().create_unsigned(&payload).unwrap()
    //     .submit_and_watch().await?
    //     .wait_for_finalized().await?;

    // let peer_id = "5EvjPEhDqnDsMsqdR2DFViEaLSHhw7Psk2pFLHfVEzoAFtTm";
    // let amount = 15550;
    // let price = 2;
    //
    // let parameters = rpc_params!["null", peer_id, amount, price];
    //
    // api.rpc()
    //     .call("create_energy_offer".into(), Some(parameters.build().unwrap().get()), None)
    //     .await?;
    // .sign_and_submit_then_watch_default(&payload, &signer)
    // .await?
    // .wait_for_finalized_success()
    // .await?;

    async fn handle_block(block: SubstrateBlock) -> crate::prelude::Result<()> {
        let block_number = block.header().number;
        let block_hash = block.hash();

        println!("Block #{block_number}:");
        println!("  Hash: {block_hash}");
        println!("  Extrinsics:");

        let body = match block.body().await {
            Ok(block_body) => block_body,
            Err(e) => return Err(Error::SUBSTRATE(format!("Encountered error while trying to get block body: {:?}", e)))
        };
        for ext in body.extrinsics() {
            let idx = ext.index();
            let events = match ext.events().await {
                Ok(extrinsic_events) => extrinsic_events,
                Err(e) => return Err(Error::SUBSTRATE(format!("Encountered error while trying to get extrinsic events: {:?}", e)))
            };

            let bytes_hex = format!("0x{}", hex::encode(ext.bytes()));

            println!("    Extrinsic #{idx}:");
            println!("      Bytes: {bytes_hex}");
            println!("      Events:");

            for evt in events.iter() {
                let evt = match evt {
                    Ok(details) => details,
                    Err(e) => return Err(Error::SUBSTRATE(format!("Encountered error while trying to get event details: {:?}", e)))
                };

                let pallet_name = evt.pallet_name();
                let event_name = evt.variant_name();

                println!("        {pallet_name}_{event_name}");
            }
        };

        Ok(())
    }

    node_driver::subscribe_to_finalized_blocks(connection, |block| handle_block(block)).await;

    Ok(())
}