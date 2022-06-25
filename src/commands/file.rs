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
    pub(crate) binary: String,
}

impl FileTy {
    pub(crate) fn new(bin: String) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(FileTy { binary: bin })
    }

    pub(crate) fn dump(&self) -> String {
        self.binary.to_string()
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

        self.binary = v[1].to_string();

        // FIXME: This is from main.rs
        // This entire block should be inside init_debugger and also removed
        // from main().
        let bin = fs::read(self.binary.clone())?;
        // Lets focus on ELF only for now.
        let obj = object::File::parse(&*bin)?;

        if obj.architecture() == object::Architecture::X86_64 {
            init_debugger(&bin, &obj)?;
        } else {
            return Err(Box::new(wdbError("file format not supported".into())));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_FileTy() {}
}
