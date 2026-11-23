use hyperpsee::{
  BigBlockGasPriceRequest, BigBlockGasPriceResponse, UsingBigBlocksRequest, UsingBigBlocksResponse,
};
use jsonrpsee::{core::client::ClientT, http_client::HttpClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client = HttpClientBuilder::default().build("https://rpc.hyperliquid.xyz/evm")?;

  let price: BigBlockGasPriceResponse = client
    .request("eth_bigBlockGasPrice", BigBlockGasPriceRequest::default())
    .await?;
  println!("下一个大块的基础费率：{}", price);

  let using_big_blocks: UsingBigBlocksResponse = client
    .request(
      "eth_usingBigBlocks",
      UsingBigBlocksRequest::new("0x0000000000000000000000000000000000000000".parse()?),
    )
    .await?;
  println!("地址是否使用大块：{}", using_big_blocks);
  Ok(())
}
