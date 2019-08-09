# Middle Square Weyl Sequence RNG for Rust

[![travis](https://img.shields.io/travis/tidwall/weyl-rs.svg)](https://travis-ci.org/tidwall/weyl-rs/)
[![license](https://img.shields.io/crates/l/weyl.svg)](LICENSE)
[![version](https://img.shields.io/crates/v/weyl.svg)](https://crates.io/crates/weyl/)
[![documentation](https://docs.rs/weyl/badge.svg)](https://docs.rs/weyl/)

A new implementation of John von Neumann's middle square random number generator (RNG).
A Weyl sequence is utilized to keep the generator running through a long period.

[Paper](https://arxiv.org/pdf/1704.00358.pdf)

## Using

This library includes a few thread-safe functions.

```rust
weyl::u64()            // generates a random u64
weyl::f64()            // generates a random f64
weyl::fill(&mut bytes) // fill byte slice with random data
weyl::seed(my_seed)    // reseed the number generator
```

There's also a `Generator` type in case you need to generate random numbers in
an isolated thread. It's little faster because it avoids mutex locks.

```rust
let mut rand = weyl::Generator::new(my_seed);
println!("{}", rand.u64());             // generates a random u64
```

## Contact

Josh Baker [@tidwall](http://twitter.com/tidwall)

## License

Weyl source code is available under the ISC [License](/LICENSE).

