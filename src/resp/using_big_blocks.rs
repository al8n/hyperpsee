/// Response indicating whether an address is using big blocks.
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  derive_more::From,
  derive_more::Into,
  derive_more::Display,
  derive_more::AsRef,
  derive_more::Deref,
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[display("{_0}")]
pub struct UsingBigBlocksResponse(bool);

impl UsingBigBlocksResponse {
  /// Returns whether the address is using big blocks.
  #[inline(always)]
  pub const fn value(&self) -> bool {
    self.0
  }
}
