//! This submodule performs all handling for breakpoint command in the
//! available in the debugger.
use crate::context::Context;
use crate::error::{wdbError, wdbErrorKind};
use crate::utils::{dump, edump};

// TODO: Provide better interface to setting breakpoints such as:
// 1. simply the function names (starting of that function.
// 2. setting at main
// 3. setting at end of a function
#[derive(Default)]
pub(crate) struct BreakPointTy {
    pub(crate) brlist: Vec<(String, u32)>,
    // pub(crate) file: String,
    // pub(crate) line: u32,
    // mod_info: ModuleInfo,
}

impl BreakPointTy {
    // TODO: Let us take the input str itself and use from_str to parse
    // it.
    pub(crate) fn new(f: String, l: u32) -> Result<Self, wdbError> {
        Ok(BreakPointTy {
            brlist: vec![(f, l)],
            // file: "".to_string(),
            // line: l,
        })
    }

    // Get the line number from the path. The path is of format
    // 'file:line'. It should not exit with error if colon is not found.
    pub(crate) fn parse(path: &str) -> Result<(String, u32), wdbError> {
        match path.trim().split_once(':') {
            Some(iter) => {
                let (file, line) = iter;
                if let Err(parsed_line) = line.parse::<u32>() {
                    return Err(wdbError::from(wdbErrorKind::BreakPointParseIntError));
                }

                Ok((file.to_string(), line.parse::<u32>().unwrap()))
            }
            None => Err(wdbError::from(wdbErrorKind::BreakPointParseError)),
        }
    }

    // Parse br and insert breakpoint to insert it to self.
    fn insert(&mut self, br: &str) -> Result<&mut Self, wdbError> {
        match BreakPointTy::parse(br) {
            Ok(parsed) => {
                let (file, line) = parsed;
                self.brlist.push((file, line));
                // self.file = file;
                // self.line = line;
                Ok(self)
            }
            Err(err) => Err(err),
        }
    }

    pub(crate) fn dump(&self) -> String {
        // self.line.to_string()
        let mut lists = String::from("\t");

        for pair in &self.brlist {
            let (f, l) = pair;
            let br_str = f.clone() + " ".into() + &*l.to_string() + "\n\t";
            lists = lists + br_str.as_str();
        }

        return lists;
    }
}

impl std::str::FromStr for BreakPointTy {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (file, line) = BreakPointTy::parse(s).unwrap();

        // Ok(BreakPointTy { file, line })
        Ok(BreakPointTy {
            brlist: vec![(file, line)],
        })
    }
}

impl crate::commands::CmdTy for BreakPointTy {
    type cmd = String;
    type ParentCtx = ();

    fn process(&mut self, cmd: Self::cmd) -> Result<Self::ParentCtx, wdbError> {
        // Assign breakpoint (replace first byte of current instruction
        // with 0xcc.
        // FIXME: Already pass a Vec<&str> of input command in all
        // process(es). Or better associate this vector inside ctx (not in
        // any of BrCtx, FCtx, etc. because they all will derive this from
        // ctx).
        let v: Vec<&str> = cmd.split_whitespace().collect();

        if v.len() != 2 {
            return Err(wdbError::from(wdbErrorKind::BreakPointIUError));
        }

        let breakpoint = v[1];
        // This processing should happen inside CmdTy trait's process
        // method.
        match self.insert(breakpoint) {
            Ok(x) => {
                let (file, line) = self.brlist.last().unwrap();
                println!("breakpoint set at {}:{}", file, line);
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

    fn dump_help(&self) {
        println!(
            "{}
        Breakpoint command will insert breakpoints(duh!) to specific addresses.
        The format for specifying an address is `file:line number`.

        The implementation is crudely based on replacing first byte of
        instruction with 0xcc which triggers the interrupt and the interrupt
        handler returns a SIGTRAP. Trapping this signal, we can stop the
        debugger and process more commands as we wish.",
            wdbErrorKind::BreakPointIUError
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        // test parse()
        let brk_t = BreakPointTy {
            brlist: vec![("file".into(), 123)], // file: "file".into(),
                                                // line: 123,
        };
        // FIXME: Match with appropriate wdbErrorKind(s). Also, if
        // wdbErrorKind is static in nature then should we allocate those
        // errors on heap (using Box)? Should it not be better to have
        // static error return types in Results.
        match BreakPointTy::parse("file:123") {
            Ok(x) => {}
            Err(e) => {
                eprintln!("test_parse failed");
            }
        }
        match BreakPointTy::parse("file:abc") {
            Ok(x) => {}
            Err(e) => {
                eprintln!("test_parse failed");
            }
        }
    }
}
