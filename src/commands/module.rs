use crate::error::wdbError;

pub(crate) struct ModuleInfo {
    source: &'static str, // source file name (infer from elf header)
    binary: &'static str, // binary path
}

impl ModuleInfo {
    pub(crate) fn new(src: &'static str, bin: &'static str) -> Result<Self, wdbError> {
        // TODO: Checks if binary doesn't exists or different architecture.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mod_info = ModuleInfo::new("main.c", "a.out").unwrap();
        assert!(mod_info.source == "main.c");
        assert!(mod_info.binary == "a.out");
    }
}
