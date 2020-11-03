//! Module for constants
//!
//! This module contains constants available to other files.

/// Crate version
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION"); // compile even without cargo;

/// Author name
pub const AUTHOR: &'static str = "109149 <109149qwe@gmail.com>";

/// Crate about information
pub const ABOUT: &'static str = "Initial project generator";

// src/suggestions.rs
