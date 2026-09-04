//! Cipher, statistics, number-theory and group-theory code behind the interactive labs.
//!
//! Everything here is plain Rust with no web dependencies, unit-tested natively with
//! `cargo test -p praxis-core`, and compiled into the WebAssembly bundle unchanged.
pub mod affine;
pub mod alphabet;
pub mod bifid;
pub mod groups;
pub mod hill;
pub mod numtheory;
pub mod periodic;
pub mod rng;
pub mod stats;
pub mod transposition;
