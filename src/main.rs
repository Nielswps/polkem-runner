#[allow(unused)] //todo!("Remove")

use polkem_runner::{
    prelude::*,
    something
};

use clap::Parser;
use polkem_runner::substrate_utils::node_connection::NodeConnection;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long, short, default_value_t = String::from("ws://localhost:9944"))]
    node_url: String,

    #[clap(long, short, default_value_t = String::from("/"))]
    data_location: String,
}

#[allow(unused)] //todo!("Remove")
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let node_url = match args.node_url.len() < 256 {
        true => args.node_url,
        false => return Err(Error::CLI(format!("Node url must be less than 256 characters")))
    };

    let data_location = match args.data_location.len() < 1024 {
        true => args.data_location,
        false => return Err(Error::CLI(format!("Path to data must be less than 1024 characters")))
    };

    let node_connection = NodeConnection::new(node_url).await?;

    match something(&node_connection).await {
        Ok(_) => {}
        Err(e) => println!("{}", e.to_string())
    };

    Ok(())
}
