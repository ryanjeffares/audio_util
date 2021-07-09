# audio_util
An audio utility library written in pure Rust.

This library is designed to be used within an audio framework that will handle calls to some form of ProcessBlock function for you, for example vst-rs.
Therefore, functions in this library are CURRENTLY designed to process or return a single sample at a time instead of trying to interop with however the framework it's being used in handles buffers etc. We could come up with an elegant system for that however...

Many more features to implement, so no official release yet, but coming to crates.io... soon.