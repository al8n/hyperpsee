use alloy_primitives::Address;
use jsonrpsee::core::traits::ToRpcParams;

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
}

impl ToRpcParams for UsingBigBlocksRequest {
  fn to_rpc_params(self) -> Result<Option<Box<serde_json::value::RawValue>>, serde_json::Error> {
    let mut params = jsonrpsee::core::params::ArrayParams::new();
    params.insert(self.0).and_then(|_| params.to_rpc_params())
  }
}
