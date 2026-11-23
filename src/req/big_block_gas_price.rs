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

  /// Returns the RPC method name for the request.
  #[inline(always)]
  pub const fn method_name() -> &'static str {
    "eth_bigBlockGasPrice"
  }
}

impl ToRpcParams for BigBlockGasPriceRequest {
  #[inline(always)]
  fn to_rpc_params(self) -> super::RpcResult {
    Ok(None)
  }
}
