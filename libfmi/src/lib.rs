//! Generated Rust [fmi-standard](https://fmi-standard.org/) bindings.
//!
//! This crate also includes a variadic logging handler as inspired by [rust-fmi](https://gitlab.com/jondo2010/rust-fmi).

#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
mod fmi;
pub mod logger;

pub use fmi::*;
