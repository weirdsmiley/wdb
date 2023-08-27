use crate::context::Context;
use crate::debugee;
use crate::error::{wdbError, wdbErrorKind};

use super::breakpoint::BreakPointTy;
use super::file::FileTy;

#[derive(Default)]
pub(crate) struct RunTy {
    running: bool,
    pc: u32,
}

impl RunTy {
    pub(crate) fn new(run: bool, pc: u32) -> Result<Self, wdbError> {
        Ok(RunTy { running: run, pc })
    }

    pub(crate) fn run(&mut self) -> Result<&mut Self, wdbError> {
        if (self.pc).checked_add(1).is_none() {
            return Err(wdbError::from(wdbErrorKind::RunPCOverflowError));
        }

        self.running = true;
        self.pc += 1;
        Ok(self)
    }

    pub(crate) fn dump(&self) -> String {
        self.pc.to_string()
    }
}

impl crate::commands::CmdRunner for RunTy {
    type Arg<T> = FileTy;
    type Return<T> = ();
    fn process<T>(&mut self, cmd: &String, f: &mut Self::Arg<T>) -> Result<(), wdbError> {
        // start running the debugee until next interrupt is occurred
        println!("Running...");

        let args = cmd.clone();
        debugee::continue_debugee(f, args)?;

        // FIXME: Return type should be derived from continue_debugee.
        // FIXME: What is it returning?
        match self.run() {
            Ok(r) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn usage(&self) {
        println!(
            "{}
        Run command will continue the debugee program. It also increments the
        program counter (for reasons, I have no idea about)",
            wdbErrorKind::RunIUError
        );
    }
}

// impl crate::commands::CmdTy for RunTy {
//     // (binary, args to binary)
//     type cmd = (String, String);
//     type ParentCtx = ();

//     fn process(&mut self, cmd: Self::cmd) -> Result<Self::ParentCtx, wdbError> {
//         // start running the debugee until next interrupt is occurred
//         println!("Running...");

//         let (path, args) = cmd;
//         // debugee::continue_debugee(path, args)?;

//         // FIXME: Return type should be derived from continue_debugee.
//         // FIXME: What is it returning?
//         match self.run() {
//             Ok(r) => Ok(()),
//             Err(e) => Err(e),
//         }
//     }

//     type FileArg<T> = BreakPointTy;
//     fn processNew<T>(&mut self, f: &mut Self::FileArg<T>) {
//         println!("Processing run command");
//     }

//     fn dump_help(&self) {
//         println!(
//             "{}
//         Run command will continue the debugee program. It also increments the
//         program counter (for reasons, I have no idea about)",
//             wdbErrorKind::RunIUError
//         );
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let mut not_running = RunTy {
            running: false,
            pc: 0,
        };
        let first_run = RunTy {
            running: true,
            pc: 1,
        };
        match not_running.run() {
            Ok(x) => {
                assert!(not_running.running);
                assert!(not_running.pc == first_run.pc);
            }
            Err(_) => {
                eprintln!("test_run failed");
            }
        }
    }
}
