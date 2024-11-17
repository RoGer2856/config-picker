use std::collections::BTreeMap;

use directories::BaseDirs;
use variable_resolver::error::DecodeStringError;

pub struct VariableResolver {
    variables: BTreeMap<String, String>,
}

impl VariableResolver {
    pub fn new(base_dirs: BaseDirs) -> Self {
        Self {
            variables: BTreeMap::from([(
                "HOME".to_string(),
                base_dirs.home_dir().to_string_lossy().to_string(),
            )]),
        }
    }

    fn resolve_variable(&self, varname: impl AsRef<str>) -> Option<&str> {
        self.variables.get(varname.as_ref()).map(|s| s.as_str())
    }

    pub fn decode_string(&self, text: impl AsRef<str>) -> Result<String, DecodeStringError> {
        variable_resolver::decode_string(text, |name| self.resolve_variable(name))
    }
}
