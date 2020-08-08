//! Swift ABI metadata.
//!
//! # Relevant files
//!
//! - [`Metadata.h`](https://github.com/apple/swift/blob/master/include/swift/ABI/Metadata.h)
//! - [`MetadataValues.h`](https://github.com/apple/swift/blob/master/include/swift/ABI/MetadataValues.h)
//! - [`ValueWitness.def`](https://github.com/apple/swift/blob/master/include/swift/ABI/ValueWitness.def)

mod value_witness;

pub use value_witness::*;