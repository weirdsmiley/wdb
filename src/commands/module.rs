pub(crate) struct ModuleInfo {
    source: &'static str, // source file name (infer from elf header)
    binary: &'static str, // binary path
}

impl ModuleInfo {
    pub(crate) fn new(
        src: &'static str,
        bin: &'static str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(ModuleInfo {
            source: src,
            binary: bin,
        })
    }

    pub(crate) fn dump(&self) -> String {
        format!(
            "{{
        source: {}
        binary: {}
    }}",
            self.source, self.binary
        )
    }
}
