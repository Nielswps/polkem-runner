use std::ops::Add;
use polkem_runner::{
    prelude::*,
    log_finalised_blocks,
    log_proposed_and_finalised_blocks,
};

use clap::Parser;
use polkem_runner::substrate_utils::node_connection::NodeConnection;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short)]
    log_endpoint: String,

    #[clap(long, short, default_value_t = String::from("localhost:9944"))]
    node_url: String,

    #[clap(long, short, default_value_t = String::from("node"))]
    node_id: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Process execution parameters
    let args = Args::parse();

    let log_endpoint = match args.log_endpoint.len() < 256 {
        true => args.log_endpoint,
        false => return Err(Error::CLI(format!("Log endpoint must be less than 256 characters")))
    };

    let node_url = match args.node_url.len() < 256 {
        true => args.node_url,
        false => return Err(Error::CLI(format!("Node url must be less than 256 characters")))
    };

    let node_id = match args.node_id.len() < 128 {
        true => args.node_id,
        false => return Err(Error::CLI(format!("Node ID must be less than 128 characters")))
    };

    println!("Endpoint for logging: {}\nURL of running node: {}\nID for node: {}", &log_endpoint, &node_url, &node_id);

    // Start connections to node and logging agent
    let node_connection_1 = NodeConnection::new(&node_url).await?;
    let node_connection_2 = NodeConnection::new(&node_url).await?;
    let endpoint_for_logs = if log_endpoint.ends_with('/') { log_endpoint } else { log_endpoint.add("/") };

    // Start logging proposed and finalised blocks
    match log_proposed_and_finalised_blocks(node_connection_1, node_connection_2, endpoint_for_logs.add(&node_id).as_str()).await {
        Ok(_) => {}
        Err(e) => println!("{}", e.to_string())
    };

    Ok(())
}
