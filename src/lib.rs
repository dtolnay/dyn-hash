use core::hash::{Hash, Hasher};

pub trait DynHash {
    fn hash(&self, state: &mut dyn Hasher);
}

impl<T: Hash> DynHash for T {
    fn hash(&self, mut state: &mut dyn Hasher) {
        Hash::hash(self, &mut state);
    }
}

impl Hash for dyn DynHash {
    fn hash<H: Hasher>(&self, state: &mut H) {
        DynHash::hash(self, state);
    }
}
