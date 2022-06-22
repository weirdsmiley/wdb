//! This module gets the next input command inside the debugger and parses
//! it accordingly.
use crate::commands::*;
use crate::debugger::Context;
use crate::utils::*;
use std::error::Error;
use std::io::prelude::*;

// Returns the appropriate command (struct object?) which then can be
// used for do processing.
fn which_cmd(cmd: &str) -> Cmd {
    let v: Vec<&str> = cmd.split_whitespace().collect();

    // TODO: Improve regex matching
    // println!("debug:{}$", v[0]);
    match &v[0][0..1] {
        "b" => {
            if v[0].len() > 1 {
                match &v[0][0..2] {
                    "br" => Cmd::BreakPoint,
                    _ => Cmd::Unknown,
                };
            }
            return Cmd::BreakPoint;
        }
        "r" => {
            if v[0].len() > 1 {
                match &v[0][0..2] {
                    "ru" => Cmd::Run,
                    _ => Cmd::Unknown,
                };
            }
            return Cmd::Run;
        }
        "q" => return Cmd::Quit,
        "h" => return Cmd::Help,
        _ => return Cmd::Unknown,
    }
}

// Take a mutable Context reference and return it after parsing and
// making changes in it.
pub(crate) fn parse_cmd<'a>(
    ctx: &'a mut Context,
    cmd: &String,
) -> Result<&'a mut Context, Box<dyn Error>> {
    // Bypassing Ctrl-d to prevent exiting and empty inputs.
    if cmd.is_empty() || cmd == "\n" {
        return Ok(ctx);
    }

    // cmd is being passed around twice if only newline is hit
    match which_cmd(&cmd) {
        Cmd::BreakPoint => {
            // TODO: Can this be passed as reference? Is GAT coming in
            // picture?
            ctx.BrCtx.process(cmd.to_string())?;
        }
        Cmd::Run => {
            ctx.RCtx.process(None)?;
        }
        Cmd::Quit => {
            std::process::exit(0);
        }
        Cmd::Help => {
            // use dump!()
            println!("There is no help! Bye :')");
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

    print!("(wdb): ");
    std::io::stdout().flush()?;

    let stdin = std::io::stdin();
    stdin.read_line(input)?;

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
