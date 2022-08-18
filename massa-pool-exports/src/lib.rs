//! Copyright (c) 2022 MASSA LABS <info@massa.net>

//! Pool of operation and endorsements waiting to be included in a block
//!
#![warn(missing_docs)]
#![warn(unused_crate_dependencies)]
#![feature(map_first_last)]

mod config;
mod controller_traits;
mod error;
mod types;

pub use config::PoolConfig;
pub use controller_traits::PoolController;
pub use error::PoolError;

#[cfg(feature = "testing")]
pub mod tests;
