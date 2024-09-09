WIP macros for autogenerating boilerplate formatting and serialisation code for gecko profiler markers in Rust. 

`src/lib.rs` -- contains an example/test marker payload struct and a skeleton marker formatting and serialisation API similar to the gecko API.
`macros/src/lib.rs` -- implementation of a `derive` macro for the `ProfilerMarker` trait.