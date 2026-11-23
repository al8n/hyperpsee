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
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[display("{_0} wei/gas")]
pub struct BigBlockGasPrice(U256);

impl BigBlockGasPrice {
  /// Returns the base fee in wei per gas.
  #[inline(always)]
  pub const fn get(&self) -> &U256 {
    &self.0
  }
}
