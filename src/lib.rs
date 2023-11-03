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
