Hash trait that is object-safe
==============================

[<img alt="github" src="https://img.shields.io/badge/github-dtolnay/dyn--hash-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/dtolnay/dyn-hash)
[<img alt="crates.io" src="https://img.shields.io/crates/v/dyn-hash.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/dyn-hash)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-dyn--hash-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/dyn-hash)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/dtolnay/dyn-hash/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/dtolnay/dyn-hash/actions?query=branch%3Amaster)

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
