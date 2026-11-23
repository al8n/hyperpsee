use alloy_primitives::Address;
use jsonrpsee::core::traits::ToRpcParams;
use serde_json::value::to_raw_value;

/// Request to check if an address is using big blocks.
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  derive_more::From,
  derive_more::Into,
  derive_more::AsRef,
  derive_more::Deref,
)]
pub struct UsingBigBlocksRequest(Address);

impl UsingBigBlocksRequest {
  /// Creates a new `UsingBigBlocksRequest`.
  #[inline(always)]
  pub const fn new(address: Address) -> Self {
    Self(address)
  }

  /// Returns the address associated with this request.
  #[inline(always)]
  pub const fn address(&self) -> &Address {
    &self.0
  }

  /// Returns the JSON-RPC method name for this request.
  #[inline(always)]
  pub const fn method_name() -> &'static str {
    "eth_usingBigBlocks"
  }
}

impl ToRpcParams for UsingBigBlocksRequest {
  #[inline(always)]
  fn to_rpc_params(self) -> super::RpcResult {
    to_raw_value(&[self.as_ref()]).map(Some)
  }
}
