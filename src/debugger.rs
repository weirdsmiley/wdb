//! This module focuses solely on the debugger.
use crate::commands::*;
use crate::debugee;
use crate::parse;
use crate::utils::{dump, edump, wdbError};
// TODO: Parallelize continue_debugee
use object::Object;
use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;

// This stores all other structs defined in parse.rs
// Should this be made into a DAG?
pub(crate) struct Context {
    // FIXME: I have messed up the diff between using &str and String. Fix it!
    // TODO: This should contain the current command as an AST of parsed tokens.
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
            FCtx: file::FileTy::new(bin.to_string()).unwrap(),
            BrCtx: breakpoint::BreakPointTy::new(0).unwrap(),
            RCtx: run::RunTy::new(false, 0).unwrap(),
        })
    }

    pub(crate) fn dump(&self) -> String {
        format!(
            "{{
    Module : {}
    File: {}
    Breakpoints: {}
    Program counter: {}
}} ",
            self.ModInfo.dump(),
            self.FCtx.dump(),
            self.BrCtx.dump(),
            self.RCtx.dump()
        )
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
pub(crate) fn init_debugger(path: &String) -> Result<(), Box<dyn std::error::Error>> {
    // use .text section to get the instructions
    // if let Some(section) = obj.section_by_name(".text") {
    //     instprint!("{:#x?}", section.data()?);
    // } else {
    //     eprintln!("section not available");
    // }
    // println!("{:#x?}", bin);

    let bin = fs::read(&path)?;
    // Lets focus on ELF only for now.
    let obj = object::File::parse(&*bin)?;

    if obj.architecture() != object::Architecture::X86_64 {
        return Err(Box::new(wdbError("file format not supported".into())));
    }

    let mut Ctx: Context = Context::new("main.rs", "bin")?;
    let mut cmd = String::new();

    loop {
        parse::get_next_cmd(&mut cmd)?;

        if let Err(err) = parse::parse_cmd(&mut Ctx, &cmd) {
            edump!(err);
        };

        #[cfg(debug_assertions)]
        dump!(Ctx);
    }
}
