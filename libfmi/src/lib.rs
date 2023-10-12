//! Generated Rust [fmi-standard](https://fmi-standard.org/) bindings.
//!
//! This crate also includes a variadic logging handler as inspired by [rust-fmi](https://gitlab.com/jondo2010/rust-fmi).
//!
//! # Example
//!
//! ```no_run
//! use libfmi::Fmi2Dll;
//!
//! let fmi = unsafe { Fmi2Dll::new("../tests/fmu/bouncing_ball/binaries/linux64/bouncing_ball.so") }?;
//! let version = unsafe { fmi.fmi2GetVersion() };
//!
//! println!("FMI version: {:?}", unsafe {
//!     std::ffi::CStr::from_ptr(version)
//! });
//!
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

#[allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]
mod fmi;
pub mod logger;

pub use fmi::*;
