use super::super::prelude::*;
use std::borrow::Borrow;

pub struct NodeConnection {
    client: subxt::OnlineClient::<subxt::SubstrateConfig>,
}

impl NodeConnection {
    pub async fn new(node_url: impl Into<String>) -> Result<NodeConnection> {
        let mut url = String::with_capacity(256);
        url.push_str(&node_url.into());

        let client = match subxt::OnlineClient::<subxt::SubstrateConfig>::from_url(format!("ws://{}", &url)).await {
            Ok(api) => api,
            Err(e) => return Err(Error::Substrate(format!("An error occurred while trying to establish a connection to the node listening at '{url}': {:?}", e)))
        };

        Ok(NodeConnection { client })
    }

    pub fn get_api(&self) -> &subxt::OnlineClient<subxt::SubstrateConfig> {
        self.client.borrow()
    }
}

#[cfg(test)]
mod tests {
    use crate::substrate_utils::*;
}
