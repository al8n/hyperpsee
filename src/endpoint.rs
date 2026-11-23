use jsonrpsee::{
  core::client::{ClientT, Error},
  http_client::HttpClient,
};

use crate::{
  BigBlockGasPriceRequest, BigBlockGasPriceResponse, GetSystemTxsRequest, SystemTransaction,
  UsingBigBlocksRequest, UsingBigBlocksResponse,
};

/// A client for interacting with the HyperEVM JSON-RPC endpoints.
pub struct Client {
  client: HttpClient,
}

impl Client {
  /// Creates a new `Client` with the given `HttpClient`.
  pub fn new(client: HttpClient) -> Self {
    Self { client }
  }

  /// Retrieves the system transactions included in the HyperEVM block with the specified block number.
  pub async fn get_system_txs(
    &self,
    req: GetSystemTxsRequest,
  ) -> Result<Vec<SystemTransaction>, Error> {
    self.client.request(req.method_name(), req).await
  }

  /// Retrieves the current gas price for big blocks in the HyperEVM network.
  pub async fn big_block_gas_price(&self) -> Result<BigBlockGasPriceResponse, Error> {
    self
      .client
      .request(
        BigBlockGasPriceRequest::method_name(),
        BigBlockGasPriceRequest::new(),
      )
      .await
  }

  /// Returns whether the specified address is using big blocks in the HyperEVM network.
  pub async fn using_big_blocks(
    &self,
    params: UsingBigBlocksRequest,
  ) -> Result<UsingBigBlocksResponse, Error> {
    self
      .client
      .request(UsingBigBlocksRequest::method_name(), params)
      .await
  }
}
