#[doc = include_str!("../README.md")]

pub mod semaphore;

#[allow(unused_imports)]
pub use semaphore::*;

#[cfg(test)]
mod tests;
