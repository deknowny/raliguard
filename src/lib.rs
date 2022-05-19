#![cfg_attr(not(debug_assertions), deny(warnings, missing_docs, clippy::all, clippy::cargo))]
#![doc = include_str!("../README.md")]

pub mod semaphore;

#[allow(unused_imports)]
pub use semaphore::*;

#[cfg(test)]
mod tests;
