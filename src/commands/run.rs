use crate::commands::CmdTy;
use crate::debugger::Context;
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
        self.is_running = true;
        self.pc += 1;
        Ok(self)
    }
}

impl CmdTy for RunTy {
    fn process(self, Ctx: &mut Context) {
        // start running the debugee until next interrupt is occurred
    }
}
