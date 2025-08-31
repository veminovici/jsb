//! Prelude module for JSB Standard Library
//! 
//! This module re-exports commonly used items for convenience.

pub use crate::VERSION;
pub use crate::init;

// Re-export common standard library types
pub use std::{
    fmt,
    io,
    error::Error,
    result::Result,
};

// Re-export common collections
#[cfg(feature = "alloc")]
pub use alloc::{
    string::String,
    vec::Vec,
    boxed::Box,
};
