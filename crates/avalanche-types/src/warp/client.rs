use std::io::{Error, ErrorKind, Read, Result};

use crate::{
    proto::pb::warp::{
        signer_client,
        SignRequest,
        SignResponse,
    },
};
use prost::bytes::Bytes;
use tonic::transport::Channel;

#[derive(Clone)]
pub struct WarpSignerClient {
    inner: signer_client::SignerClient<Channel>,
}

impl WarpSignerClient {
    pub fn new(client_conn: Channel) -> Self {
        Self {
            inner: signer_client::SignerClient::new(client_conn)
                .max_decoding_message_size(usize::MAX)
                .max_encoding_message_size(usize::MAX),
        }
    }
}

#[tonic::async_trait]
impl super::WarpSignerClient for WarpSignerClient {
    async fn sign(&self,
                  network_id: u32,
                  source_chain_id: &str,
                  payload: Vec<u8>) -> Result<SignResponse> {
        let mut client = self.inner.clone();
        let chain_id = String::from(source_chain_id);
        let res = client
            .sign(SignRequest {
                network_id,
                source_chain_id: Bytes::from(chain_id),
                payload: Bytes::from(payload),
            })
            .await
            .map_err(|e| {
                Error::new(
                    ErrorKind::Other,
                    format!("sign failed: {:?}", e),
                )
            })?;
        Ok(res.into_inner())
    }
}