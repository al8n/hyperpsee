use hyperpsee::{BigBlockGasPrice, BigBlockGasPriceRequest};
use jsonrpsee::{core::client::ClientT, http_client::HttpClientBuilder, rpc_params};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = HttpClientBuilder::default().build("https://rpc.hyperliquid.xyz/evm")?;

  let price: BigBlockGasPrice = client
    .request("eth_bigBlockGasPrice", BigBlockGasPriceRequest::default())
    .await?;
  println!("下一个大块的基础费率：{}", price);

  Ok(())
}
