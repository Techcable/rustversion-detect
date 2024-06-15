# compile benchmarks
Some informal benchmarks on compilation:

## On an M1 Mac on 2024-06-15:
The `rustversion` crate takes about 0.22 secs to compile, while `rustversion-detect` takes 0.09 secs to compile.
Regardless of the crate, compiling the `build.rs` script for the test crate takes about 0.11 seconds to compile and measures 0 seconds to run.

For the rustversion & rustversion-detect crate it takes around 0.15 seconds to compile and 0.5 seconds to run

However, when using rustversion only at runtime (not in `build.rs`), the total compile time is roughtly the same.
If you already have a `build.rs` script or have other dependencies the crate can wait on,
then `rustversion-detect` will probably be somewhat faster and lighter.

I hope the difference is more significant than 0.1 seconds on slower computers, or else this whole project would be kind of wasteful.
