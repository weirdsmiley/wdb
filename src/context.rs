use crate::commands;
use crate::commands::*;
use crate::error::wdbError;
use crate::error::wdbErrorKind;
use object::Object;
use std::{env, fs, process};

// This stores all other structs defined in parse.rs
// Should this be made into a DAG?
#[derive(Default)]
pub(crate) struct Context {
    // TODO: This should contain the current command as an AST of parsed tokens.
    // pub(crate) ModInfo: module::ModuleInfo<'a>,
    pub(crate) FCtx: file::FileTy,
    pub(crate) BrCtx: breakpoint::BreakPointTy,
    pub(crate) RCtx: run::RunTy,
}

impl Context {
    pub(crate) fn new(path: Option<String>) -> Result<Self, wdbError> {
        if path.is_some() {
            // Check binary for existence and arch support.
            if let Ok(bin) = fs::read(path.as_ref().unwrap().clone()) {
                if let Ok(obj) = object::File::parse(&*bin) {
                    if obj.architecture() != object::Architecture::X86_64 {
                        return Err(wdbError::from(wdbErrorKind::ArchitectureError));
                    }
                }
            }
        }

        // let fctx = file::FileTy::new(path);

        match file::FileTy::new(path) {
            Ok(f) => Ok(Context {
                // ModInfo: module::ModuleInfo::new(bin)?,
                FCtx: f,
                BrCtx: breakpoint::BreakPointTy::default(),
                RCtx: run::RunTy::default(),
            }),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn dump(&self) -> String {
        format!(
            "{{
  File: {}
  Breakpoints: {}
  Program counter: {}
}} ",
            // self.ModInfo.dump(),
            self.FCtx.dump(),
            self.BrCtx.dump(),
            self.RCtx.dump()
        )
    }

    pub(crate) fn usage(&self) {
        // self.BrCtx.dump_help();
        // self.RCtx.dump_help();
        // commands::CmdRunner::dump_help(&self.BrCtx);
        // commands::CmdRunner::dump_help(&self.RCtx);
        self.BrCtx.usage();
        self.RCtx.usage();
        self.FCtx.usage();
    }
}
