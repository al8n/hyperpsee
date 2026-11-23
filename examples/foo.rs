use hyperpsee::{Client, GetSystemTxsRequest, SystemTransaction, UsingBigBlocksRequest};
use jsonrpsee::http_client::HttpClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = Client::new(HttpClientBuilder::default().build("https://rpc.hyperliquid.xyz/evm")?);

  let price = client.big_block_gas_price().await?;
  println!("{}", price);

  let using_big_blocks = client
    .using_big_blocks(UsingBigBlocksRequest::new(
      "0x0000000000000000000000000000000000000000".parse()?,
    ))
    .await?;
  println!("{}", using_big_blocks);

  let txs: Vec<SystemTransaction> = client
    .get_system_txs(GetSystemTxsRequest::BlockNumber(45_600_053))
    .await?;

  for tx in txs {
    println!("{:#?}", tx);
  }

  let txs: Vec<SystemTransaction> = client
    .get_system_txs(GetSystemTxsRequest::BlockHash(
      "0x796a25db7d96bc6a2fda5624eeeb73fa6cf6aa1d35ed8d179eadf4a58dff55e3".parse()?,
    ))
    .await?;

  for tx in txs {
    println!("{:#?}", tx);
  }
  Ok(())
}
