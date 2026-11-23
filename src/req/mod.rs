use alloy_primitives::{BlockHash, BlockNumber, U64};
use derive_more::{From, IsVariant, TryUnwrap, Unwrap};
use jsonrpsee::core::traits::ToRpcParams;
use serde_json::value::{RawValue, to_raw_value};

pub use big_block_gas_price::BigBlockGasPriceRequest;

pub use using_big_blocks::UsingBigBlocksRequest;

type RpcResult = Result<Option<Box<RawValue>>, serde_json::Error>;

mod big_block_gas_price;
mod using_big_blocks;

/// Represents a request to retrieve system transactions by either block hash or block number.
#[derive(Debug, Clone, PartialEq, Eq, Hash, IsVariant, Unwrap, TryUnwrap, From)]
#[unwrap(ref, ref_mut)]
#[try_unwrap(ref, ref_mut)]
pub enum GetSystemTxsRequest {
  /// Gets the system transactions for the block with the specified hash.
  BlockHash(BlockHash),
  /// Gets the system transactions for the block with the specified number.
  BlockNumber(BlockNumber),
}

impl GetSystemTxsRequest {
  /// Returns the RPC method name corresponding to the request.
  #[inline(always)]
  pub const fn method_name(&self) -> &'static str {
    match self {
      Self::BlockHash(_) => "eth_getSystemTxsByBlockHash",
      Self::BlockNumber(_) => "eth_getSystemTxsByBlockNumber",
    }
  }
}

impl ToRpcParams for GetSystemTxsRequest {
  #[inline(always)]
  fn to_rpc_params(self) -> RpcResult {
    let param = match self {
      Self::BlockHash(hash) => to_raw_value(&[hash])?,
      Self::BlockNumber(number) => to_raw_value(&[U64::from(number)])?,
    };
    Ok(Some(param))
  }
}
