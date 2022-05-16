//! This module gets the next input command inside the debugger and parses
//! it accordingly.
use crate::commands::*;
use crate::debugger::Context;
use crate::utils::*;
use std::error::Error;
use std::io::prelude::*;

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
// NOTE: The processing part should be handled by the CmdTy trait's process
// method.
fn which_cmd(cmd: &str) -> Cmd {
    let v: Vec<&str> = cmd.split_whitespace().collect();

    // FIXME: Improve regex matching
    match v[0] {
        "b" | "br" => return Cmd::BreakPoint,
        "r" => return Cmd::Run,
        "q" => return Cmd::Quit,
        _ => return Cmd::Unknown,
    }
}

// Take a mutable Context reference and return it after parsing and
// making changes in it.
pub(crate) fn parse_cmd<'a>(
    ctx: &'a mut Context,
    cmd: &String,
) -> Result<&'a mut Context, Box<dyn Error>> {
    // Bypassing Ctrl-d to prevent exiting
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
            // This processing shouuld happen inside CmdTy trait's process
            // method.
            match ctx.BrCtx.insert(breakpoint) {
                Ok(x) => {
                    println!("breakpoint set at {}:{}", ctx.BrCtx.file, ctx.BrCtx.line);
                }
                Err(_) => {
                    println!("breakpoint format not supported");
                }
            };
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
