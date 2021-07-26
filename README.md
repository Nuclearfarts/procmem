# procmem
A simple cross-platform library for accessing the memory of other processes. Uses the [process-memory crate](https://crates.io/crates/process-memory) by Tommoa for functionality.

### A note on binary licensing
This repository is released under the CC0 license (contained in LICENSE).
However, the process-memory crate and the Rust standard library, both of which are linked statically, are licensed under MIT, meaning that attribution must given in binary distributions.
The process-memory license is available [here](https://github.com/Tommoa/rs-process-memory/blob/master/LICENSE) for convenience.
There are no clear guidelines for attribution for the Rust standard library (see https://github.com/rust-lang/rust/issues/67014).
