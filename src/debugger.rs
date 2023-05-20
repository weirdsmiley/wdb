//! This module focuses solely on the debugger.
use crate::context::Context;
use crate::debugee;
use crate::error::wdbError;
use crate::error::wdbErrorKind;
use crate::parse;
use crate::utils::{dump, edump};
// TODO: Parallelize continue_debugee
use object::Object;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

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
pub(crate) fn init_debugger(args: Vec<String>) -> Result<(), wdbError> {
    if args.len() < 1 {
        return Err(wdbError::from(wdbErrorKind::NonExistentBinary));
    }

    // use .text section to get the instructions
    // if let Some(section) = obj.section_by_name(".text") {
    //     instprint!("{:#x?}", section.data()?);
    // } else {
    //     eprintln!("section not available");
    // }
    // println!("{:#x?}", bin);

    let mut Ctx = Context::new(args[1].to_owned()).unwrap();

    loop {
        let cmd = parse::get_next_cmd()?;

        match parse::parse_cmd(&mut Ctx, &cmd) {
            Ok(ctx) => Ctx = ctx,
            Err(err) => edump!(err),
        };

        // // Why we need this?
        // if let Ok(binary) = fs::read(&Ctx.FCtx.path) {
        //     let objfh = object::File::parse(&*binary).unwrap();
        // }

        #[cfg(debug_assertions)]
        dump!(&mut Ctx);
    }
}
