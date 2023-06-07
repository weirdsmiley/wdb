//! This module gets the next input command inside the debugger and parses
//! it accordingly.
use crate::commands::*;
use crate::context::Context;
use crate::error::{wdbError, wdbErrorKind};
use crate::utils::*;
use std::error::Error;
use std::io::prelude::*;

// Returns the appropriate command (struct object?) which then can be
// used for do processing.
fn which_cmd(cmd: &str) -> Cmd {
    // FIXME: Don't split multiple times through execution. The best way is to
    // put an AST inside Context.
    let v: Vec<&str> = cmd.split_whitespace().collect();

    // TODO: Improve regex matching, lets go char by char and implement a parser
    // println!("debug:{}$", v[0]);
    match &v[0][0..1] {
        "b" => {
            if v[0].len() > 1 {
                match &v[0][0..2] {
                    "br" => Cmd::BreakPoint,
                    _ => Cmd::Unknown,
                };
            }
            Cmd::BreakPoint
        }
        "r" => {
            if v[0].len() > 1 {
                match &v[0][0..2] {
                    "ru" => Cmd::Run,
                    _ => Cmd::Unknown,
                };
            }
            Cmd::Run
        }
        "q" => Cmd::Quit,
        "h" => Cmd::Help,
        "f" => Cmd::File,
        // "list" => Cmd::List,
        _ => Cmd::Unknown,
    }
}

// Take a mutable Context reference and return it after parsing and
// making changes in it.
pub(crate) fn parse_cmd<'a>(
    ctx: &'a mut Context,
    cmd: &'a String,
) -> Result<Context, Box<dyn Error>>
where
    Context: Default,
{
    // Bypassing Ctrl-d to prevent exiting and empty inputs.
    if cmd.is_empty() || cmd == "\n" {
        println!();
        return Ok(std::mem::take(ctx));
    }

    // cmd is being passed around twice if only newline is hit
    match which_cmd(cmd) {
        // TODO: Move passing around cmd as argument, every match case will
        // identify important args here and only pass those important args.
        Cmd::File => {
            let ctx = ctx.FCtx.process(cmd.to_owned())?;
            return Ok(ctx);
        }
        Cmd::BreakPoint => {
            ctx.BrCtx.process(cmd.clone())?;
        }
        Cmd::Run => {
            ctx.RCtx.process(Some(ctx.FCtx.path.clone()))?;
        }
        Cmd::Quit => {
            std::process::exit(0);
        }
        Cmd::Help => {
            // FIXME: Why do we need help module if every command can dump_help
            // itself?
            ctx.dump_help();
        }
        // Cmd::List => {
        //     todo!();
        // }
        Cmd::Unknown => {
            eprintln!("unknown command");
        }
    }
    Ok(std::mem::take(ctx))
}

pub(crate) fn get_next_cmd() -> Result<String, wdbError> {
    let mut input = String::new();

    print!("@(wdb) ");
    if std::io::stdout().flush().is_err() {
        return Err(wdbError::from(wdbErrorKind::BreakPointParseError));
    }

    let stdin = std::io::stdin();
    if stdin.read_line(&mut input).is_err() {
        return Err(wdbError::from(wdbErrorKind::ParseError));
    }

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_which_cmd() {
        assert_eq!(which_cmd("b test.rs:345"), Cmd::BreakPoint);
        assert_eq!(which_cmd("br test.rs:345"), Cmd::BreakPoint);
        assert_eq!(which_cmd("r"), Cmd::Run);
        assert_eq!(which_cmd("q"), Cmd::Quit);
        assert_eq!(which_cmd("h"), Cmd::Help);
        assert_eq!(which_cmd("break"), Cmd::BreakPoint);
        assert_eq!(which_cmd("run"), Cmd::Run);
        assert_eq!(which_cmd("quit"), Cmd::Quit);
        assert_eq!(which_cmd("help"), Cmd::Help);
    }
}
