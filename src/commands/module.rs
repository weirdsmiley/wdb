use crate::error::wdbError;

pub(crate) struct ModuleInfo<'a> {
    binary: &'a String, // binary path
}

impl<'a> ModuleInfo<'a> {
    pub(crate) fn new(bin: &'a String) -> Result<Self, wdbError> {
        // TODO: Checks if binary doesn't exists or different architecture.
        Ok(ModuleInfo { binary: bin })
    }

    pub(crate) fn dump(&self) -> String {
        format!(
            "{{
  binary: {}
}}",
            self.binary
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let module = String::from("a.out");
        let mod_info = ModuleInfo::new(&module).unwrap();
        assert!(mod_info.binary == "a.out");
    }
}
