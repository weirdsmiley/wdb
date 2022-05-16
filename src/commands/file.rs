// All structs of type *Ty are actual debugger commands(removing Ty at
// end). All of their members are placeholders for their possible
// options.
pub(crate) struct FileTy {
    binary: &'static str,
}

impl FileTy {
    pub(crate) fn new(bin: &'static str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(FileTy { binary: bin })
    }

    pub(crate) fn dump(&self) -> String {
        String::from("a")
    }
}
