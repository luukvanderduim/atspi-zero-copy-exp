# zero-copy experiment

Simplified the [atspi](https://github.com/odilia-app/atspi) crate to see whether zero-copy is possible.

# how to

```term
cargo test
```
 will demonstrate deserializing a borrowed (or owned) body.
 
# caveats

The code is simplified a lot to make it easier to reason about.
It is very well possible that I oversimplified it in the process.

We'll see.

