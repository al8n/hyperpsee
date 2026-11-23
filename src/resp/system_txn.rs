use alloy_primitives::{Address, BlockHash, BlockNumber, Bytes, TxHash, U8, U64, U256};

mod block_number_serde {
  use alloy_primitives::{BlockNumber, U64};
  use serde::{Deserialize, Serialize, de::Deserializer};

  pub fn deserialize<'de, D>(deserializer: D) -> Result<BlockNumber, D::Error>
  where
    D: Deserializer<'de>,
  {
    U64::deserialize(deserializer).map(|v| v.to::<u64>())
  }

  pub fn serialize<S>(block_number: &BlockNumber, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    U64::from(*block_number).serialize(serializer)
  }
}

/// A HyperCore-originated system transaction included in a HyperEVM block.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct SystemTransaction {
  /// Hash identifying this transaction.
  hash: TxHash,
  /// Hash of the HyperEVM block containing this transaction.
  block_hash: BlockHash,
  /// Height of the containing HyperEVM block.
  #[cfg_attr(feature = "serde", serde(with = "block_number_serde"))]
  block_number: BlockNumber,
  /// Transaction index reported by the node.
  transaction_index: U64,

  /// EVM transaction type as reported by the node.
  #[cfg_attr(feature = "serde", serde(rename = "type"))]
  ty: U8,
  /// EVM chain ID.
  chain_id: U64,
  /// Sender nonce.
  nonce: U64,

  /// Sender reported for this system transaction.
  from: Address,
  /// Recipient, or `None` for contract creation.
  #[cfg_attr(
    feature = "serde",
    serde(default, skip_serializing_if = "Option::is_none")
  )]
  to: Option<Address>,
  /// Native HYPE transferred, in wei.
  value: U256,
  /// Transaction input bytes (calldata).
  input: Bytes,

  /// Gas limit in gas units.
  gas: U64,
  /// Gas price in wei per gas.
  gas_price: U256,

  /// Raw `v` value reported by the node.
  v: U256,
  /// Raw `r` value reported by the node.
  r: U256,
  /// Raw `s` value reported by the node.
  s: U256,
}

impl SystemTransaction {
  /// Creates a new `SystemTransaction` with default values for all fields.
  #[inline(always)]
  pub const fn new() -> Self {
    Self {
      hash: TxHash::new([0; 32]),
      block_hash: BlockHash::new([0; 32]),
      block_number: 0,
      transaction_index: U64::from_limbs([0]),
      ty: U8::from_limbs([0]),
      chain_id: U64::from_limbs([0]),
      nonce: U64::from_limbs([0]),
      from: Address::new([0; 20]),
      to: None,
      value: U256::from_limbs([0; 4]),
      input: Bytes::new(),
      gas: U64::from_limbs([0]),
      gas_price: U256::from_limbs([0; 4]),
      v: U256::from_limbs([0; 4]),
      r: U256::from_limbs([0; 4]),
      s: U256::from_limbs([0; 4]),
    }
  }

  /// Sets the hash identifying this transaction.
  #[inline(always)]
  pub const fn with_hash(mut self, hash: TxHash) -> Self {
    self.hash = hash;
    self
  }

  /// Sets the hash identifying this transaction.
  /// Returns a mutable reference to the transaction after setting the hash.
  #[inline(always)]
  pub const fn set_hash(&mut self, hash: TxHash) -> &mut Self {
    self.hash = hash;
    self
  }

  /// Returns the hash identifying this transaction.
  #[inline(always)]
  pub const fn hash(&self) -> &TxHash {
    &self.hash
  }

  /// Sets the block hash of the containing HyperEVM block.
  #[inline(always)]
  pub const fn with_block_hash(mut self, block_hash: BlockHash) -> Self {
    self.block_hash = block_hash;
    self
  }

  /// Sets the block hash of the containing HyperEVM block.
  /// Returns a mutable reference to the transaction after setting the block hash.
  #[inline(always)]
  pub const fn set_block_hash(&mut self, block_hash: BlockHash) -> &mut Self {
    self.block_hash = block_hash;
    self
  }

  /// Returns the hash of the HyperEVM block containing this transaction.
  #[inline(always)]
  pub const fn block_hash(&self) -> &BlockHash {
    &self.block_hash
  }

