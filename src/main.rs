use object::Object;
use std::env;
use std::error::Error;
use std::fs;
use std::process;
use std::sync::{Arc, Mutex};

mod debugee;
mod debugger;
mod parse;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        eprintln!("file not found");
        process::exit(1);
    }
    // we need to move both of these
    let bin : Vec<u8> = fs::read(&args[1])?;
    let obj: Arc<Mutex<object::File<'static>>> = Arc::new(Mutex::from(object::File::parse(&*bin)?));

    if obj.lock().unwrap().architecture() == object::Architecture::X86_64 {
        debugger::init_debugger(bin, obj)?;
    } else {
        eprintln!("file format not supported on this architecture");
        process::exit(1);
    }

    Ok(())
}
