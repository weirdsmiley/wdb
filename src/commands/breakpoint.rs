//! This submodule performs all handling for breakpoint command in the
//! available in the debugger.
use crate::debugger::Context;
use crate::utils::{wdbError, wdbErrorKind};
use std::error::Error;
use std::io::ErrorKind;

pub(crate) struct BreakPointTy {
    pub(crate) file: String,
    pub(crate) line: u32,
    // mod_info: ModuleInfo,
}

impl BreakPointTy {
    // TODO: Let us take the input str itself and use from_str to parse
    // it.
    pub(crate) fn new(l: u32) -> Result<Self, Box<dyn Error>> {
        Ok(BreakPointTy {
            file: "".to_string(),
            line: l,
        })
    }

    // Get the line number from the path. The path is of format
    // 'file:line'. It should not exit with error if colon is not found.
    pub(crate) fn parse(path: &str) -> Result<(String, u32), Box<dyn Error>> {
        match path.trim().split_once(':') {
            Some(iter) => {
                let (file, line) = iter;
                if let Err(parsed_line) = line.parse::<u32>() {
                    return Err(Box::new(wdbErrorKind::BreakPointParseIntError));
                }

                return Ok((file.to_string(), line.parse::<u32>().unwrap()));
            }
            None => {
                return Err(Box::new(wdbErrorKind::BreakPointParseError));
            }
        }
    }

    // Parse br and insert breakpoint to insert it to self.
    pub(crate) fn insert(&mut self, br: &str) -> Result<&mut Self, Box<dyn Error>> {
        match BreakPointTy::parse(br) {
            Ok(parsed) => {
                let (file, line) = parsed;
                self.file = file;
                self.line = line;
                return Ok(self);
            }
            Err(err) => {
                return Err(err);
            }
        }
    }

    pub(crate) fn dump(&self) -> String {
        self.line.to_string()
    }
}

impl std::str::FromStr for BreakPointTy {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // get file name and line number
        let (file, line) = BreakPointTy::parse(s).unwrap();

        Ok(BreakPointTy { file, line })
    }
}

impl crate::commands::CmdTy for BreakPointTy {
    type cmd = String;
    fn process(&mut self, c: Self::cmd) -> Result<(), Box<dyn Error>> {
        // Assign breakpoint (replace first byte of current instruction
        // with 0xcc.
        let v: Vec<&str> = c.split_whitespace().collect();

        if v.len() == 1 {
            return Err(Box::new(wdbErrorKind::BreakPointIUError));
        }

        let breakpoint = v[1];
        // This processing should happen inside CmdTy trait's process
        // method.
        match self.insert(breakpoint) {
            Ok(x) => {
                println!("breakpoint set at {}:{}", self.file, self.line);
            }
            Err(err) => {
                // TODO: Implement enum class for wdbErrorKind and match
                // against those values.
                // match error.as_ref() {
                // wdbError("".into()) => {

                // },
                // _ => {
                //     eprintln!("breakpoint format not supported");
                // }
                return Err(err);
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_BreakPointTy() {
        // test parse()
        let brk_t = BreakPointTy {
            file: "file".into(),
            line: 123,
        };
        // FIXME: Match with appropriate wdbErrorKind(s). Also, if
        // wdbErrorKind is static in nature then should we allocate those
        // errors on heap (using Box)? Should it not be better to have
        // static error return types in Results.
        match BreakPointTy::parse("file:123") {
            Ok(x) => {}
            Err(e) => {
                eprintln!("test_BreakPointTy failed");
            }
        }
        match BreakPointTy::parse("file:abc") {
            Ok(x) => {}
            Err(e) => {
                eprintln!("test_BreakPointTy failed");
            }
        }
    }
}
