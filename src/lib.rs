#![cfg_attr(nightly, deny(missing_docs))]
#![cfg_attr(nightly, feature(external_doc))]
#![cfg_attr(nightly, doc(include = "../README.md"))]
// #![cfg_attr(test, deny(warnings))]

mod bitfield;
mod change;
mod iter;

pub use crate::bitfield::Bitfield;
pub use crate::change::Change;
pub use crate::iter::Iter;
