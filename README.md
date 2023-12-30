# Weird Data

> Note: this crate is still in development and might change in the future.

> Disclaimer: the random number generators used in this crate are NOT CRYPTOGRAPHICALLY SECURE. Only use these generators for generating testing inputs, do not rely on them for cryptographic purposes in production code! For instance, you may test a cryptographic tool with these generators, but you may not deploy code that relies on these generators for security in production.

Generate random data in such a way as to make rare edge-cases very likely.

For instance, if generating a random `f32` by uniformly sampling 32 bits of data, certain values will rarely appear, such as `NAN` and `INFINITY`. When doing randomized testing, like fuzzing, it isn't very useful to repeatedly generate well-behaved data. It is much more useful if we can artificially increase the likelihood of these special values, so that we test with them more often.

Additionally, some random number crates will never generate certain problematic bit-patterns, such as `NAN`.

## Roadmap
- [ ] generate weird `f32` and `f64` values
    - [ ] generate random `NAN` numbers
        - [ ] generate random quiet `NAN` numbers
        - [ ] generate random signaling `NAN` numbers
    - [ ] generate random denormal numbers
    - [ ] generate special values, like `EPSILON` and `-0.0`
- [ ] generate weird integers, like 0, `MAX`, `MIN`
- [ ] generate weird UTF-8 strings
- [ ] generate random data-structures, leveraging other generators
    - [ ] fill array with random data
    - [ ] generate vectors with random data
    - [ ] generate vectors of problematic sizes
- [ ] generate random structs with macros and patterns, leveraging other generators
