use crate::debugee;
use crate::debugger::Context;
use crate::utils::{wdbError, wdbErrorKind};
use std::error::Error;

pub(crate) struct RunTy {
    is_running: bool,
    pc: u32,
}

impl RunTy {
    pub(crate) fn new(run: bool, pc: u32) -> Result<Self, Box<dyn Error>> {
        Ok(RunTy {
            is_running: run,
            pc,
        })
    }

    pub(crate) fn run(&mut self) -> Result<&mut Self, Box<dyn Error>> {
        self.pc = u32::MAX;
        if (self.pc).checked_add(1) == None {
            return Err(Box::new(wdbErrorKind::RunPCOverflowError));
        }
        self.is_running = true;
        self.pc += 1;
        Ok(self)
    }
}

impl crate::commands::CmdTy for RunTy {
    type cmd = Option<String>;
    fn process(&mut self, cmd: Self::cmd) -> Result<(), Box<dyn Error>> {
        // start running the debugee until next interrupt is occurred
        println!("Running...");
        // FIXME: Fix the binary context.
        let bin = vec![];
        debugee::continue_debugee(&bin)?;

        // FIXME: Return type should be derived from continue_debugee.
        match self.run() {
            Ok(r) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_RunTy() {
        let mut not_running = RunTy {
            is_running: false,
            pc: 0,
        };
        let first_run = RunTy {
            is_running: true,
            pc: 1,
        };
        match not_running.run() {
            Ok(x) => {
                assert!(not_running.is_running);
                assert!(not_running.pc == first_run.pc);
            }
            Err(_) => {
                eprintln!("test_RunTy failed");
            }
        }
    }
}
