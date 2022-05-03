//! This module gets the next input command inside the debugger and parses
//! it accordingly.
use std::error::Error;

trait CmdTy {
    fn process(self);
}

struct BreakPointTy {
    line: u32,
}

impl CmdTy for BreakPointTy {
    fn process(self) {
        // assign breakpoint (replace first byte of current instruction
        // with 0xcc
    }
}

struct RunTy {
    is_running: bool,
    pc: u32,
}

impl CmdTy for RunTy {
    fn process(self) {
        // start running the debugee until next interrupt is occurred
    }
}

// Create an enum CmdTy which contains many structs associated with the cmds
// like breakpoint, run, continue, step, etc.
// fn parse_cmd(cmd: &String) -> Result<CmdTy, Box<dyn Error>> {

// }

pub(crate) fn parse_cmd2(cmd: &String) -> Result<(), Box<dyn Error>> {
    // convert String to Vec and match first element
    let v: Vec<&str> = cmd.split_whitespace().collect();
    println!("{:?}", v);
    // lets match here only and return ??

    // Bypassing Ctrl-d
    if cmd.is_empty() {
        return Ok(());
    }

    match v[0] {
        "b" => {
            let line = v[1];
            println!("Breakpoint set at {}", line);
        }
        _ => {
            eprintln!("unknown command");
        }
    }
    Ok(())
}

pub(crate) fn get_next_cmd(input: &mut String) -> Result<&mut String, Box<dyn Error>> {
    let prev_input = input.clone();
    *input = String::new();

    let stdin = std::io::stdin();
    print!("(rdb): ");
    stdin.read_line(input)?;

    if input == "\n" {
        *input = prev_input.clone();
    }

    Ok(input)
}
