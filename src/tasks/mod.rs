//! Builtin tasks
pub mod cargo;
pub mod cp;
pub mod env;
pub mod link;
pub mod sh;
#[cfg(feature = "network")]
pub mod wget;
