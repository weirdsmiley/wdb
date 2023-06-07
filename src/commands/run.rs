use crate::context::Context;
use crate::debugee;
use crate::error::{wdbError, wdbErrorKind};

#[derive(Default)]
pub(crate) struct RunTy {
    is_running: bool,
    pc: u32,
}

impl RunTy {
    pub(crate) fn new(run: bool, pc: u32) -> Result<Self, wdbError> {
        Ok(RunTy {
            is_running: run,
            pc,
        })
    }

    pub(crate) fn run(&mut self) -> Result<&mut Self, wdbError> {
        if (self.pc).checked_add(1).is_none() {
            return Err(wdbError::from(wdbErrorKind::RunPCOverflowError));
        }

        self.is_running = true;
        self.pc += 1;
        Ok(self)
    }

    pub(crate) fn dump(&self) -> String {
        self.pc.to_string()
    }
}

impl crate::commands::CmdTy for RunTy {
    type cmd = Option<String>;
    type ParentCtx = ();

    fn process(&mut self, cmd: Self::cmd) -> Result<Self::ParentCtx, wdbError> {
        // start running the debugee until next interrupt is occurred
        println!("Running...");

        if let Some(path) = cmd {
            debugee::continue_debugee(path)?;
        }

        // FIXME: Return type should be derived from continue_debugee.
        // FIXME: What is it returning?
        match self.run() {
            Ok(r) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn dump_help(&self) {
        println!(
            "{}
        Run command will continue the debugee program. It also increments the
        program counter (for reasons, I have no idea about)",
            wdbErrorKind::RunIUError
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
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
                eprintln!("test_run failed");
            }
        }
    }
}
