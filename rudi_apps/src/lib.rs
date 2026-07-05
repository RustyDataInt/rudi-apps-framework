//! The `rudi_apps` Rust library crate provides the framework for 
//! developing Rusty Data Interface (RuDI) interactive apps. The 
//! framework defines structures, components, utilities, and server 
//! functions to quickly build modular apps in a proven UI pattern.
//! Its public tools are called either by a tool suite's server 
//! `build.rs` and `main.rs` or by an app's runtime.

// modules
pub mod components; // where dx components installs them
pub mod server;
pub mod ui;
pub mod api;
pub mod prelude;
