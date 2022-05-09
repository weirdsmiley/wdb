//! This module gets the next input command inside the debugger and parses
//! it accordingly.
use crate::debugger::Context;
use crate::utils::*;
use std::error::Error;
use std::fmt;
use std::io::prelude::*;
use std::str::FromStr;

// Custom error handling, coz why not!
#[derive(Debug)]
struct rdbError(String);

impl fmt::Display for rdbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(rdb): {}", self.0)
    }
}

impl Error for rdbError {}

// List of all supported commands in the debugger.
#[derive(Debug, PartialEq, PartialOrd)]
enum Cmd {
    BreakPoint,
    Run,
    Quit,
    // Help,
    // Print,
    Unknown,
}

// TODO: What was this for? If we are processing each command inside its
// own impl block then there is no use of a trait.
trait CmdTy {
    fn process(self, Ctx: &mut Context);
}

pub(crate) struct ModuleInfo {
    source: &'static str, // source file name
    binary: &'static str, // binary path
}

impl ModuleInfo {
    pub(crate) fn new(src: &'static str, bin: &'static str) -> Result<Self, Box<dyn Error>> {
        Ok(ModuleInfo {
            source: src,
            binary: bin,
        })
    }

    pub(crate) fn dump(&self) -> String {
        String::from("modinfo")
    }
}

// All structs of type *Ty are actual debugger commands(removing Ty at
// end). All of their members are placeholders for their possible
// options.
pub(crate) struct FileTy {
    binary: &'static str,
}

impl FileTy {
    pub(crate) fn new(bin: &'static str) -> Result<Self, Box<dyn Error>> {
        Ok(FileTy { binary: bin })
    }

    pub(crate) fn dump(&self) -> String {
        String::from("a")
    }
}

pub(crate) struct BreakPointTy {
    file: String,
    line: u32,
    // mod_info: ModuleInfo,
}

impl BreakPointTy {
    // TODO: Let us take the input str itself and use from_str to parse
    // it.
    pub(crate) fn new(l: u32) -> Result<Self, Box<dyn Error>> {
        Ok(BreakPointTy {
            file: "no idea".to_string(),
            line: l,
        })
    }

    // Get the line number from the path. The path is of format
    // 'file:line'. It should not exit with error if colon is not found.
    pub(crate) fn parse(path: &str) -> Result<(String, u32), Box<dyn Error>> {
        match path.trim().split_once(':') {
            Some(iter) => {
                let (file, line) = iter;
                return Ok((file.to_string(), line.parse::<u32>().unwrap()));
            }
            None => {
                return Err(Box::new(rdbError("breakpoint not parsed".into())));
            }
        }
    }

    // Parse br and insert breakpoint to insert it to self.
    pub(crate) fn insert(&mut self, br: &str) -> Result<&mut Self, Box<dyn Error>> {
        let (file, line) = BreakPointTy::parse(br).unwrap();

        self.file = file;
        self.line = line;

        Ok(self)
    }

    pub(crate) fn dump(&self) -> String {
        self.line.to_string()
    }
}

impl FromStr for BreakPointTy {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // get file name and line number
        let (file, line) = BreakPointTy::parse(s).unwrap();

        Ok(BreakPointTy { file, line })
    }
}

impl CmdTy for BreakPointTy {
    fn process(self, Ctx: &mut Context) {
        // assign breakpoint (replace first byte of current instruction
        // with 0xcc
    }
}

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

// Create an enum CmdTy which contains many structs associated with the cmds
// like breakpoint, run, continue, step, etc.
// fn parse_cmd(cmd: &String) -> Result<CmdTy, Box<dyn Error>> {

// }

// Returns the appropriate command (struct object?) which then can be
// used for do processing.
// Or can we return a tuple (enum, struct object) just match the fist
// element to find the type and object can be used later
// TODO: Can we do it with a Result<Box<trait>>? All we need to return is
// a built object of certain CmdTy and all its implemented methods.
// This can never take an empty/unknown command. So we can simply return
// Box over dynamically dispatched trait CmdTy.
// This is becoming more of a do_cmd (over Context) instead of showing only
// which.
// Want to make it generic as it returns?
fn which_cmd(cmd: &str) -> Cmd {
    let v: Vec<&str> = cmd.split_whitespace().collect();

    match v[0] {
        "b" | "br" => return Cmd::BreakPoint,
        "r" => return Cmd::Run,
        "q" => return Cmd::Quit,
        _ => return Cmd::Unknown,
    }
}

// Take a mutable Context reference and return it after parsing and
// making changes in it.
pub(crate) fn parse_cmd2<'a>(
    ctx: &'a mut Context,
    cmd: &String,
) -> Result<&'a mut Context, Box<dyn Error>> {
    // Bypassing Ctrl-d
    if cmd.is_empty() {
        return Ok(ctx);
    }

    match which_cmd(cmd) {
        // TODO: which_cmd should return the already built object and
        // here we can pattern match to find if it is of the appropriate
        // struct or not.
        Cmd::BreakPoint => {
            let v: Vec<&str> = cmd.split_whitespace().collect();
            let breakpoint = v[1];
            ctx.BrCtx.insert(breakpoint)?;
            println!("Breakpoint set at {}:{}", ctx.BrCtx.file, ctx.BrCtx.line);
            dump!(ctx.BrCtx);
            // println!("{}", ctx.BrCtx.dump());
        }
        Cmd::Run => {
            println!("Running...");
            ctx.RCtx.run()?;
        }
        Cmd::Quit => {
            std::process::exit(0);
        }
        Cmd::Unknown => {
            eprintln!("unknown command");
        }
    }
    Ok(ctx)
}

pub(crate) fn get_next_cmd(input: &mut String) -> Result<&mut String, Box<dyn Error>> {
    let prev_input = input.clone();
    *input = String::new();

    print!("(rdb): ");
    std::io::stdout().flush()?;

    let stdin = std::io::stdin();
    stdin.read_line(input)?;

    if input == "\n" {
        *input = prev_input.clone();
    }

    Ok(input)
}
