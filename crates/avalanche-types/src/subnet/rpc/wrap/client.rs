use std::io::{Error, ErrorKind, Result};

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
pub struct WrapSignerClient {
    inner: signer_client::SignerClient<Channel>,
}

impl WrapSignerClient {
    pub fn new(client_conn: Channel) -> Self {
        Self {
            inner: signer_client::SignerClient::new(client_conn)
                .max_decoding_message_size(usize::MAX)
                .max_encoding_message_size(usize::MAX),
        }
    }
}

#[tonic::async_trait]
impl super::WrapSignerClient for WrapSignerClient {
    async fn sign(&self,
                  network_id: u32,
                  source_chain_id: &str,
                  payload: Vec<u8>) -> Result<SignResponse> {
        let mut client = self.inner.clone();
        let res = client
            .sign(SignRequest {
                network_id,
                source_chain_id: Bytes::from(source_chain_id.to_vec()),
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