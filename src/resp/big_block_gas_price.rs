use alloy_primitives::U256;

/// Base fee per gas for the next HyperEVM big block.
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  derive_more::From,
  derive_more::Into,
  derive_more::Display,
  derive_more::AsRef,
  derive_more::Deref,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[display("{_0} wei/gas")]
pub struct BigBlockGasPriceResponse(U256);

impl BigBlockGasPriceResponse {
  /// Returns the base fee in wei per gas.
  #[inline(always)]
  pub const fn value(&self) -> &U256 {
    &self.0
  }
}
