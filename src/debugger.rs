//! This module focuses solely on the debugger.
use crate::debugee;
use crate::parse;
// TODO: Parallelize continue_debugee
use crate::commands::*;
use std::sync::{Arc, Mutex};
use std::thread;

// This stores all other structs defined in parse.rs
// Should this be made into a DAG?
pub(crate) struct Context {
    pub(crate) ModInfo: module::ModuleInfo,
    pub(crate) FCtx: file::FileTy,
    pub(crate) BrCtx: breakpoint::BreakPointTy,
    pub(crate) RCtx: run::RunTy,
}

impl Context {
    // Should this return Error ype otherwise too?
    // Why should it contain dyn?
    pub(crate) fn new(
        src: &'static str,
        bin: &'static str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Context {
            ModInfo: module::ModuleInfo::new(src, bin)?,
            FCtx: file::FileTy::new("").unwrap(),
            BrCtx: breakpoint::BreakPointTy::new(0).unwrap(),
            RCtx: run::RunTy::new(false, 0).unwrap(),
        })
    }
}

// Start the debugger
// 1. First inside a loop {} ask for user input
// 2. If the user input is a breakpoint
// 3. Replace first byte of PC instruction, with 0xcc
// 4. Continue running the debugee, and waitpid for any interrupts
// 5. When an interrupt is raised, we must have hit the breakpoint
// 6. Dump that instruction. (for now)
//
// The main part of debugger was the following:
//      insert breakpoint
//      continue_debugee() // this should be moved to a new thread
//      waitpid() // but we are still waiting for the debugee to stop
//                // essentially a sequential program
//      SIGTRAP returned, breakpoint hit, dump source line
pub(crate) fn init_debugger(
    bin: &Vec<u8>,
    obj: &object::File,
) -> Result<(), Box<dyn std::error::Error>> {
    // use .text section to get the instructions
    // if let Some(section) = obj.section_by_name(".text") {
    //     instprint!("{:#x?}", section.data()?);
    // } else {
    //     eprintln!("section not available");
    // }
    // println!("{:#x?}", bin);

    // FIXME: Find the source filename from the ELF header.
    let mut Ctx: Context = Context::new("main.rs", "bin")?;

    let mut cmd = String::new();

    loop {
        parse::get_next_cmd(&mut cmd)?;
        parse::parse_cmd(&mut Ctx, &cmd)?;

        // This has to be the modified binary (that is binary after
        // inserting 0xcc at appropriate place).
        // TODO: Move this in new thread. and to wait for this particular
        // thread, we can join. But that's is so inefficient, as it
        // becomes a sequential program only. Maybe we have to take a look
        // at this procedure in a different way to make it parallel.

        // FIXME: Fix obj dependency over bin while borrow happens.

        // let debugee_thread = thread::spawn(move || {
        //     debugee::continue_debugee(&obj);
        // });

        // debugee_thread
        //     .join()
        //     .expect("unable to join debugee thread");

        // TODO: In order to run the debugee program, we can use
        // fexecve which is in nix crate.
        // Simply continue_debugee
        // and waitpid();
        debugee::continue_debugee(bin)?;
    }
}
