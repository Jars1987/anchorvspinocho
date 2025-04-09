#![no_std]

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

#[cfg(feature = "std")]
extern crate std;

pub mod error;
pub mod instruction;
pub mod state;

pinocchio_pubkey::declare_id!("A24MN2mj3aBpDLRhY6FonnbTuayv7oRqhva2R2hUuyqx");
