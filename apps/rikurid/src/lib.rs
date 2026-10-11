//! `rikurid`, the trusted Rikuri daemon (Stage Plan S00.07). It is the only
//! process that changes canonical state (Architecture §6.2). Clients reach
//! it through the versioned envelope in `rikuri-api`, either over its local
//! socket ([`server`]) or in-process ([`kernel::Kernel::handle`]); both run
//! the same kernel code. See `README.md` for the Module Contract.

mod candidate;
pub mod crash;
pub mod entities;
pub mod health;
pub mod kernel;
pub mod server;

pub use kernel::{Idempotency, Kernel, KernelConfig, KernelError};
