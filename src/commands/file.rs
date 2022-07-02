use object::Object;
use std::{env, fs, process};

use crate::debugger::{init_debugger, Context};
use crate::utils::{wdbError, wdbErrorKind};
use std::error::Error;

// All structs of type *Ty are actual debugger commands(removing Ty at
// end). All of their members are placeholders for their possible
// options.
/// Load a new binary and re-run the debugger.
pub(crate) struct FileTy {
    pub(crate) path: String,
}

impl FileTy {
    pub(crate) fn new(bin: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(FileTy { path: bin })
    }

    pub(crate) fn dump(&self) -> String {
        self.path.to_string()
    }
}

// The only task is to load the new binary as member 'binary' and then can we
// run main
impl crate::commands::CmdTy for FileTy {
    type cmd = String;
    fn process(&mut self, cmd: Self::cmd) -> Result<(), Box<dyn Error>> {
        let v: Vec<&str> = cmd.split_whitespace().collect();

        if v.len() != 2 {
            return Err(Box::new(wdbErrorKind::FileIUError));
        }

        // TODO: access parent struct (kernel has a macro for this)

        self.path = v[1].to_string();

        // This is creating new Context and we are losing previous information.
        // This is becoming like a fork of new child process. This is not ideal,
        // and rather it should be cleaning up the current Context and returning
        // back (safe-guarding the path obviously).
        // init_debugger(&self.path)?;

        Ok(())
    }

    fn dump_help(&self) {
        println!(
            "{}
        File command will load any binary (currently expecting a 64-bit ELF
        build with DWARF 4/5 debugging symbols) dynamically.",
            wdbErrorKind::FileIUError
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_FileTy() {}
}
