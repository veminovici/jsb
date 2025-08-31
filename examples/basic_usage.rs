//! Basic usage example for the JSB Standard Library

use jsb_std::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    println!("JSB Standard Library Version: {}", VERSION);
    
    // Initialize the library
    init()?;
    
    println!("JSB Standard Library initialized successfully!");
    
    Ok(())
}
