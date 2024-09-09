WIP macros for autogenerating boilerplate formatting and serialisation code for gecko profiler markers in Rust. 

`src/lib.rs` -- contains an example/test marker payload struct and a skeleton marker formatting and serialisation API similar to the gecko API.
`macros/src/lib.rs` -- implementation of a `derive` macro for the `ProfilerMarker` trait.

**TODO:**

- [ ] - Refactor struct member parsing into separate pass. Extract attribute information at this point.
- [ ] - Refactor struct member attribute parsing to support syntax like: `#[marker_format(Integer, Searchable, name = 'More readable name here']` (see https://doc.rust-lang.org/reference/attributes.html#meta-item-attribute-syntax)
- [ ] - Polish + integration into Gecko.
