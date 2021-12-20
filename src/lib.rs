#![allow(dead_code)]

//! # spwnmsg-core
//! 
//! ## Typedefs and basic functions for interacting with spwnmsg

pub use tokio;

pub mod base_types;

// #[cfg(feature = "server")]
pub mod server;

// #[cfg(feature = "client")]
pub mod client;
