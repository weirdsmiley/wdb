//! This module is for the exection of the debuggee program.
use crate::error::wdbError;
use object::{Object, ObjectSection};
use std::error::Error;
use std::process::Command;
use std::{borrow, env, fs, thread};

// Get the modified binary file and run it.
// TODO: Inspiration from Valgrind's  development
// https://nnethercote.github.io/2022/07/27/twenty-years-of-valgrind.html
// Doing binary interpretation is slow. Then how does gdb/lldb work faster?
// Can cache be used in more efficient manner?
pub(crate) fn continue_debugee(cmd: String, args: String) -> Result<(), wdbError> {
    // FIXME: Run obj binary, but this is not a binary, it is an object
    // file
    // TODO: In order to run the debugee program, we can use
    // fexecve which is in nix crate.
    // Simply continue_debugee
    // and waitpid();
    // This should be when 'run' command is hit.
    // debugee::continue_debugee(bin)?;
    // let path = "test/bin";
    // let file = fs::File::open(path).unwrap();

    let handle = thread::spawn(move || {
        let output;

        if args.is_empty() {
            output = Command::new(cmd).output().expect("Failed to run {path}");
        } else {
            output = Command::new(cmd)
                .arg(args)
                .output()
                .expect("Failed to run {path} {args}");
        }

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{stdout}");
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("{stderr}");
        }
    });

    handle
        .join()
        .expect("Failed to join spawned thread for {path}");

    Ok(())

    // let mmap = unsafe { memmap::Mmap::map(&file).unwrap() };
    // let object = object::File::parse(&*mmap).unwrap();
    // let endian = if object.is_little_endian() {
    //     gimli::RunTimeEndian::Little
    // } else {
    //     gimli::RunTimeEndian::Big
    // };
    // dump_file(&object, endian)?;

    // Ok(())
}

// TODO: Remove this boilerplate!
fn dump_file(obj: &object::File, endian: gimli::RunTimeEndian) -> Result<(), gimli::Error> {
    let load_section = |id: gimli::SectionId| -> Result<borrow::Cow<[u8]>, gimli::Error> {
        match obj.section_by_name(id.name()) {
            Some(ref section) => Ok(section
                .uncompressed_data()
                .unwrap_or(borrow::Cow::Borrowed(&[][..]))),
            None => Ok(borrow::Cow::Borrowed(&[][..])),
        }
    };

    let dwarf_cow = gimli::Dwarf::load(&load_section)?;
    // Borrow a `Cow<[u8]>` to create an `EndianSlice`.
    let borrow_section: &dyn for<'a> Fn(
        &'a borrow::Cow<[u8]>,
    ) -> gimli::EndianSlice<'a, gimli::RunTimeEndian> =
        &|section| gimli::EndianSlice::new(section, endian);

    // Create `EndianSlice`s for all of the sections.
    let dwarf = dwarf_cow.borrow(&borrow_section);

    // Iterate over the compilation units.
    let mut iter = dwarf.units();
    while let Some(header) = iter.next()? {
        // println!(
        //     "Unit at <.debug_info+0x{:x}>",
        //     header.offset().as_debug_info_offset().unwrap().0
        // );
        let unit = dwarf.unit(header)?;

        // Iterate over the Debugging Information Entries (DIEs) in the unit.
        let mut entries = unit.entries();
        if let Some((delta_depth, entry)) = entries.next_dfs()? {
            println!("{:#010x}: {}", entry.offset().0, entry.tag());

            // Iterate over the attributes in the DIE.
            let mut attrs = entry.attrs();
            while let Some(attr) = attrs.next()? {
                // Print further
                // match attr.value() {

                // }
                println!("   {}: {:?}", attr.name(), attr.value());
            }
        }
    }

    Ok(())
}
