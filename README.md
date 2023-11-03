Hash trait that is object-safe
==============================

This crate provides a `DynHash` trait that can be used in trait objects. Types
that implement the standard library's [`std::hash::Hash`] trait are
automatically usable by a `DynHash` trait object.

[`std::hash::Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html

## Example

```rust
use dyn_hash::DynHash;

trait MyTrait: DynHash {
    /* ... */
}

// Implement std::hash::Hash for dyn MyTrait
dyn_hash::hash_trait_object!(MyTrait);

// Now data structures containing Box<dyn MyTrait> can derive Hash:
#[derive(Hash)]
struct Container {
    trait_object: Box<dyn MyTrait>,
}
```

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
