pub mod entities;
pub mod error;
mod http;
pub mod session;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate failure;
extern crate reqwest;
