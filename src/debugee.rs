//! This module is for the exection of the debuggee program.
use std::error::Error;

// Get the modified binary file and run it.
pub fn continue_debugee(bin: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    // FIXME: Run obj binary, but this is not a binary, it is an object
    // file
    Ok(())
}
