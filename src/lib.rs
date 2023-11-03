mod macros;

use core::hash::{Hash, Hasher};

pub trait DynHash {
    fn hash(&self, state: &mut dyn Hasher);
}

impl<T: Hash> DynHash for T {
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
