//! [![github]](https://github.com/dtolnay/dyn-hash)&ensp;[![crates-io]](https://crates.io/crates/dyn-hash)&ensp;[![docs-rs]](https://docs.rs/dyn-hash)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! <br>
//!
//! This crate provides a [`DynHash`] trait that can be used in trait objects.
//! Types that implement the standard library's [`std::hash::Hash`] trait are
//! automatically usable by a `DynHash` trait object.
//!
//! # Example
//!
//! ```
//! use dyn_hash::DynHash;
//!
//! trait MyTrait: DynHash {
//!     /* ... */
//! }
//!
//! // Implement std::hash::Hash for dyn MyTrait
//! dyn_hash::hash_trait_object!(MyTrait);
//!
//! // Now data structures containing Box<dyn MyTrait> can derive Hash:
//! #[derive(Hash)]
//! struct Container {
//!     trait_object: Box<dyn MyTrait>,
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/dyn-hash/0.0.0")]

mod macros;

use core::hash::{Hash, Hasher};

/// This trait is implemented for any type that implements [`std::hash::Hash`].
pub trait DynHash {
    fn hash(&self, state: &mut dyn Hasher);
}

impl<T: Hash + ?Sized> DynHash for T {
    fn hash(&self, mut state: &mut dyn Hasher) {
        Hash::hash(self, &mut state);
    }
}

hash_trait_object!(DynHash);

// Not public API. Referenced by macro-generated code.
#[doc(hidden)]
pub mod __private {
    pub use core::hash::{Hash, Hasher};
    pub use core::marker::{Send, Sync};
}
