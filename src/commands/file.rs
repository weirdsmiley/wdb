use object::Object;
use std::{env, fs, process};

use crate::commands::breakpoint::BreakPointTy;
use crate::commands::run::RunTy;
use crate::context::{self, Context};
use crate::debugger::init_debugger;
use crate::error::{wdbError, wdbErrorKind};

// All structs of type *Ty are actual debugger commands(removing Ty at
// end). All of their members are placeholders for their possible
// options.
/// Load a new binary and re-run the debugger.
#[derive(Default)]
pub(crate) struct FileTy {
    pub(crate) path: Option<String>,
    // add a [u8] to store the binary in this format
    pub(crate) binary: Vec<u8>, // this will be filled only when path isn't None
}

// TODO: Move! to utils in form of a macro
macro_rules! binaryformat {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        res
    }}
}
fn split_in_4(content: String) -> String {
    let content = content
        .trim_start_matches('[')
        .trim_end_matches(']')
        .replace(",", "");
    let contents = content.split_whitespace();
    let mut new_content = String::from("");
    let mut idx = 0;
    let delimiter = " ";
    for each in contents {
        if idx % 16 == 0 {
            new_content += "\n";
        }
        new_content += &(each.to_owned() + delimiter);
        idx += 1;
    }
    new_content
}

impl FileTy {
    pub(crate) fn new(path: Option<String>) -> Result<Self, wdbError> {
        // if path.is_none() {
        //     // skip file reading
        //     // isn't is better to move file reading at process<T> and only
        //     // return a struct in new()
        // }
        // FIXME: Return wdbError
        // if let binary = std::fs::read(&path) {
        //     return Ok(FileTy { path, binary: binary.unwrap() });
        // }
        // return Err(wdbError::from(wdbErrorKind::FileIUError));
        // match std::fs::read(&path) {
        //     Ok(binary) => Ok(FileTy { path, binary }),
        //     Err(_) => Err(wdbError::from(wdbErrorKind::FileIUError)),
        // }

        if path.is_none() {
            Ok(FileTy {
                path,
                binary: vec![],
            })
        } else {
            match std::fs::read(path.as_ref().unwrap()) {
                Ok(binary) => Ok(FileTy { path, binary }),
                Err(_) => Err(wdbError::from(wdbErrorKind::FileIUError)),
            }
        }
    }

    pub(crate) fn dump(&self) -> String {
        if self.path.is_none() {
            return "no binary attached".into();
        }
        let binary = vec![
            0x66, 0x11, 0xf1, 0x3f, 0x66, 0x11, 0xf1, 0x3f, 0x66, 0x11, 0x01, 0x3f,
        ];
        let binary = &self.binary;
        let contents = split_in_4(binaryformat!("{:04X?}", /*self.binary*/ binary));
        self.path.as_ref().unwrap().to_string() /* + contents.as_str()*/
    }
}

impl crate::commands::CmdRunner for FileTy {
    type Arg<T> = ();
    type Return<T> = context::Context;
    fn process<T>(
        &mut self,
        cmd: &String,
        f: &mut Self::Arg<T>,
    ) -> Result<Self::Return<T>, wdbError> {
        let v: Vec<&str> = cmd.split_whitespace().collect();

        if v.len() != 2 {
            return Err(wdbError::from(wdbErrorKind::FileIUError));
        }

        if let Some(path) = v[1].get(..) {
            return Ok(Context::new(Some(path.into()))?);
        } else {
            return Err(wdbError::from(wdbErrorKind::RunIUError));
        }

        // This is creating new Context and we are losing previous information.
        // This is becoming like a fork of new child process. This is not ideal,
        // and rather it should be cleaning up the current Context and returning
        // back (safe-guarding the path obviously).
        // init_debugger(&self.path)?;
    }

    fn usage(&self) {
        println!(
            "{}
        File command will load any binary (currently expecting a 64-bit ELF
        build with DWARF 4/5 debugging symbols) dynamically.",
            wdbErrorKind::FileIUError
        );
    }
}

// // The only task is to load the new binary as member 'binary' and then can we
// // run main
// impl crate::commands::CmdTy for FileTy {
//     type cmd = String;
//     type ParentCtx = Context;

//     fn process(&mut self, cmd: Self::cmd) -> Result<Self::ParentCtx, wdbError> {
//         let v: Vec<&str> = cmd.split_whitespace().collect();

//         if v.len() != 2 {
//             return Err(wdbError::from(wdbErrorKind::FileIUError));
//         }

//         if let Some(path) = v[1].get(..) {
//             return Ok(Context::new(path.into())?);
//         } else {
//             return Err(wdbError::from(wdbErrorKind::RunIUError));
//         }

//         // This is creating new Context and we are losing previous information.
//         // This is becoming like a fork of new child process. This is not ideal,
//         // and rather it should be cleaning up the current Context and returning
//         // back (safe-guarding the path obviously).
//         // init_debugger(&self.path)?;
//     }

//     type FileArg<T> = ();
//     fn processNew<T>(&mut self, f: &mut Self::FileArg<T>) {
//         println!("Processing file command");
//     }

//     fn dump_help(&self) {
//         println!(
//             "{}
//         File command will load any binary (currently expecting a 64-bit ELF
//         build with DWARF 4/5 debugging symbols) dynamically.",
//             wdbErrorKind::FileIUError
//         );
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let file = String::from("a.out");
        let file_type = FileTy::new(Some(file));
        match file_type {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }
}
