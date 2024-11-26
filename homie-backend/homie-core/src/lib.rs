#![deny(clippy::all)]

#[cfg(feature = "db")]
pub mod adapter;
pub mod domain;
pub mod error;

#[cfg(test)]
mod tests;
