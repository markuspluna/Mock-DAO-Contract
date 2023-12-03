#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

mod contract;
mod dependencies;

pub use contract::*;
