//! Prelude module for JSB Standard Library
//!
//! This module re-exports commonly used items for convenience.

pub use crate::init;
pub use crate::VERSION;

// Re-export common standard library types
pub use std::{error::Error, fmt, io, result::Result};

// Re-export common collections
#[cfg(feature = "alloc")]
pub use alloc::{boxed::Box, string::String, vec::Vec};
