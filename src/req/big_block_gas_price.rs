use jsonrpsee::core::traits::ToRpcParams;

/// Base fee per gas for the next HyperEVM big block.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BigBlockGasPriceRequest(());

impl BigBlockGasPriceRequest {
  /// Creates a new `BigBlockGasPriceRequest`.
  #[inline(always)]
  pub const fn new() -> Self {
    Self(())
  }
}

impl ToRpcParams for BigBlockGasPriceRequest {
  fn to_rpc_params(self) -> Result<Option<Box<jsonrpsee::core::JsonRawValue>>, serde_json::Error> {
    jsonrpsee::core::params::ArrayParams::new().to_rpc_params()
  }
}
