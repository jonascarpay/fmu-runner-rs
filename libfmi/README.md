# libfmi

[![Crates.io](https://img.shields.io/crates/v/libfmi.svg?maxAge=2592000)](https://crates.io/crates/libfmi)
[![Documentation](https://docs.rs/libfmi/badge.svg)](https://docs.rs/libfmi)
![Crates.io](https://img.shields.io/crates/l/libfmi.svg?maxAge=2592000)

<!-- cargo-rdme start -->

Generated Rust [fmi-standard](https://fmi-standard.org/) bindings.

This crate also includes a variadic logging handler as inspired by [rust-fmi](https://gitlab.com/jondo2010/rust-fmi).

## Example

```rust
use libfmi::Fmi2Dll;

let fmi = unsafe { Fmi2Dll::new("../tests/fmu/bouncing_ball/binaries/linux64/bouncing_ball.so") }?;
let version = unsafe { fmi.fmi2GetVersion() };

println!("FMI version: {:?}", unsafe {
    std::ffi::CStr::from_ptr(version)
});

```

<!-- cargo-rdme end -->
