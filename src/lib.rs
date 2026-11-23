#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]

pub use endpoint::Client;
pub use req::*;
pub use resp::*;

mod req;
mod resp;

mod endpoint;
