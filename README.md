# rustversion-detect
This crate provides a simple API for detecting the rustc
compiler version.

It is primarily intended for build scripts, but is also usable at runtime.

The implementation is forked from the [`rustversion` crate], but with proc-macro code removed.

[`rustversion` crate](https://github.com/dtolnay/rustversion)

```toml
[build-dependencies]
rustversion-detect = "0.1"
```

## License
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
