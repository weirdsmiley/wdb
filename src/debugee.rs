//! This module is for the exection of the debuggee program.
use std::error::Error;
use std::sync::{Arc, Mutex};

// Get the modified binary file and run it.
pub(crate) fn continue_debugee(obj: Arc<Mutex<object::File>>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
