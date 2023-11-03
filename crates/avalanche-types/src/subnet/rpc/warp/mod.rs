pub mod client;

use std::io::Result;
use crate::proto::warp::SignResponse;

#[tonic::async_trait]
pub trait WarpSignerClient: Send + Sync + CloneBox {
    async fn sign(
        &self,
        network_id: u32,
        source_chain_id: &str,
        payload: Vec<u8>,
    ) -> Result<SignResponse>;
}

pub trait CloneBox {
    fn clone_box(&self) -> Box<dyn WarpSignerClient + Send + Sync>;
}

impl<T> CloneBox for T
    where
        T: 'static + WarpSignerClient + Clone + Send + Sync,
{
    fn clone_box(&self) -> Box<dyn WarpSignerClient + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn WarpSignerClient + Send + Sync> {
    fn clone(&self) -> Box<dyn WarpSignerClient + Send + Sync> {
        self.clone_box()
    }
}