  /// Sets the block number of the containing HyperEVM block.
  #[inline(always)]
  pub const fn with_block_number(mut self, block_number: BlockNumber) -> Self {
    self.block_number = block_number;
    self
  }

  /// Sets the block number of the containing HyperEVM block.
  /// Returns a mutable reference to the transaction after setting the block number.
  #[inline(always)]
  pub const fn set_block_number(&mut self, block_number: BlockNumber) -> &mut Self {
    self.block_number = block_number;
    self
  }

  /// Returns the height of the containing HyperEVM block.
  #[inline(always)]
  pub const fn block_number(&self) -> BlockNumber {
    self.block_number
  }

  /// Sets the transaction index reported by the node.
  #[inline(always)]
  pub const fn with_transaction_index(mut self, transaction_index: U64) -> Self {
    self.transaction_index = transaction_index;
    self
  }

  /// Sets the transaction index reported by the node.
  /// Returns a mutable reference to the transaction after setting the transaction index.
  #[inline(always)]
  pub const fn set_transaction_index(&mut self, transaction_index: U64) -> &mut Self {
    self.transaction_index = transaction_index;
    self
  }

  /// Returns the transaction index reported by the node.
  #[inline(always)]
  pub const fn transaction_index(&self) -> U64 {
    self.transaction_index
  }

  /// Sets the EVM transaction type as reported by the node.
  #[inline(always)]
  pub const fn with_ty(mut self, ty: U8) -> Self {
    self.ty = ty;
    self
  }

  /// Sets the EVM transaction type as reported by the node.
  /// Returns a mutable reference to the transaction after setting the type.
  #[inline(always)]
  pub const fn set_ty(&mut self, ty: U8) -> &mut Self {
    self.ty = ty;
    self
  }

  /// Returns the EVM transaction type as reported by the node.
  #[inline(always)]
  pub const fn ty(&self) -> U8 {
    self.ty
  }

  /// Sets the EVM chain ID.
  #[inline(always)]
  pub const fn with_chain_id(mut self, chain_id: U64) -> Self {
    self.chain_id = chain_id;
    self
  }

  /// Sets the EVM chain ID.
  /// Returns a mutable reference to the transaction after setting the chain ID.
  #[inline(always)]
  pub const fn set_chain_id(&mut self, chain_id: U64) -> &mut Self {
    self.chain_id = chain_id;
    self
  }

  /// Returns the EVM chain ID.
  #[inline(always)]
  pub const fn chain_id(&self) -> U64 {
    self.chain_id
  }

  /// Sets the sender nonce.
  #[inline(always)]
  pub const fn with_nonce(mut self, nonce: U64) -> Self {
    self.nonce = nonce;
    self
  }

  /// Sets the sender nonce.
  /// Returns a mutable reference to the transaction after setting the nonce.
  #[inline(always)]
  pub const fn set_nonce(&mut self, nonce: U64) -> &mut Self {
    self.nonce = nonce;
    self
  }

  /// Returns the sender nonce.
  #[inline(always)]
  pub const fn nonce(&self) -> U64 {
    self.nonce
  }

  /// Sets the sender reported for this system transaction.
  #[inline(always)]
  pub const fn with_from(mut self, from: Address) -> Self {
    self.from = from;
    self
  }

  /// Sets the sender reported for this system transaction.
  /// Returns a mutable reference to the transaction after setting the sender.
  #[inline(always)]
  pub const fn set_from(&mut self, from: Address) -> &mut Self {
    self.from = from;
    self
  }

  /// Returns the sender reported for this system transaction.
  #[inline(always)]
  pub const fn from(&self) -> &Address {
    &self.from
  }

  /// Sets the recipient, or `None` for contract creation.
  #[inline(always)]
  pub const fn with_to(mut self, to: Option<Address>) -> Self {
    self.to = to;
    self
  }

  /// Sets the recipient, or `None` for contract creation.
  /// Returns a mutable reference to the transaction after setting the recipient.
  #[inline(always)]
  pub const fn set_to(&mut self, to: Option<Address>) -> &mut Self {
    self.to = to;
    self
  }

  /// Returns the recipient, or `None` for contract creation.
  #[inline(always)]
  pub const fn to(&self) -> Option<&Address> {
    self.to.as_ref()
  }

