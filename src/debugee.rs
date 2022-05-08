//! This module is for the exection of the debuggee program.
use std::error::Error;

// Get the modified binary file and run it.
// TODO: Need to figure out how to modify the current binary to change its
// first byte of every instruction to 0xcc. We need a better mechanism to
// run every instruction which is read from an elf file.
pub fn continue_debugee(bin: &Vec<u8>) -> Result<(), Box<dyn Error>> {
    // Run obj binary, but this is not a binary, it is an object file
    Ok(())
}
