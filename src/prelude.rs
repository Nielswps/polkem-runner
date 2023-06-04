pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub type SubstrateBlock = subxt::blocks::Block<subxt::SubstrateConfig, subxt::OnlineClient<subxt::SubstrateConfig>>;