  /// Sets the native HYPE transferred, in wei.
  #[inline(always)]
  pub const fn with_value(mut self, value: U256) -> Self {
    self.value = value;
    self
  }

  /// Sets the native HYPE transferred, in wei.
  /// Returns a mutable reference to the transaction after setting the value.
  #[inline(always)]
  pub const fn set_value(&mut self, value: U256) -> &mut Self {
    self.value = value;
    self
  }

  /// Returns the native HYPE transferred, in wei.
  #[inline(always)]
  pub const fn value(&self) -> &U256 {
    &self.value
  }

  /// Sets the transaction input bytes (calldata).
  #[inline(always)]
  pub fn with_input(mut self, input: Bytes) -> Self {
    self.input = input;
    self
  }

  /// Sets the transaction input bytes (calldata).
  /// Returns a mutable reference to the transaction after setting the input.
  #[inline(always)]
  pub fn set_input(&mut self, input: Bytes) -> &mut Self {
    self.input = input;
    self
  }

  /// Returns the transaction input bytes (calldata).
  #[inline(always)]
  pub const fn input(&self) -> &Bytes {
    &self.input
  }

  /// Sets the gas limit in gas units.
  #[inline(always)]
  pub const fn with_gas(mut self, gas: U64) -> Self {
    self.gas = gas;
    self
  }

  /// Sets the gas limit in gas units.
  /// Returns a mutable reference to the transaction after setting the gas limit.
  #[inline(always)]
  pub const fn set_gas(&mut self, gas: U64) -> &mut Self {
    self.gas = gas;
    self
  }

  /// Returns the gas limit in gas units.
  #[inline(always)]
  pub const fn gas(&self) -> U64 {
    self.gas
  }

  /// Sets the gas price in wei per gas.
  #[inline(always)]
  pub const fn with_gas_price(mut self, gas_price: U256) -> Self {
    self.gas_price = gas_price;
    self
  }

  /// Sets the gas price in wei per gas.
  /// Returns a mutable reference to the transaction after setting the gas price.
  #[inline(always)]
  pub const fn set_gas_price(&mut self, gas_price: U256) -> &mut Self {
    self.gas_price = gas_price;
    self
  }

  /// Returns the gas price in wei per gas.
  #[inline(always)]
  pub const fn gas_price(&self) -> &U256 {
    &self.gas_price
  }

  /// Sets the `v` value reported by the node.
  #[inline(always)]
  pub const fn with_v(mut self, v: U256) -> Self {
    self.v = v;
    self
  }

  /// Sets the `v` value reported by the node.
  /// Returns a mutable reference to the transaction after setting the `v` value.
  #[inline(always)]
  pub const fn set_v(&mut self, v: U256) -> &mut Self {
    self.v = v;
    self
  }

  /// Returns the raw `v` value reported by the node.
  #[inline(always)]
  pub const fn v(&self) -> &U256 {
    &self.v
  }

  /// Sets the `r` value reported by the node.
  #[inline(always)]
  pub const fn with_r(mut self, r: U256) -> Self {
    self.r = r;
    self
  }

  /// Sets the `r` value reported by the node.
  /// Returns a mutable reference to the transaction after setting the `r` value.
  #[inline(always)]
  pub const fn set_r(&mut self, r: U256) -> &mut Self {
    self.r = r;
    self
  }

  /// Returns the raw `r` value reported by the node.
  #[inline(always)]
  pub const fn r(&self) -> &U256 {
    &self.r
  }

  /// Sets the `s` value reported by the node.
  #[inline(always)]
  pub const fn with_s(mut self, s: U256) -> Self {
    self.s = s;
    self
  }

  /// Sets the `s` value reported by the node.
  /// Returns a mutable reference to the transaction after setting the `s` value.
  #[inline(always)]
  pub const fn set_s(&mut self, s: U256) -> &mut Self {
    self.s = s;
    self
  }

  /// Returns the raw `s` value reported by the node.
  #[inline(always)]
  pub const fn s(&self) -> &U256 {
    &self.s
  }
}

#[test]
fn test_system_txn_accessors() {
  assert_eq!(SystemTransaction::default(), SystemTransaction::new());
}
