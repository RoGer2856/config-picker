use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::error::{ReadConfigTypeDescriptorError, WriteConfigTypeDescriptorError};

#[derive(Serialize, Deserialize)]
pub struct ConfigTypeDescriptor {
    paths: Vec<String>,
}

impl ConfigTypeDescriptor {
    pub fn new() -> Self {
        Self { paths: Vec::new() }
    }

    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ReadConfigTypeDescriptorError> {
        let file = File::open(path).map_err(ReadConfigTypeDescriptorError::CouldNotOpenFile)?;
        Ok(serde_json::from_reader(file)?)
    }

    pub fn write_to_file(
        &self,
        path: impl AsRef<Path>,
    ) -> Result<(), WriteConfigTypeDescriptorError> {
        let serialized = serde_json::to_string(self)?;

        let mut file =
            File::create(path).map_err(WriteConfigTypeDescriptorError::CouldNotOpenFile)?;

        file.write(serialized.as_bytes())
            .map_err(WriteConfigTypeDescriptorError::CouldNotWriteDataToFile)?;

        Ok(())
    }

    pub fn paths(&self) -> impl Iterator<Item = PathBuf> + '_ {
        self.paths.iter().map(PathBuf::from)
    }
}
