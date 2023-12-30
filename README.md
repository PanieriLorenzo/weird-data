# Weird Data

> Note: this crate is still in development and might change in the future.

Generate random data in such a way as to make rare edge-cases very likely.

For instance, if generating a random `f32` by uniformly sampling 32 bits of data, certain values will rarely appear, such as `NAN` and `INFINITY`. When doing randomized testing, like fuzzing, it isn't very useful to repeatedly generate well-behaved data. It is much more useful if we can artificially increase the likelihood of these special values, so that we test with them more often.

Additionally, some random number crates will never generate certain problematic bit-patterns, such as `NAN`.
