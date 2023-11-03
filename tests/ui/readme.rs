use std::hash::Hash;

trait MyTrait: Hash {
    /* ... */
}

fn main() {
    let _: &dyn MyTrait;
}
